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
use mandelbrot::{CanvasSize};
use piston::window::WindowSettings;
use benzene::{Driver, Component, interpret, start};


fn settings() -> WindowSettings {
    WindowSettings::new("Mandelrust", (900, 600))
}

fn main() {
    let c = CanvasSize::new_from_center(900, 600, [-0.5, 0.0], 1.0);
    let max = 25600u32;

    let mut driver2d = Driver2d::new(settings());

    let output = start(
        Component {
            init: app::init(c, max),
            update: app::update,
            view: app::view,
            effect: |_, _| None
        },
        interpret(driver2d.output(), app::intent)
        );

    driver2d.run(output);
}
