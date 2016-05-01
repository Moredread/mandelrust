#[macro_use]
extern crate timeit;
extern crate rust_mpfr;
extern crate pbr;
extern crate image;

use std::fs::File;
use std::path::Path;
use pbr::ProgressBar;
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

fn main() {
    let v: Mpfr = From::<f64>::from(1.234567);

    let c = canvas_size { pixel_width: 1280, pixel_height: 1280, top: 1.0, bottom: -1.0, left: -2.0, right: 1.0 };

    let max = 256u32;

    let mut v = Vec::<u32>::with_capacity(c.pixel_count() as usize);

    let mut pb = ProgressBar::new(c.pixel_count() as u64);
//    pb.format("╢▌▌░╟");

    for i in 0..c.pixel_count() {
        let (p_x, p_y) = c.idx_to_coord(i as usize);
        let (x, y) = c.coordinates(p_x, p_y);

//        println!("{} {} {} {} {}", i, p_x, p_y, x, y);

        v.push(iterate::<f64>(x, y, max).unwrap_or(max));
        if i % c.pixel_height == 0 && i != 0 { pb.add(c.pixel_height as u64); };
    }

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(c.pixel_width, c.pixel_height);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let i = v[c.coord_to_idx(x, y)];
//        println!("{} {} {}", x, y, i);
        *pixel = image::Luma([i as u8]);
    }

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);

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
