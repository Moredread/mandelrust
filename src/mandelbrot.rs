use palette::{Rgb, Hsv, Gradient, IntoColor};
use std::fs::File;
use std::path::Path;
use pbr::ProgressBar;
use std::ops::{Mul, Add, Neg, Sub};
use driver::Driver2d;
use image;

#[derive(Copy, Clone)]
pub struct CanvasSize {
    pub pixel_width: u32,
    pub pixel_height: u32,
    top: f64,
    bottom: f64,
    left: f64,
    right: f64,
}

impl CanvasSize {
    pub fn new(pixel_width: u32,
               pixel_height: u32,
               top: f64,
               bottom: f64,
               left: f64,
               right: f64)
               -> CanvasSize {
        CanvasSize {
            pixel_width: pixel_width,
            pixel_height: pixel_height,
            top: top,
            bottom: bottom,
            left: left,
            right: right,
        }
    }

    pub fn new_from_center(pixel_width: u32,
                           pixel_height: u32,
                           center: [f64; 2],
                           zoom: f64)
                           -> CanvasSize {
        let aspect = pixel_height as f64 / pixel_width as f64;
        let width = 3.0 / zoom;
        let height = width * aspect;

        let top = center[1] + height / 2.0;
        let bottom = center[1] - height / 2.0;
        let right = center[0] + width / 2.0;
        let left = center[0] - width / 2.0;

        CanvasSize::new(pixel_width, pixel_height, top, bottom, left, right)
    }

    fn coordinates(&self, x: u32, y: u32) -> (f64, f64) {
        let x_ = self.left() +
                 (self.right() - self.left()) * (x as f64) / (self.pixel_width as f64);
        let y_ = self.top() +
                 (self.bottom() - self.top()) * (y as f64) / (self.pixel_height as f64);
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

    fn bottom(&self) -> f64 {
        self.bottom
    }

    fn top(&self) -> f64 {
        self.top
    }

    fn left(&self) -> f64 {
        self.left
    }

    fn right(&self) -> f64 {
        self.right
    }
}

pub fn iterate<T: Add<Output=T> + Mul<Output=T> + Neg + Sub<Output=T> + From<f64> + Clone + PartialOrd>(x0: T, y0: T, max_iterations: u32) -> Option<u32>
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

pub fn calculate_all(canvas_size: CanvasSize, max_iterations: u32) -> Vec<u32> {
    let mut v = Vec::<u32>::with_capacity(canvas_size.pixel_count() as usize);
    let mut pb = ProgressBar::new(canvas_size.pixel_count() as u64);

    for i in 0..canvas_size.pixel_count() {
        let (p_x, p_y) = canvas_size.idx_to_coord(i as usize);
        let (x, y) = canvas_size.coordinates(p_x, p_y);

        v.push(iterate::<f64>(x, y, max_iterations).unwrap_or(max_iterations));
        if i % canvas_size.pixel_height == 0 {
            pb.add(canvas_size.pixel_height as u64);
        };
    }

    v
}

pub fn make_image(data: Vec<u32>, canvas_size: CanvasSize, max_iterations: u32) -> image::RgbImage {
    let mut imgbuf = image::RgbImage::new(canvas_size.pixel_width, canvas_size.pixel_height);

    let n_colors = 256u32;

    let grad = Gradient::new(vec![Hsv::from(Rgb::new(1.0, 0.0, 0.0)),
                                  Hsv::from(Rgb::new(0.0, 1.0, 1.0))]);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let i = data[canvas_size.coord_to_idx(x, y)];
        let color: [u8; 3] = match i == max_iterations {
            true => [0, 0, 0],
            false => grad.get((i % n_colors) as f32 / n_colors as f32).into_rgb().to_pixel(),
        };
        *pixel = image::Rgb(color);
    }
    imgbuf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_canvas_size() {
        let c = CanvasSize::new(900, 600, 1.0, -1.0, -2.0, 1.0);

        assert_eq!(c.top(), 1.0);
        assert_eq!(c.bottom(), -1.0);
        assert_eq!(c.left(), -2.0);
        assert_eq!(c.right(), 1.0);
    }

    #[test]
    fn new_canvas_size_from_center() {
        let c = CanvasSize::new_from_center(900, 600, [-0.5, 0.0], 1.0);

        assert_eq!(c.top(), 1.0);
        assert_eq!(c.bottom(), -1.0);
        assert_eq!(c.left(), -2.0);
        assert_eq!(c.right(), 1.0);
    }

    #[test]
    fn new_canvas_size_from_center_and_zoom() {
        let c = CanvasSize::new_from_center(900, 600, [-0.5, 0.0], 2.0);

        assert_eq!(c.top(), 0.5);
        assert_eq!(c.bottom(), -0.5);
        assert_eq!(c.left(), -1.25);
        assert_eq!(c.right(), 0.25);
    }


}
