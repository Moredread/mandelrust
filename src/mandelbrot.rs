use palette::{Hsv, Srgb, IntoColor};
use std::ops::{Mul, Add, Neg, Sub};
use rayon::prelude::*;
use rug::{Float, Assign};
use image;
use num::complex::Complex64;
use std::fmt::Display;

// Default precision for MPFR floats (in bits)
#[allow(dead_code)]
const DEFAULT_PREC: u32 = 53;

#[derive(Clone)]
pub struct CanvasSize {
    pub pixel_width: u32,
    pub pixel_height: u32,
    top: Float,
    bottom: Float,
    left: Float,
    right: Float,
}

impl CanvasSize {
    pub fn new(pixel_width: u32,
               pixel_height: u32,
               top: Float,
               bottom: Float,
               left: Float,
               right: Float)
               -> CanvasSize {
        CanvasSize {
            pixel_width,
            pixel_height,
            top,
            bottom,
            left,
            right,
        }
    }

    pub fn get_prec(&self) -> u32 {
        self.top.prec()
    }

    pub fn set_prec(&self, prec: u32) -> CanvasSize {
        let new_top = Float::with_val(prec, &self.top);
        let new_bottom = Float::with_val(prec, &self.bottom);
        let new_left = Float::with_val(prec, &self.left);
        let new_right = Float::with_val(prec, &self.right);

        CanvasSize::new(self.pixel_width,
                        self.pixel_height,
                        new_top,
                        new_bottom,
                        new_left,
                        new_right)
    }

    pub fn new_from_center(pixel_width: u32,
                           pixel_height: u32,
                           center: [Float; 2],
                           zoom: Float)
                           -> CanvasSize {
        let prec = center[0].prec();
        let aspect = pixel_height as f64 / pixel_width as f64;
        let width = Float::with_val(prec, 3.0 / &zoom);
        let height = Float::with_val(prec, &width * aspect);

        let half_height = Float::with_val(prec, &height / 2.0);
        let half_width = Float::with_val(prec, &width / 2.0);

        let top = Float::with_val(prec, &center[1] + &half_height);
        let bottom = Float::with_val(prec, &center[1] - &half_height);
        let right = Float::with_val(prec, &center[0] + &half_width);
        let left = Float::with_val(prec, &center[0] - &half_width);

        CanvasSize::new(pixel_width, pixel_height, top, bottom, left, right)
    }

    fn width(&self) -> Float {
        Float::with_val(self.get_prec(), &self.right - &self.left)
    }

    fn height(&self) -> Float {
        Float::with_val(self.get_prec(), &self.top - &self.bottom)
    }

    pub fn center(&self) -> [Float; 2] {
        let prec = self.get_prec();
        [Float::with_val(prec, &self.left + self.width() / 2.0),
         Float::with_val(prec, &self.bottom + self.height() / 2.0)]
    }

    pub fn zoom(&self, zoom: Float) -> CanvasSize {
        CanvasSize::new_from_center(self.pixel_width,
                                    self.pixel_height,
                                    self.center(),
                                    Float::with_val(self.get_prec(), self.get_zoom() * zoom))
    }

    pub fn get_zoom(&self) -> Float {
        Float::with_val(self.get_prec(), 3.0 / self.width())
    }

    pub fn move_center(&self, new_center: [Float; 2]) -> CanvasSize {
        CanvasSize::new_from_center(self.pixel_width,
                                    self.pixel_height,
                                    new_center,
                                    self.get_zoom())
    }

    pub fn move_center_to_pixel(&self, coord: [f64; 2]) -> CanvasSize {
        let new_center = self.coordinates([coord[0] as u32, coord[1] as u32]);

        self.move_center(new_center)
    }

