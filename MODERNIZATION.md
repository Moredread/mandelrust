# Modernization Changes

This document describes the changes made to modernize the codebase from 2016 to 2025.

## Summary

The codebase has been updated to work with modern Rust (2021 edition) and updated dependencies. The core Mandelbrot calculation library has been fully modernized, while the old Piston-based GUI has been removed due to deprecated dependencies.

## Major Changes

### 1. Rust Edition
- **Updated from:** Rust 2015
- **Updated to:** Rust 2021
- Removed all `extern crate` declarations (no longer needed in Rust 2021)

### 2. MPFR Library Migration
- **Old:** `rust-mpfr` (git dependency, unmaintained, 9+ years old)
- **New:** `rug` v1.26 (modern, actively maintained MPFR bindings)
- **Changes:**
  - `Mpfr` type → `rug::Float`
  - `mpfr!(value)` macro → `Float::with_val(precision, value)`
  - `.get_prec()` → `.prec()`
  - `Mpfr::new2(prec)` → `Float::new(prec)` or `Float::with_val(prec, val)`
  - Precision changed from `usize` to `u32` (rug's native type)

### 3. Updated Dependencies

| Crate | Old Version | New Version |
|-------|-------------|-------------|
| image | 0.6 | 0.25 |
| palette | * | 0.7 |
| rayon | * | 1.10 |
| num | 0.1 | 0.4 |
| rug | N/A | 1.26 |

### 4. Removed Dependencies

The following Piston game engine dependencies were removed as they are no longer maintained:
- `piston` (0.17)
- `glutin` (0.4)
- `glium` (0.13)
- `piston2d-graphics` (0.13)
- `piston2d-glium_graphics` (0.19)
- `pistoncore-glutin_window` (0.20)
- `pistoncore-input` (0.8)
- `shader_version` (0.2)
- `carboxyl` (0.2)
- `carboxyl_window` (0.0.3)
- `benzene` (0.2)
- `clippy` (now built into rustup)

### 5. Files Updated

#### Fully Migrated
- `src/lib.rs` - Simplified, removed extern crate declarations
- `src/mandelbrot.rs` - Complete migration to rug::Float
  - All struct methods updated
  - All tests updated
  - All helper functions updated
- `benches/mandelbrot.rs` - Updated benchmarks to use rug
- `examples/orbit.rs` - Updated to use rug with high-precision float parsing

#### Replaced
- `src/main.rs` - New simple image generator (old GUI version saved as `main.rs.old`)

#### Archived (renamed to .old)
- `src/main.rs.old` - Old Piston-based interactive GUI
- `src/driver.rs.old` - Old Piston driver
- `src/app.rs.old` - Old Piston application logic

## Building

```bash
# Build library only
cargo build --lib

# Build with main binary (generates mandelbrot.png)
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo +nightly bench
```

## Usage

### Library Usage

```rust
use mandelrust::mandelbrot::*;
use rug::Float;

let prec = 128u32;
let canvas = CanvasSize::new_from_center(
    1920, 1080,
    [Float::with_val(prec, -0.5), Float::with_val(prec, 0.0)],
    Float::with_val(prec, 1.0)
);

let data = calculate_all_mpfr(canvas.clone(), 1000);
let img = make_image(data, canvas, 1000);
img.save("output.png").unwrap();
```

### Binary Usage

```bash
cargo run --release
# Generates mandelbrot.png
```

## Future Improvements

To restore interactive GUI functionality, consider:
- Using `winit` + `wgpu` for cross-platform GPU-accelerated rendering
- Using `egui` for a modern immediate-mode GUI
- Using `iced` for a reactive GUI framework
- Or keeping it simple with `pixels` + `winit` for 2D pixel manipulation

## System Dependencies

The `rug` crate requires:
- GMP (GNU Multiple Precision Arithmetic Library)
- MPFR (Multiple Precision Floating-Point Reliable Library)
- MPC (Multiple Precision Complex Library)

On Ubuntu/Debian:
```bash
sudo apt-get install libgmp-dev libmpfr-dev libmpc-dev
```

On macOS:
```bash
brew install gmp mpfr libmpc
```

On Windows:
- Pre-built binaries are available through rug's build system
- Or use MSYS2: `pacman -S mingw-w64-x86_64-gmp mingw-w64-x86_64-mpfr mingw-w64-x86_64-mpc`
