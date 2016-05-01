extern crate rust_mpfr;

use rust_mpfr::mpfr::Mpfr;

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

fn iterate(x0: f64, y0: f64, max_iterations: u32) -> Option<u32> {
    let mut i = 1;
    let mut x = x0;
    let mut y = y0;

    while (x * x + y * y < 4.0) && (i < max_iterations) {
        let xtemp = x * x - y * y + x0;
        y = 2.0 * x * y + y0;
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

    let (x, y) = c.coordinates(1024, 1024);

    let max = 1000000;
    let i = iterate(0.0, 0.0, max).unwrap_or(0);

    println!("Hello, world! {} {} {} {}", v, x, y, i);
}
