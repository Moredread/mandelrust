use palette::{Hsv, Gradient, IntoColor, RgbHue};
use std::ops::{Mul, Add, Neg, Sub};
use rayon::prelude::*;
use rust_mpfr::mpfr::*;
use image;
use num::complex::Complex64;
use std::fmt::Display;

#[derive(Clone)]
pub struct CanvasSize {
    pub pixel_width: u32,
    pub pixel_height: u32,
    top: Mpfr,
    bottom: Mpfr,
    left: Mpfr,
    right: Mpfr,
}

impl CanvasSize {
    pub fn new(pixel_width: u32,
               pixel_height: u32,
               top: Mpfr,
               bottom: Mpfr,
               left: Mpfr,
               right: Mpfr)
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

    pub fn get_prec(&self) -> usize {
        self.top.get_prec()
    }

    pub fn set_prec(&self, prec: usize) -> CanvasSize {
        let mut new_top = Mpfr::new2(prec);
        let mut new_bottom = Mpfr::new2(prec);
        let mut new_left = Mpfr::new2(prec);
        let mut new_right = Mpfr::new2(prec);

        new_top.set(&self.top);
        new_bottom.set(&self.bottom);
        new_left.set(&self.left);
        new_right.set(&self.right);

        CanvasSize::new(self.pixel_width,
                        self.pixel_height,
                        new_top,
                        new_bottom,
                        new_left,
                        new_right)
    }

    pub fn new_from_center(pixel_width: u32,
                           pixel_height: u32,
                           center: [Mpfr; 2],
                           zoom: Mpfr)
                           -> CanvasSize {
        let aspect = pixel_height as f64 / pixel_width as f64;
        let width = 3.0 / zoom;
        let height = &width * aspect;

        let top = &center[1] + &height / 2.0;
        let bottom = &center[1] - &height / 2.0;
        let right = &center[0] + &width / 2.0;
        let left = &center[0] - &width / 2.0;

        CanvasSize::new(pixel_width, pixel_height, top, bottom, left, right)
    }

    fn width(&self) -> Mpfr {
        &self.right - &self.left
    }

    fn height(&self) -> Mpfr {
        &self.top - &self.bottom
    }

    pub fn center(&self) -> [Mpfr; 2] {
        [&self.left + self.width() / 2.0, &self.bottom + self.height() / 2.0]
    }

    pub fn zoom(&self, zoom: Mpfr) -> CanvasSize {
        CanvasSize::new_from_center(self.pixel_width,
                                    self.pixel_height,
                                    self.center(),
                                    self.get_zoom() * zoom)
    }

    pub fn get_zoom(&self) -> Mpfr {
        3.0 / self.width()
    }

    pub fn move_center(&self, new_center: [Mpfr; 2]) -> CanvasSize {
        CanvasSize::new_from_center(self.pixel_width,
                                    self.pixel_height,
                                    new_center,
                                    self.get_zoom())
    }

    pub fn move_center_to_pixel(&self, coord: [f64; 2]) -> CanvasSize {
        let new_center = self.coordinates([coord[0] as u32, coord[1] as u32]);

        self.move_center(new_center)
    }

