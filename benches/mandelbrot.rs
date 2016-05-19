//! Simple benchmarks
#![feature(test)]

extern crate test;
extern crate mandelrust;
extern crate rust_mpfr;

use test::Bencher;
use mandelrust::mandelbrot::*;
use rust_mpfr::mpfr::*;

#[bench]
fn bench_iterate_float(b: &mut Bencher) {
    let max = 1000;
    b.iter(|| { iterate::<f64>(-0.5f64, 0f64, max) });
}

#[bench]
fn bench_iterate_mpfr(b: &mut Bencher) {
    let max = 1000;
    let m1 = Mpfr::new_from_str("-0.5", 10).unwrap();
    let m2 = Mpfr::new_from_str("0.0", 10).unwrap();
    b.iter(|| { iterate::<Mpfr>(m1.clone(), m2.clone(), max) });
}

#[bench]
fn bench_calculate_all(b: &mut Bencher) {
    let max = 1000;
    let c = CanvasSize::new_from_center(200, 200, [-0.5, 0.0], 1.0);
    b.iter(|| calculate_all(c, max));
}
