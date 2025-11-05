# Deep Zoom Mandelbrot Research (2016-2025)

**Research Date:** November 2025
**Context:** This document summarizes major algorithmic breakthroughs in Mandelbrot deep zoom rendering since this codebase was last actively developed (July 2016).

---

## Executive Summary

The field of Mandelbrot deep zoom rendering has undergone a **revolution** since 2016. Three major developments enable 100-10,000x speedups:

1. **Perturbation Theory** (popularized 2013, matured post-2016) - Use one high-precision reference orbit with low-precision deltas per pixel
2. **BLA - Bilinear Approximation** (2021-2022) - Skip iterations when orbit is approximately linear (10x faster than older methods)
3. **Automatic Glitch Correction** (2014-2021) - Detect and fix artifacts that plagued early implementations

These techniques have enabled zoom depths of **10^-4141** (compared to ~10^-100 in 2016) with interactive rendering speeds.

---

## Timeline of Major Developments

### 2013: K.I. Martin - Perturbation Theory
- **Paper:** SuperFractalThing (sft_maths.pdf)
- **Breakthrough:** Compute one reference orbit at high precision, calculate pixel deltas at low precision
- **Impact:** Images that took 6 months now render in 6 hours
- **Problem:** Introduced "glitch" artifacts with no automatic detection

### 2014: Pauldelbrot - Glitch Detection Criterion
- **Platform:** FractalForums.org post
- **Breakthrough:** Mathematical criterion to detect when perturbation fails: `|Z_n + Δz_n| < τ·|Z_n|`
- **Derivation:** Perturbed the perturbation equations to analyze precision loss
- **Impact:** Enabled automatic glitch detection and re-rendering with new reference points
- **Note:** Heuristic without rigorous proof, but extremely effective in practice

### 2021: Zhuoran - Rebasing Method
- **Platform:** FractalForums.org pseudocode post
- **Breakthrough:** Prevent glitches instead of detecting them
- **Method:** When Δz drifts too far, reset to a new reference orbit and continue
- **Impact:** Simpler and more reliable than detection-based approaches
- **Current Status:** Considered superior to Pauldelbrot's detection

### 2021-2022: Bilinear Approximation (BLA)
- **Developers:** Zhuoran and community on FractalForums.org
- **Breakthrough:** Linear approximation method that replaces series approximation
- **Impact:** 10x faster than NanoMB series approximation (2 min vs 20 min at 5e-433 zoom)
- **Advantages:** Simpler, easier to parallelize, better stopping conditions, more general

---

## Technical Deep Dive

### 1. Perturbation Theory

#### The Core Concept
Instead of computing every pixel at high precision:
```
Traditional: For each pixel, compute z_n+1 = z_n² + c using MPFR (slow!)
Perturbation: Compute one reference Z_n = Z_n² + C at high precision
              Then for each pixel: Δz_n+1 = 2·Z_n·Δz_n + Δz_n² + Δc (fast!)
```

#### The Mathematics
Given reference orbit `Z_n` and pixel orbit `z_n = Z_n + Δz_n`:

```
z_n+1 = z_n² + c
(Z_n + Δz_n)² + (C + Δc) = Z_n² + 2·Z_n·Δz_n + Δz_n² + C + Δc
```

Since we know `Z_n+1 = Z_n² + C`, we get:
```
z_n+1 = Z_n+1 + 2·Z_n·Δz_n + Δz_n² + Δc
```

Therefore:
```
Δz_n+1 = 2·Z_n·Δz_n + Δz_n² + Δc
```

**Key insight:** This can be computed in standard double precision (float64) as long as Δz stays small!

#### Complexity Analysis
- **Traditional high-precision:** O(K × N × W × H) where K is precision bits
- **Perturbation:** O(K × N) + O((N-M) × W × H) where M iterations are skipped via approximation

For deep zooms: ~1000x speedup from perturbation alone.