    pub fn coordinates(&self, pixel_coordinates: [u32; 2]) -> [Mpfr; 2] {
        let x_ = &self.left +
                 (&self.right - &self.left) *
                 (pixel_coordinates[0] as f64 / self.pixel_width as f64);
        let y_ = &self.top +
                 (&self.bottom - &self.top) *
                 (pixel_coordinates[1] as f64 / self.pixel_height as f64);
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

pub fn iterate_all_start<T>(x0: T, y0: T, x_start: T, y_start: T, max_iterations: u32) -> Vec<(T, T)>
    where T: Add<Output=T> + for<'a> Add<&'a T, Output=T>
           + Mul<Output=T> + for<'a> Mul<&'a T, Output=T>
           + Neg + Sub<Output=T> + From<f64>
           + Clone + PartialOrd + Display,
          for <'a> &'a T: Mul<Output=T>
{
    let mut i = 0;
    let mut x = x_start.clone();
    let mut y = y_start.clone();
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


pub fn iterate_all<T>(x0: T, y0: T, max_iterations: u32) -> Vec<(T, T)>
    where T: Add<Output=T> + for<'a> Add<&'a T, Output=T>
           + Mul<Output=T> + for<'a> Mul<&'a T, Output=T>
           + Neg + Sub<Output=T> + From<f64>
           + Clone + PartialOrd + Display,
          for <'a> &'a T: Mul<Output=T>
{
    let x = T::from(0.0f64);
    let y = T::from(0.0f64);

    iterate_all_start::<T>(x0, y0, x, y, max_iterations)
}

pub fn delta(d: Complex64, x_n: Complex64, input: [Complex64; 3]) -> (Complex64, [Complex64; 3]) {
    let a_n = input[0];
    let b_n = input[1];
    let c_n = input[2];

    let a_n1 = 2f64 * a_n * x_n + 1f64;
    let b_n1 = 2f64 * b_n * x_n + a_n * a_n;
    let c_n1 = 2f64 * c_n * x_n + a_n * b_n;
    let x_n1 = a_n1 * d + b_n1 * d * d + c_n1 * d * d * d;

    (x_n1, [a_n1, b_n1, c_n1])
}

pub fn iterate_delta<'a>(reference: &'a Vec<(Mpfr, Mpfr)>, d_x: f64, d_y: f64) -> Vec<(f64, f64)> {
    pub fn mpfr_to_complex<'a, 'b>(x: &'a Mpfr, y: &'b Mpfr) -> Complex64 {
        let x_: f64 = x.into();
        let y_: f64 = y.into();

        Complex64::new(x_, y_)
    }

    let a0 = Complex64::new(1f64, 0f64);
    let b0 = Complex64::new(0f64, 0f64);
    let c0 = Complex64::new(0f64, 0f64);
    let mut input = [a0, b0, c0];
    let d = Complex64::new(d_x, d_y);
    let x0 = mpfr_to_complex(&reference[0].0, &reference[0].1);
    let mut x = x0;

    let mut v = Vec::<(f64, f64)>::new();

    v.push((x.re, x.im));

    let mut i = 1;

    while i < reference.len() && x.norm() <= 2f64 {
        let x_n = mpfr_to_complex(&reference[i].0, &reference[i].1);
        let (delta_n, input_new) = delta(d, x_n, input);

        x = x_n + delta_n;
//        println("X: {}", x - mpfr_to_complex(&reference[i + 1].0, &reference[i].1));
        input = input_new;

        v.push((x.re, x.im));
        i = i + 1;
    }

    v
}

pub fn calculate_all_mpfr(canvas_size: CanvasSize, max_iterations: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    (0..canvas_size.pixel_count())
        .into_par_iter()
        .weight_max()
        .map(|i| canvas_size.coordinates(canvas_size.idx_to_coord(i as usize)))
        .map(|c| {
            iterate::<Mpfr>((c[0].clone()), (c[1].clone()), max_iterations)
                .unwrap_or(max_iterations)
        })
        .collect_into(&mut v);
    v
}

pub fn calculate_all_float(canvas_size: CanvasSize, max_iterations: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    (0..canvas_size.pixel_count())
        .into_par_iter()
        .weight_max()
        .map(|i| canvas_size.coordinates(canvas_size.idx_to_coord(i as usize)))
        .map(|c| {
            iterate::<f64>(Into::<f64>::into(&c[0]), Into::<f64>::into(&c[1]), max_iterations)
                .unwrap_or(max_iterations)
        })
        .collect_into(&mut v);
    v
}

pub fn calculate_all_delta(canvas_size: CanvasSize, max_iterations: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    let reference_coord = canvas_size.coordinates(canvas_size.idx_to_coord(0));
    let reference = iterate_all::<Mpfr>(reference_coord[0].clone(), reference_coord[1].clone(), max_iterations);
    println!("Ref {} {}", reference.len(), max_iterations - reference.len() as u32);
    (0..canvas_size.pixel_count())
        .into_par_iter()
        .weight_max()
        .map(|i| canvas_size.coordinates(canvas_size.idx_to_coord(i as usize)))
        .map(|c| {
            let d_x: f64 = (&(&c[0] - &reference_coord[0])).into();
            let d_y: f64 = (&(&c[1] - &reference_coord[1])).into();

            let v = iterate_delta(&reference, d_x, d_y);

            if reference.len() - v.len() == 0 {
               println!("{} {}", v.len(), reference.len() - v.len());
            }

            let mut i = v.len();

            if reference.len() - v.len() <= 0 {
                println!("Additional calc: {}", reference.len() - v.len());

                let last_x = Into::<Mpfr>::into(v[v.len() - 1].0);
                let last_y = Into::<Mpfr>::into(v[v.len() - 1].1);

                let remaining = (max_iterations as usize - v.len()) as u32;

                let v_mpfr = iterate_all_start::<Mpfr>(c[0].clone(), c[1].clone(), last_x, last_y, remaining);
                i += v_mpfr.len();
                println!("After: {} {} {}", i, v_mpfr.len(), remaining);
            }

            v.len() as u32

//            iterate::<Mpfr>((c[0].clone()), (c[1].clone()), max_iterations)
//                .unwrap_or(max_iterations)
        })
        .collect_into(&mut v);
    v
}

fn color_from_iteration(iterations: u32, max_iterations: u32) -> [u8; 3] {
    const N_COLORS: u32 = 256u32;
    const BLACK: [u8; 3] = [0u8, 0u8, 0u8];

    let grad = Gradient::new(vec![Hsv::new(RgbHue::from(0f32), 1.0, 1.0),
                                  Hsv::new(RgbHue::from(180f32), 1.0, 1.0),
                                  Hsv::new(RgbHue::from(360f32), 1.0, 1.0)]);

    if iterations == max_iterations {
        BLACK
    } else {
        grad.get((iterations % N_COLORS) as f32 / N_COLORS as f32).into_rgb().to_pixel()
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
    use rust_mpfr::mpfr::*;

    #[test]
    fn new_canvas_size() {
        let c = CanvasSize::new(900, 600, mpfr!(1.0), mpfr!(-1.0), mpfr!(-2.0), mpfr!(1.0));

        assert_eq!(c.top, mpfr!(1.0));
        assert_eq!(c.bottom, mpfr!(-1.0));
        assert_eq!(c.left, mpfr!(-2.0));
        assert_eq!(c.right, mpfr!(1.0));
        assert_eq!(c.height(), mpfr!(2.0));
        assert_eq!(c.width(), mpfr!(3.0));
    }

    #[test]
    fn new_canvas_size_from_center() {
        let c = CanvasSize::new_from_center(900, 600, [mpfr!(-0.5), mpfr!(0.0)], mpfr!(1.0));

        assert_eq!(c.top, mpfr!(1.0));
        assert_eq!(c.bottom, mpfr!(-1.0));
        assert_eq!(c.left, mpfr!(-2.0));
        assert_eq!(c.right, mpfr!(1.0));
        assert_eq!(c.height(), mpfr!(2.0));
        assert_eq!(c.width(), mpfr!(3.0));
    }

    #[test]
    fn new_canvas_size_from_center_and_zoom() {
        let c = CanvasSize::new_from_center(900, 600, [mpfr!(-0.5), mpfr!(0.0)], mpfr!(2.0));

        assert_eq!(c.top, mpfr!(0.5));
        assert_eq!(c.bottom, mpfr!(-0.5));
        assert_eq!(c.left, mpfr!(-1.25));
        assert_eq!(c.right, mpfr!(0.25));
        assert_eq!(c.height(), mpfr!(1.0));
        assert_eq!(c.width(), mpfr!(1.5));
    }

    #[test]
    fn test_center() {
        let c = CanvasSize::new_from_center(900, 600, [mpfr!(-0.5), mpfr!(0.0)], mpfr!(1.0));

        assert_eq!(c.center(), [mpfr!(-0.5), mpfr!(0.0)]);
    }

    #[test]
    fn test_width_and_height() {
        let c = CanvasSize::new_from_center(900, 600, [mpfr!(-0.5), mpfr!(0.0)], mpfr!(1.0));

        assert_eq!(c.width(), mpfr!(3.0));
        assert_eq!(c.height(), mpfr!(2.0));
    }

    #[test]
    fn test_to_zoom() {
        let c = CanvasSize::new_from_center(900, 600, [mpfr!(-0.5), mpfr!(0.0)], mpfr!(1.0));
        let zoomed = c.zoom(mpfr!(2.0));

        assert_eq!(zoomed.center(), [mpfr!(-0.5), mpfr!(0.0)]);
        assert_eq!(zoomed.height(), mpfr!(1.0));
        assert_eq!(zoomed.width(), mpfr!(1.5));

        let zoomed_again = zoomed.zoom(mpfr!(2.0));
        assert_eq!(zoomed_again.center(), [mpfr!(-0.5), mpfr!(0.0)]);
        assert_eq!(zoomed_again.height(), mpfr!(0.5));
        assert_eq!(zoomed_again.width(), mpfr!(0.75));
    }

    #[test]
    fn test_move_center() {
        let c = CanvasSize::new_from_center(900, 600, [mpfr!(-0.5), mpfr!(0.0)], mpfr!(1.0));

        assert_eq!(c.move_center([mpfr!(0.0), mpfr!(-1.0)]).center(),
                   [mpfr!(0.0), mpfr!(-1.0)]);
    }

    #[test]
    fn test_pixel_count() {
        let c = CanvasSize::new_from_center(2, 3, [mpfr!(-0.5), mpfr!(0.0)], mpfr!(1.0));

        assert_eq!(c.pixel_count(), 6);
    }

    #[test]
    fn test_iterate_all() {
        let c = CanvasSize::new_from_center(2, 3, [mpfr!(-0.5), mpfr!(0.0)], mpfr!(1.0));
        let x = c.center()[0].clone();
        let y = c.center()[1].clone();
        let v = iterate_all::<Mpfr>(x, y, 10);

        assert_eq!(v.len(), 10);
    }

    #[test]
    fn test_iterate_all_prec() {
        let prec = 128;
        let c = CanvasSize::new_from_center(2, 3, [mpfr!(-0.5), mpfr!(0.0)], mpfr!(1.0))
            .set_prec(prec);
        let x = c.center()[0].clone();
        let y = c.center()[1].clone();
        let v = iterate_all::<Mpfr>(x, y, 10);

        assert_eq!(v[9].0.get_prec(), prec);
        assert_eq!(v[9].1.get_prec(), prec);
    }
}
