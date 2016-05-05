#[macro_use]
extern crate timeit;
extern crate rust_mpfr;
extern crate pbr;
extern crate image;
extern crate palette;

use palette::{Rgb, Hsv, Gradient, IntoColor};
use std::fs::File;
use std::path::Path;
use pbr::ProgressBar;
use rust_mpfr::mpfr::Mpfr;
use std::ops::{Mul, Add, Neg, Sub};

#[derive(Copy, Clone)]
struct CanvasSize {
    pixel_width: u32,
    pixel_height: u32,
    top: f64,
    bottom: f64,
    left: f64,
    right: f64,
}

impl CanvasSize {
    fn coordinates(&self, x: u32, y: u32) -> (f64, f64) {
        let x_ = self.left + (self.right - self.left) * (x as f64) / (self.pixel_width as f64);
        let y_ = self.top + (self.bottom - self.top) * (y as f64) / (self.pixel_height as f64);
        return (x_, y_);
    }

    fn coord_to_idx(&self, x: u32, y: u32) -> usize {
        assert!(x < self.pixel_width);
        assert!(y < self.pixel_height);
        (self.pixel_width * y + x) as usize
    }

    fn idx_to_coord(&self, idx: usize) -> (u32, u32) {
        let y = (idx as u32) / self.pixel_width;
        let x = (idx as u32) - y * self.pixel_width;
        (x, y)
    }

    fn pixel_count(&self) -> u32 {
        self.pixel_width * self.pixel_height
    }
}

fn iterate<T: Add<Output=T> + Mul<Output=T> + Neg + Sub<Output=T> + From<f64> + Clone + PartialOrd>(x0: T, y0: T, max_iterations: u32) -> Option<u32>
{
    let mut i = 0;
    let mut x = T::from(0.0f64);
    let mut y = T::from(0.0f64);

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

fn calculate_all(canvas_size: CanvasSize, max_iterations: u32) -> Vec<u32> {
    let mut v = Vec::<u32>::with_capacity(canvas_size.pixel_count() as usize);
    let mut pb = ProgressBar::new(canvas_size.pixel_count() as u64);

    for i in 0..canvas_size.pixel_count() {
        let (p_x, p_y) = canvas_size.idx_to_coord(i as usize);
        let (x, y) = canvas_size.coordinates(p_x, p_y);

        v.push(iterate::<f64>(x, y, max_iterations).unwrap_or(max_iterations));
        if i % canvas_size.pixel_height == 0 && i != 0 { pb.add(canvas_size.pixel_height as u64); };
    }

    v
}

fn main() {
    let c = CanvasSize {
        pixel_width: 1280,
        pixel_height: 1280,
        top: 1.0,
        bottom: -1.0,
        left: -2.0,
        right: 1.0
    };
    let max = 256u32;

    let v = calculate_all(c, max);
    let mut imgbuf = image::ImageBuffer::new(c.pixel_width, c.pixel_height);

    let n_colors = 256u32;

    let grad = Gradient::new(vec![
        Hsv::from(Rgb::new(1.0f32, 0.1, 0.1)),
        Hsv::from(Rgb::new(0.1, 1.0, 1.0))
    ]);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let i = v[c.coord_to_idx(x, y)];
        let c: [u8; 3] = match i == max {
            true => [0, 0, 0],
            false => grad.get((i % n_colors) as f32 / n_colors as f32).into_rgb().to_pixel(),
        };
        *pixel = image::Rgb(c);
    }

    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