#### Our Code Status
The `delta()` function at `src/mandelbrot.rs:189` shows we were thinking about this:
```rust
pub fn delta(d: Complex64, x_n: Complex64, input: [Complex64; 4]) -> (Complex64, [Complex64; 3]) {
    let a_n = input[0];  // These are series approximation coefficients
    let b_n = input[1];
    let c_n = input[2];
    // ...
}
```

But `calculate_all_delta()` at line 216 doesn't actually use it - it just calls regular iteration!

---

### 2. Glitch Detection and Prevention

#### The Problem: Precision Loss
When `Z_n ≈ -Δz_n`, floating-point catastrophic cancellation occurs:
```
Z_n = 1.234567890123456
Δz_n = -1.234567890123450
Z_n + Δz_n = 0.000000000000006  ← Most significant bits canceled!
```

Nearby pixels become indistinguishable → uniform color blobs (glitches).

#### Pauldelbrot's Criterion (2014)

**Formula:**
```
|Z_n + Δz_n| < τ · |Z_n|
```

Where `τ` is typically between `1e-8` (strict) to `1e-2` (lenient).

**Derivation:** He perturbed the perturbation equations:
```
Let z = Z + Δz + e  (where e is rounding error)
Let c = C + Δc + f

When e/Δz is NOT small, precision is insufficient
This happens when |Z + Δz| << |Z|
```

**Implementation:**
```rust
// Check after each iteration
if (Z + delta_z).norm() < threshold * Z.norm() {
    // Mark pixel as glitched
    // Queue for re-rendering with different reference
}
```

**Cost:** Negligible - `|Z + Δz|²` is already computed for escape test!

#### Zhuoran's Rebasing Method (2021)

**Concept:** Don't detect glitches - prevent them!

**Algorithm:**
```
1. Compute reference orbit from point R
2. For each pixel, compute Δz relative to R
3. When |Δz| exceeds threshold:
   - Start a new reference orbit from current pixel position
   - Reset Δz and continue
4. Only need as many references as critical points (typically 1 for Mandelbrot)
```

**Why it works:** Multiplication in complex plane causes rotation. Drifting orbits naturally realign when reset.

**Current status:** Considered superior to detection-based approaches.

---

### 3. Series Approximation vs BLA

#### Series Approximation (2013-2021)

**Concept:** Use Taylor series to skip iterations:
```
Δz_n = A_n·δ + B_n·δ² + C_n·δ³ + O(δ⁴)
```

**Coefficient computation:**
```
A_n+1 = 2·Z_n·A_n + 1
B_n+1 = 2·Z_n·B_n + A_n²
C_n+1 = 2·Z_n·C_n + 2·A_n·B_n
```

**Usage:** Compute coefficients once for reference orbit, then evaluate for each pixel with their specific `δ = Δc`.

**Variants:**
- Basic SA: 3rd order polynomial
- NanoMB1/NanoMB2: Higher-order optimizations
- Knighty's extension: Biseries in (z,c) for periodic points

**Problems:**
- Complex to implement correctly
- Difficult to parallelize (coefficient dependencies)
- Unclear stopping conditions (no rigorous error bounds)
- Limited to specific formulas

#### Bilinear Approximation - BLA (2021-2022)

**Concept:** Linearize when quadratic term is negligible:
```
Full perturbation: Δz_n+1 = 2·Z_n·Δz_n + Δz_n² + Δc
Linear approximation: Δz_n+1 ≈ 2·Z_n·Δz_n + Δc  (when |Δz_n²| << rest)
```

**Single-step BLA:**
```
Δz_n+1 = A·Δz_n + B
where:
  A = 2·Z_n
  B = Δc
```

**Multi-step BLA (the key innovation):**
Combine BLAs hierarchically:
```
Level 0: M entries (1 iteration each)
Level 1: M/2 entries (2 iterations each) - combine pairs from Level 0
Level 2: M/4 entries (4 iterations each) - combine pairs from Level 1
...
Level k: M/2^k entries (2^k iterations each)
```

