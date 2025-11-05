//! Simple benchmarks
#![feature(test)]

extern crate test;

use test::Bencher;
use mandelrust::mandelbrot::*;
use rug::Float;

const BENCH_PREC: u32 = 53;

#[bench]
fn bench_iterate_float(b: &mut Bencher) {
    let max = 1000;

    b.iter(|| { iterate::<f64>(-0.5f64, 0f64, max) });
}

#[bench]
fn bench_iterate_mpfr(b: &mut Bencher) {
    let max = 1000;
    let m1 = Float::with_val(BENCH_PREC, -0.5);
    let m2 = Float::with_val(BENCH_PREC, 0.0);

    b.iter(|| { iterate_float(m1.clone(), m2.clone(), max) });
}

#[bench]
fn bench_calculate_all(b: &mut Bencher) {
    let max = 1000;
    let c = CanvasSize::new_from_center(50, 50,
        [Float::with_val(BENCH_PREC, -0.5), Float::with_val(BENCH_PREC, 0.0)],
        Float::with_val(BENCH_PREC, 1.0));

    b.iter(|| calculate_all_mpfr(c.clone(), max));
}

#[bench]
fn bench_make_image(b: &mut Bencher) {
    let max = 1000;
    let c = CanvasSize::new_from_center(50, 50,
        [Float::with_val(BENCH_PREC, -0.5), Float::with_val(BENCH_PREC, 0.0)],
        Float::with_val(BENCH_PREC, 1.0));
    let data = calculate_all_mpfr(c.clone(), max);

    b.iter(|| make_image(data.clone(), c.clone(), max));
}