    pub fn coordinates(&self, pixel_coordinates: [u32; 2]) -> [Float; 2] {
        let prec = self.get_prec();
        let x_ratio = pixel_coordinates[0] as f64 / self.pixel_width as f64;
        let y_ratio = pixel_coordinates[1] as f64 / self.pixel_height as f64;

        let x_range = Float::with_val(prec, &self.right - &self.left);
        let y_range = Float::with_val(prec, &self.bottom - &self.top);

        let x_offset = Float::with_val(prec, x_range * x_ratio);
        let y_offset = Float::with_val(prec, y_range * y_ratio);

        let x_ = Float::with_val(prec, &self.left + x_offset);
        let y_ = Float::with_val(prec, &self.top + y_offset);
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

// Generic version for types like f64
pub fn iterate<T>(x0: T, y0: T, max_iterations: u32) -> Option<u32>
    where T: Add<Output=T> + for<'a> Add<&'a T, Output=T>
           + Mul<Output=T> + for<'a> Mul<&'a T, Output=T>
           + Neg + Sub<Output=T> + From<f64>
           + Clone + PartialOrd + Display,
          for <'a> &'a T: Mul<Output=T>
{
    let i = iterate_all::<T>(x0, y0, max_iterations).len() as u32;

    if i != max_iterations {
        Some(i)
    } else {
        None
    }
}

pub fn iterate_all<T>(x0: T, y0: T, max_iterations: u32) -> Vec<(T, T)>
    where T: Add<Output=T> + for<'a> Add<&'a T, Output=T>
           + Mul<Output=T> + for<'a> Mul<&'a T, Output=T>
           + Neg + Sub<Output=T> + From<f64>
           + Clone + PartialOrd + Display,
          for <'a> &'a T: Mul<Output=T>
{
    let mut i = 0;
    let mut x = T::from(0.0f64);
    let mut y = T::from(0.0f64);
    let mut v = Vec::new();

    while &x * &x + &y * &y < T::from(4.0f64) && i < max_iterations {
        let xtemp = &x * &x - &y * &y + &x0;
        let ytemp = T::from(2.0f64) * &x * &y + &y0;

        v.push((xtemp.clone(), ytemp.clone()));
        i += 1;

        if x == xtemp && y == ytemp {
            for _ in 0..(max_iterations - i) {
                v.push((x.clone(), y.clone()));
            }
            i = max_iterations;
        }

        x = xtemp;
        y = ytemp;
    }

    v
}

// Specialized version for rug::Float
pub fn iterate_float(x0: Float, y0: Float, max_iterations: u32) -> Option<u32> {
    let i = iterate_all_float(x0, y0, max_iterations).len() as u32;

    if i != max_iterations {
        Some(i)
    } else {
        None
    }
}

pub fn iterate_all_float(x0: Float, y0: Float, max_iterations: u32) -> Vec<(Float, Float)> {
    let prec = x0.prec();
    let mut i = 0;
    let mut x = Float::with_val(prec, 0.0);
    let mut y = Float::with_val(prec, 0.0);
    let mut v = Vec::new();

    let four = Float::with_val(prec, 4.0);
    let two = Float::with_val(prec, 2.0);
    let mut magnitude_squared = Float::with_val(prec, 0.0);

    // Calculate initial magnitude
    magnitude_squared.assign(&x * &x + &y * &y);

    while magnitude_squared < four && i < max_iterations {
        let x_squared = Float::with_val(prec, &x * &x);
        let y_squared = Float::with_val(prec, &y * &y);
        let x_diff = Float::with_val(prec, &x_squared - &y_squared);
        let xtemp = Float::with_val(prec, x_diff + &x0);

        let two_x = Float::with_val(prec, &two * &x);
        let xy_prod = Float::with_val(prec, two_x * &y);
        let ytemp = Float::with_val(prec, xy_prod + &y0);

        v.push((xtemp.clone(), ytemp.clone()));
        i += 1;

        if x == xtemp && y == ytemp {
            for _ in 0..(max_iterations - i) {
                v.push((x.clone(), y.clone()));
            }
            i = max_iterations;
        }

        x = xtemp;
        y = ytemp;

        // Reuse magnitude_squared for next iteration check
        magnitude_squared.assign(&x * &x + &y * &y);
    }

    v
}

pub fn delta(d: Complex64, x_n: Complex64, input: [Complex64; 4]) -> (Complex64, [Complex64; 3]) {
    let a_n = input[0];
    let b_n = input[1];
    let c_n = input[2];

    let a_n1 = 2f64 * a_n * x_n + 1f64;
    let b_n1 = 2f64 * b_n * x_n + a_n * a_n;
    let c_n1 = 2f64 * c_n * x_n + a_n * b_n;
    let x_n1 = a_n1 * d + b_n1 * d * d + c_n1 * d * d * d;

    (x_n1, [a_n1, b_n1, c_n1])
}

pub fn calculate_all_mpfr(canvas_size: CanvasSize, max_iterations: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    (0..canvas_size.pixel_count())
        .into_par_iter()
        .map(|i| canvas_size.coordinates(canvas_size.idx_to_coord(i as usize)))
        .map(|c| {
            iterate_float(c[0].clone(), c[1].clone(), max_iterations)
                .unwrap_or(max_iterations)
        })
        .collect_into_vec(&mut v);
    v
}

pub fn calculate_all_delta(canvas_size: CanvasSize, max_iterations: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    (0..canvas_size.pixel_count())
        .into_par_iter()
        .map(|i| canvas_size.coordinates(canvas_size.idx_to_coord(i as usize)))
        .map(|c| {
            iterate_float(c[0].clone(), c[1].clone(), max_iterations)
                .unwrap_or(max_iterations)
        })
        .collect_into_vec(&mut v);
    v
}

fn color_from_iteration(iterations: u32, max_iterations: u32) -> [u8; 3] {
    const N_COLORS: u32 = 256u32;
    const BLACK: [u8; 3] = [0u8, 0u8, 0u8];

    if iterations == max_iterations {
        BLACK
    } else {
        // Create a color based on iteration count using HSV color space
        let hue = ((iterations % N_COLORS) as f32 / N_COLORS as f32) * 360.0;
        let hsv: Hsv = Hsv::new(hue, 1.0, 1.0);
        let rgb: Srgb<f32> = hsv.into_color();
        let (r, g, b) = rgb.into_components();
        [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]
    }
}

pub fn make_image(data: Vec<u32>, canvas_size: CanvasSize, max_iterations: u32) -> image::RgbImage {
    image::RgbImage::from_fn(canvas_size.pixel_width, canvas_size.pixel_height, |x, y| {
        let i = data[canvas_size.coord_to_idx([x, y])];
        image::Rgb(color_from_iteration(i, max_iterations))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper macro to create Float values for tests
    macro_rules! float {
        ($val:expr) => {
            Float::with_val(DEFAULT_PREC, $val)
        };
    }

    #[test]
    fn new_canvas_size() {
        let c = CanvasSize::new(900, 600, float!(1.0), float!(-1.0), float!(-2.0), float!(1.0));

        assert_eq!(c.top, float!(1.0));
        assert_eq!(c.bottom, float!(-1.0));
        assert_eq!(c.left, float!(-2.0));
        assert_eq!(c.right, float!(1.0));
        assert_eq!(c.height(), float!(2.0));
        assert_eq!(c.width(), float!(3.0));
    }

    #[test]
    fn new_canvas_size_from_center() {
        let c = CanvasSize::new_from_center(900, 600, [float!(-0.5), float!(0.0)], float!(1.0));

        assert_eq!(c.top, float!(1.0));
        assert_eq!(c.bottom, float!(-1.0));
        assert_eq!(c.left, float!(-2.0));
        assert_eq!(c.right, float!(1.0));
        assert_eq!(c.height(), float!(2.0));
        assert_eq!(c.width(), float!(3.0));
    }

    #[test]
    fn new_canvas_size_from_center_and_zoom() {
        let c = CanvasSize::new_from_center(900, 600, [float!(-0.5), float!(0.0)], float!(2.0));

        assert_eq!(c.top, float!(0.5));
        assert_eq!(c.bottom, float!(-0.5));
        assert_eq!(c.left, float!(-1.25));
        assert_eq!(c.right, float!(0.25));
        assert_eq!(c.height(), float!(1.0));
        assert_eq!(c.width(), float!(1.5));
    }

    #[test]
    fn test_center() {
        let c = CanvasSize::new_from_center(900, 600, [float!(-0.5), float!(0.0)], float!(1.0));

        assert_eq!(c.center(), [float!(-0.5), float!(0.0)]);
    }

    #[test]
    fn test_width_and_height() {
        let c = CanvasSize::new_from_center(900, 600, [float!(-0.5), float!(0.0)], float!(1.0));

        assert_eq!(c.width(), float!(3.0));
        assert_eq!(c.height(), float!(2.0));
    }

    #[test]
    fn test_to_zoom() {
        let c = CanvasSize::new_from_center(900, 600, [float!(-0.5), float!(0.0)], float!(1.0));
        let zoomed = c.zoom(float!(2.0));

        assert_eq!(zoomed.center(), [float!(-0.5), float!(0.0)]);
        assert_eq!(zoomed.height(), float!(1.0));
        assert_eq!(zoomed.width(), float!(1.5));

        let zoomed_again = zoomed.zoom(float!(2.0));
        assert_eq!(zoomed_again.center(), [float!(-0.5), float!(0.0)]);
        assert_eq!(zoomed_again.height(), float!(0.5));
        assert_eq!(zoomed_again.width(), float!(0.75));
    }

    #[test]
    fn test_move_center() {
        let c = CanvasSize::new_from_center(900, 600, [float!(-0.5), float!(0.0)], float!(1.0));

        assert_eq!(c.move_center([float!(0.0), float!(-1.0)]).center(),
                   [float!(0.0), float!(-1.0)]);
    }

    #[test]
    fn test_pixel_count() {
        let c = CanvasSize::new_from_center(2, 3, [float!(-0.5), float!(0.0)], float!(1.0));

        assert_eq!(c.pixel_count(), 6);
    }

    #[test]
    fn test_iterate_all() {
        let c = CanvasSize::new_from_center(2, 3, [float!(-0.5), float!(0.0)], float!(1.0));
        let x = c.center()[0].clone();
        let y = c.center()[1].clone();
        let v = iterate_all_float(x, y, 10);

        assert_eq!(v.len(), 10);
    }

    #[test]
    fn test_iterate_all_prec() {
        let prec = 128;
        let c = CanvasSize::new_from_center(2, 3, [float!(-0.5), float!(0.0)], float!(1.0))
            .set_prec(prec);
        let x = c.center()[0].clone();
        let y = c.center()[1].clone();
        let v = iterate_all_float(x, y, 10);

        assert_eq!(v[9].0.prec(), prec);
        assert_eq!(v[9].1.prec(), prec);
    }
}