**Combining formula:**
```
If BLA₁ skips k iterations and BLA₂ skips k more:
  Combined BLA: Δz_n+2k = A₂·(A₁·Δz_n + B₁) + B₂
                        = (A₂·A₁)·Δz_n + (A₂·B₁ + B₂)
```

**Validity check:**
```
|Δz_n| < r = max(0, ε·|Z_n|/|J_f(Z_n)| - 1)
```
Where:
- ε = machine precision threshold (e.g., 1e-6)
- J_f(Z_n) = Jacobian (derivative) of f at Z_n

**Why BLA is superior:**
- ✅ Conceptually simpler (just linear algebra)
- ✅ Easy to parallelize (independent computations)
- ✅ Clear validity conditions (radius check)
- ✅ Works for any formula (Burning Ship, hybrids, etc.)
- ✅ 10x faster than NanoMB (2 min vs 20 min at 5e-433)

---

### 4. When BLA is Applied in the Iteration Loop

This is the critical question for implementation!

#### The Iteration Loop Structure

```rust
// Precomputation phase (once per frame):
fn compute_bla_table(reference_orbit: &[Complex], max_iter: usize) -> BLATable {
    let mut table = BLATable::new();

    // Level 0: Single-step BLAs
    for n in 0..max_iter {
        let A = 2.0 * reference_orbit[n];
        let B = Complex::zero(); // Will be filled per-pixel with Δc
        table.add_level0(n, A, B);
    }

    // Level 1+: Hierarchical merging
    for level in 1.. {
        for i in 0..(max_iter / (1 << level)) {
            let bla1 = table.get(level - 1, 2*i);
            let bla2 = table.get(level - 1, 2*i + 1);
            let combined = combine_blas(bla1, bla2);
            table.add(level, i, combined);
        }
    }

    table
}

// Per-pixel iteration:
fn iterate_pixel_with_bla(
    delta_c: Complex,
    reference_orbit: &[Complex],
    bla_table: &BLATable,
    max_iter: usize,
    epsilon: f64
) -> u32 {
    let mut delta_z = Complex::zero();
    let mut n = 0;

    while n < max_iter {
        let Z_n = reference_orbit[n];

        // 1. CHECK IF LOWEST-LEVEL BLA IS VALID
        let radius = compute_radius(Z_n, epsilon);
        if delta_z.norm() >= radius {
            // BLA not valid - do regular perturbation iteration
            delta_z = 2.0 * Z_n * delta_z + delta_z * delta_z + delta_c;
            n += 1;

            // Check escape
            if (Z_n + delta_z).norm_sqr() > 4.0 {
                return n;
            }
            continue;
        }

        // 2. FIND LARGEST VALID BLA
        // Start from highest level and work down
        let mut best_level = 0;
        let mut best_skip = 1;

        for level in (0..bla_table.max_level()).rev() {
            let skip = 1 << level;
            if n + skip > max_iter {
                continue;
            }

            let bla = bla_table.get(level, n / skip);
            let bla_radius = compute_bla_radius(bla, Z_n, epsilon);

            if delta_z.norm() < bla_radius {
                best_level = level;
                best_skip = skip;
                break; // Found largest valid BLA
            }
        }

        // 3. APPLY THE BLA
        let bla = bla_table.get(best_level, n / best_skip);
        delta_z = bla.A * delta_z + bla.B * delta_c; // Linear approximation!
        n += best_skip;

        // Check escape
        let Z_target = reference_orbit[n.min(max_iter - 1)];
        if (Z_target + delta_z).norm_sqr() > 4.0 {
            return n;
        }
    }

    max_iter // Didn't escape
}
```

#### Key Points About When BLA Applies

1. **Every iteration opportunity:** BLA is checked at every iteration (or at strategic intervals)

2. **Validity gating:** If the lowest-level BLA isn't valid, NO BLA at that iteration is valid (immediate fallback to perturbation)

