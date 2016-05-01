#[macro_use]
extern crate timeit;
extern crate rust_mpfr;

use rust_mpfr::mpfr::Mpfr;
use std::ops::{Mul, Add, Neg, Sub};

#[derive(Copy, Clone)]
struct canvas_size {
    pixel_width: u32,
    pixel_height: u32,
    top: f64,
    bottom: f64,
    left: f64,
    right: f64,
}

impl canvas_size {
    fn coordinates(&self, x: u32, y: u32) -> (f64, f64) {
        let x_ = self.left + (self.right - self.left) * (x as f64) / (self.pixel_width as f64);
        let y_ = self.top + (self.bottom - self.top) * (y as f64) / (self.pixel_height as f64);
        return (x_, y_);
    }
}

fn iterate<T: Add<Output=T> + Mul<Output=T> + Neg + Sub<Output=T> + From<f64> + Clone + PartialOrd>(x0: T, y0: T, max_iterations: u32) -> Option<u32>
{
    let mut i = 1;
    let mut x = x0.clone();
    let mut y = y0.clone();

    while x.clone() * x.clone() + y.clone() * y.clone() < T::from(4.0f64) && (i < max_iterations) {
        let xtemp = x.clone() * x.clone() - y.clone() * y.clone() + x0.clone();
        y = (T::from(2.0f64)) * x * y + y0.clone();
        x = xtemp;
        i += 1;
    }

    match i != max_iterations {
        true => Some(i),
        false => None,
    }
}

fn main() {
    let v: Mpfr = From::<f64>::from(1.234567);

    let c = canvas_size { pixel_width: 1024, pixel_height: 1024, top: 1.0, bottom: -1.0, left: -2.0, right: 1.0 };

    let max = 10000;

    timeit!({
        let x_ = Mpfr::new2_from_str(1024, "0.0", 10).unwrap();
        let y_ = x_.clone();

        iterate::<Mpfr>(x_, y_, max);
    });

    timeit!({
        let x_ = Mpfr::new2_from_str(1024000, "0.0", 10).unwrap();
        let y_ = x_.clone();

        iterate::<Mpfr>(x_, y_, max);
    });

    timeit!({
        iterate::<f64>(0.0f64, 0.0f64, max).unwrap_or(0);
    });

}
