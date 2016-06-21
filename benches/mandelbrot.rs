//! Simple benchmarks
#![feature(test)]

extern crate test;
#[macro_use]
extern crate rust_mpfr;
extern crate mandelrust;

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
    let c = CanvasSize::new_from_center(50, 50, [mpfr!(-0.5), mpfr!(0.0)], mpfr!(1.0));

    b.iter(|| calculate_all_mpfr(c.clone(), max));
}

#[bench]
fn bench_make_image(b: &mut Bencher) {
    let max = 1000;
    let c = CanvasSize::new_from_center(50, 50, [mpfr!(-0.5), mpfr!(0.0)], mpfr!(1.0));
    let data = calculate_all_mpfr(c.clone(), max);

    b.iter(|| make_image(data.clone(), c.clone(), max));
}
