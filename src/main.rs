extern crate graphics;
extern crate glium;
extern crate glium_graphics;
extern crate shader_version;
extern crate piston;
extern crate glutin_window;
#[macro_use(lift)]
extern crate carboxyl;
extern crate carboxyl_window;
extern crate benzene;
#[macro_use]
extern crate timeit;
extern crate rust_mpfr;
extern crate pbr;
extern crate image;
extern crate palette;
extern crate input;

mod driver;
mod mandelbrot;
mod app;

use std::fs::File;
use std::path::Path;
use driver::Driver2d;
use mandelbrot::{CanvasSize, calculate_all, make_image};
use piston::window::WindowSettings;
use app::App;
use benzene::Driver;

fn settings() -> WindowSettings {
    WindowSettings::new("Mandelrust", (900, 600))
}

fn main() {
    let c = CanvasSize::new_from_center(900, 600, [-0.5, 0.0], 1.0);
    let max = 256u32;

    let v = calculate_all(c, max);
    let imgbuf = make_image(v, c, max);
    //let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    //let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);

    let mut driver2d = Driver2d::new(settings());
    let output = benzene::start(App::new(imgbuf), driver2d.output());
    driver2d.run(output);
}
