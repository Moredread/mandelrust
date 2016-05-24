use palette::{Rgb, Hsv, Gradient, IntoColor};
use std::ops::{Mul, Add, Neg, Sub};
use rayon::prelude::*;
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

    fn width(&self) -> f64 {
        self.right - self.left
    }

    fn height(&self) -> f64 {
        self.top - self.bottom
    }

    fn center(&self) -> [f64; 2] {
        [self.left + self.width() / 2.0, self.bottom + self.height() / 2.0]
    }

    pub fn zoom(&self, zoom: f64) -> CanvasSize {
        CanvasSize::new_from_center(self.pixel_width, self.pixel_height, self.center(), self.get_zoom() * zoom)
    }

    pub fn get_zoom(&self) -> f64 {
        3.0 / self.width()
    }

    pub fn move_center(&self, new_center: [f64; 2]) -> CanvasSize {
        CanvasSize::new_from_center(self.pixel_width, self.pixel_height, new_center, self.get_zoom())
    }

    pub fn move_center_to_pixel(&self, coord: [f64; 2]) -> CanvasSize {
        let new_center = self.coordinates([coord[0] as u32, coord[1] as u32]);

        self.move_center(new_center)
    }

    fn coordinates(&self, pixel_coordinates: [u32; 2]) -> [f64; 2] {
        let x_ = self.left +
                 (self.right - self.left) * (pixel_coordinates[0] as f64) / (self.pixel_width as f64);
        let y_ = self.top +
                 (self.bottom - self.top) * (pixel_coordinates[1] as f64) / (self.pixel_height as f64);
        [x_, y_]
    }

    fn coord_to_idx(&self, c: [u32; 2]) -> usize {
        assert!(c[0] < self.pixel_width);
        assert!(c[1] < self.pixel_height);
        (self.pixel_width * c[1] + c[0]) as usize
    }

    fn idx_to_coord(&self, idx: usize) -> [u32; 2] {
        let y = (idx as u32) / self.pixel_width;
        let x = (idx as u32) - y * self.pixel_width;
        [x, y]
    }

    fn pixel_count(&self) -> u32 {
        self.pixel_width * self.pixel_height
    }
}

pub fn iterate<T>(x0: T, y0: T, max_iterations: u32) -> Option<u32>
    where T: Add<Output=T> + for<'a> Add<&'a T, Output=T>
           + Mul<Output=T> + for<'a> Mul<&'a T, Output=T>
           + Neg + Sub<Output=T> + From<f64>
           + Clone + PartialOrd,
          for <'a> &'a T: Mul<Output=T>
{
    let mut i = 0;
    let mut x = T::from(0.0f64);
    let mut y = T::from(0.0f64);

    while &x * &x + &y * &y < T::from(4.0f64) && (i < max_iterations) {
        let xtemp = &x * &x - &y * &y + &x0;
        y = T::from(2.0f64) * &x * &y + &y0;
        x = xtemp;
        i += 1;
    }

    if i != max_iterations { Some(i) } else { None }
}

pub fn calculate_all(canvas_size: CanvasSize, max_iterations: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    (0..canvas_size.pixel_count()).into_par_iter().weight_max().map(|i| {
        canvas_size.coordinates(canvas_size.idx_to_coord(i as usize))
    }).map(|c| {
        iterate::<f64>(c[0], c[1], max_iterations).unwrap_or(max_iterations)
    }).collect_into(&mut v);
    v
}

pub fn make_image(data: Vec<u32>, canvas_size: CanvasSize, max_iterations: u32) -> image::RgbImage {
    let n_colors = 256u32;
    let grad = Gradient::new(vec![Hsv::from(Rgb::new(1.0, 0.0, 0.0)),
                                  Hsv::from(Rgb::new(0.0, 1.0, 1.0))]);

    image::RgbImage::from_fn(
        canvas_size.pixel_width, canvas_size.pixel_height,
        |x, y| {
            let i = data[canvas_size.coord_to_idx([x, y])];
            image::Rgb(if i == max_iterations {
                [0, 0, 0]
            } else {
                grad.get((i % n_colors) as f32 / n_colors as f32).into_rgb().to_pixel()
            })
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_canvas_size() {
        let c = CanvasSize::new(900, 600, 1.0, -1.0, -2.0, 1.0);

        assert_eq!(c.top, 1.0);
        assert_eq!(c.bottom, -1.0);
        assert_eq!(c.left, -2.0);
        assert_eq!(c.right, 1.0);
        assert_eq!(c.height(), 2.0);
        assert_eq!(c.width(), 3.0);
    }

    #[test]
    fn new_canvas_size_from_center() {
        let c = CanvasSize::new_from_center(900, 600, [-0.5, 0.0], 1.0);

        assert_eq!(c.top, 1.0);
        assert_eq!(c.bottom, -1.0);
        assert_eq!(c.left, -2.0);
        assert_eq!(c.right, 1.0);
        assert_eq!(c.height(), 2.0);
        assert_eq!(c.width(), 3.0);
    }

    #[test]
    fn new_canvas_size_from_center_and_zoom() {
        let c = CanvasSize::new_from_center(900, 600, [-0.5, 0.0], 2.0);

        assert_eq!(c.top, 0.5);
        assert_eq!(c.bottom, -0.5);
        assert_eq!(c.left, -1.25);
        assert_eq!(c.right, 0.25);
        assert_eq!(c.height(), 1.0);
        assert_eq!(c.width(), 1.5);
    }

    #[test]
    fn test_center() {
        let c = CanvasSize::new_from_center(900, 600, [-0.5, 0.0], 1.0);

        assert_eq!(c.center(), [-0.5f64, 0.0]);
    }

    #[test]
    fn test_width_and_height() {
        let c = CanvasSize::new_from_center(900, 600, [-0.5, 0.0], 1.0);

        assert_eq!(c.width(), 3.0);
        assert_eq!(c.height(), 2.0);
    }

    #[test]
    fn test_to_zoom() {
        let center = [-0.5f64, 0.0];
        let c = CanvasSize::new_from_center(900, 600, center, 1.0);
        let zoomed = c.zoom(2.0);

        assert_eq!(zoomed.center(), center);
        assert_eq!(zoomed.height(), 1.0);
        assert_eq!(zoomed.width(), 1.5);

        let zoomed_again = zoomed.zoom(2.0);
        assert_eq!(zoomed_again.center(), center);
        assert_eq!(zoomed_again.height(), 0.5);
        assert_eq!(zoomed_again.width(), 0.75);
    }

    #[test]
    fn test_move_center() {
        let new_center = [0.0f64, -1.0];
        let c = CanvasSize::new_from_center(900, 600, [-0.5, 0.0], 1.0);

        assert_eq!(c.move_center(new_center).center(), new_center);
    }
}