3. **Hierarchical selection:** When BLA is valid, binary search (or descending search) finds the largest skip

4. **Regions where BLA works:**
   - Near the interior (orbits stabilizing)
   - In smooth gradient areas
   - Far from the boundary

5. **Regions requiring perturbation:**
   - Near the fractal boundary (chaotic)
   - Where Δz is large
   - Initial iterations (delta hasn't settled)

**Metaphor:** BLA is like cruise control on a highway. You can only use it when the road is smooth and straight (linear region). On curvy mountain roads (chaotic regions), you must manually steer (perturbation).

---

## GPU Implementation Considerations

### What Works Well on GPU

1. **Perturbation delta calculations** - massively parallel, one thread per pixel
2. **BLA lookups** - read-only table, perfect for GPU cache
3. **Low-precision arithmetic** - GPUs excel at float32/float64

### The CPU/GPU Split

**Modern architecture:**
```
CPU (high precision):
  - Compute reference orbit using MPFR/GMP
  - Compute BLA coefficient table
  - Detect glitches / manage rebasing

GPU (low precision):
  - Parallel delta iterations for all pixels
  - BLA application
  - Escape time computation
  - Coloring
```

### Existing GPU Implementations

1. **Kalles Fraktaler** - OpenCL for perturbation iterations (double precision)
2. **FractalShark** - CUDA implementation, specifically designed for Nvidia GPUs
3. **DeepDrill** - Perturbation + series approximation
4. **WebGL implementations** - Browser-based deep zooming

### Challenges for GPU

1. **Divergent execution:** Different pixels need different iteration counts
2. **Glitch detection:** Requires synchronization and potential re-renders
3. **Dynamic precision:** GPUs limited to float64 at best
4. **Memory bandwidth:** BLA tables can be large

### Automatic Differentiation on GPU

Recent research shows you CAN compute higher-order derivatives on GPU:
- JAX (Python): Automatic differentiation + XLA + GPU
- CUDA kernels: Adjoint algorithmic differentiation (AAD)
- Dual numbers: For distance estimation

Not widely adopted for BLA yet, but possible for series approximation coefficients.

---

## Current State of the Art (2025)

### Software Landscape

**Kalles Fraktaler 2+**
- Gold standard for desktop
- CPU-based with OpenCL support
- Uses NanoMB (not yet BLA)
- Reached zooms of 10^-433+

**FractalShark**
- Nvidia GPU exclusive
- CUDA-based linear approximation (similar to BLA)
- High-performance for consumer hardware

**DeepDrill**
- Open-source
- Perturbation + series approximation
- Good documentation

**Browser-based tools**
- WebGL2 implementations
- Real-time exploration to moderate depths
- Educational value

### Record Zooms

- **2016:** ~10^-100 was impressive
- **2020:** 10^-275 (YouTube: "New Record!")
- **2025:** 10^-4141 achieved (Reddit discussion)

### Performance Metrics

With perturbation + BLA on modern hardware:
- **Location at 5e-433:** 2 minutes (vs 20+ minutes with SA)
- **Interactive zooming:** Possible to 10^-38 at 60fps on desktop
- **Calculation vs rendering:** Rendering (coloring) now often slower than calculation!

---

## Implementation Recommendations for This Codebase

### Phase 1: Basic Perturbation (Foundation)

1. Implement reference orbit computation
   - Already have MPFR support ✅
   - Choose reference point (image center or periodic point)
   - Store orbit: `Vec<(Mpfr, Mpfr)>`

2. Implement delta iteration
   - Rewrite `calculate_all_delta()` to actually use perturbation
   - Use `Complex64` for deltas
   - Formula: `Δz_{n+1} = 2·Z_n·Δz_n + Δz_n² + Δc`

3. Add basic glitch detection
   - Pauldelbrot's criterion: `|Z + Δz| < 1e-6 * |Z|`
   - Mark glitched pixels
   - Re-render with new reference from glitched region

**Expected speedup:** 10-100x for deep zooms

### Phase 2: BLA Implementation (Major acceleration)

1. Compute BLA table from reference orbit
   - Level 0: `A_n = 2·Z_n`, `B_n = 0`
   - Level k: Hierarchical merging

2. Modify iteration loop
   - Check BLA validity each iteration
   - Apply largest valid BLA
   - Fall back to perturbation when invalid

3. Tune epsilon parameter
   - Start with 1e-6
   - Experiment with adaptive values

**Expected speedup:** Additional 5-10x on top of perturbation

### Phase 3: GPU Port (Maximum performance)

1. Reference orbit stays on CPU (MPFR)
2. Transfer BLA table to GPU
3. Parallel delta iteration in CUDA/OpenCL
4. Glitch detection on CPU after render

**Expected speedup:** 10-100x depending on GPU

### Alternative: Use Existing Libraries

Rather than implementing from scratch, consider:
- Wrapping Kalles Fraktaler as library
- Contributing to DeepDrill
- Using FractalShark's CUDA kernels
- Focusing on Rust-specific innovations (safety, type system)

---

## Open Research Questions

### Theoretical

1. **BLA error bounds:** Still no rigorous proof of convergence
2. **Optimal epsilon:** How to compute automatically per-location?
3. **Rebasing strategy:** When to create new reference orbits?
4. **Glitch prediction:** Can we predict glitches before they occur?

### Implementation

1. **GPU BLA:** Optimal memory layout for BLA tables on GPU?
2. **Hybrid precision:** Can we mix float32/float64 adaptively?
3. **Distributed rendering:** How to split work across multiple machines?
4. **Real-time zooming:** Can we compute on-the-fly without pre-rendering?

### Novel Approaches

1. **Machine learning:** Neural networks to predict iteration counts?
2. **Quantum computing:** Could quantum algorithms help?
3. **Sparse representations:** Compress reference orbits?
4. **Alternative number systems:** Unum? Posit arithmetic?

---

## References and Further Reading

### Key Papers
- K.I. Martin (2013): "SuperFractalThing" - sft_maths.pdf
- Pauldelbrot (2014): FractalForums.org post on glitch detection
- Zhuoran (2021): FractalForums.org - "Another solution to perturbation glitches"

### Tutorials
- mathr.co.uk/blog/2021-05-14_deep_zoom_theory_and_practice.html
- mathr.co.uk/blog/2022-02-21_deep_zoom_theory_and_practice_again.html (BLA)
- philthompson.me/2022/Perturbation-Theory-and-the-Mandelbrot-set.html
- philthompson.me/2023/Faster-Mandelbrot-Set-Rendering-with-BLA.html

### Software
- Kalles Fraktaler: mathr.co.uk/kf/kf.html
- DeepDrill: dirkwhoffmann.github.io/DeepDrill/
- FractalShark: github.com/mattsaccount364/FractalShark

### Community
- FractalForums.org - Active discussions
- r/fractals on Reddit
- fractalwiki.org - Growing documentation

---

## Conclusion

The world of Mandelbrot deep zooms has been revolutionized since 2016. The combination of:
- Perturbation theory (high-precision reference + low-precision deltas)
- BLA (skip iterations in linear regions)
- Automatic glitch correction (detect or prevent precision loss)

...has enabled zoom depths and rendering speeds that were unimaginable in 2016.

The good news: This codebase has the right foundation (MPFR + Rayon). The delta function stub shows we were thinking about perturbation theory. Adding these modern techniques would put mandelrust at the cutting edge of fractal rendering.

The exciting part: GPU implementation is the natural next step, and Rust's memory safety + CUDA interop could make this the safest high-performance fractal renderer in existence.

---

**Document Version:** 1.0
**Last Updated:** November 5, 2025
**Researched by:** Claude (Anthropic)
**Questions/Corrections:** Open a GitHub issue!
