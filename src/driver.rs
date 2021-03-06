use benzene::{Driver, Communication};
use std::cell::RefCell;
use std::rc::Rc;
use glium_graphics::{Glium2d, GliumGraphics, GliumWindow, Texture, TextureSettings};
use glutin_window::GlutinWindow;
use carboxyl_window::{RunnableWindow, StreamingWindow, SourceWindow, Context, Event};
use shader_version::OpenGL;
use piston::window::WindowSettings;
use image::{RgbImage, ConvertBuffer};
use graphics::{color, image, clear};
use graphics;
use graphics::Transformed;

pub struct Driver2d {
    glutin_window: Rc<RefCell<GlutinWindow>>,
    source_window: SourceWindow<Rc<RefCell<GlutinWindow>>>,
}

impl Driver2d {
    pub fn new(settings: WindowSettings) -> Driver2d {
        let glutin_window = Rc::new(RefCell::new(GlutinWindow::new(settings).ok().unwrap()));
        let source_window = SourceWindow::new(glutin_window.clone());
        Driver2d {
            glutin_window: glutin_window,
            source_window: source_window,
        }
    }
}

impl Driver<Communication<RgbImage, ()>> for Driver2d {
    type Output = Communication<Context, Event>;

    fn output(&self) -> Communication<Context, Event> {
        Communication {
            context: self.source_window.context(),
            events: self.source_window.events(),
        }
    }

    fn run(&mut self, input: Communication<RgbImage, ()>) {
        const GLVERSION: OpenGL = OpenGL::V2_1;
        let mut glium_window = GliumWindow::new(&self.glutin_window).ok().unwrap();
        let mut backend_sys = Glium2d::new(GLVERSION, &glium_window);

        let canvas = lift!(|context, view| (context.window.size, view),
                           &self.source_window.context(),
                           &input.context);


        self.source_window.run_with(120.0, || {
            let ((w, h), element) = canvas.sample();
            let mut target = glium_window.draw();
            {
                let transform = graphics::math::abs_transform(w as f64, h as f64);
                let mut backend = GliumGraphics::new(&mut backend_sys, &mut target);
                let scale_factor = w as f64 / element.dimensions().0 as f64;
                let texture = Texture::from_image(&mut glium_window,
                                                  &element.convert(),
                                                  &TextureSettings::new())
                    .unwrap();
                clear(color::BLACK, &mut backend);
                image(&texture,
                      transform.trans(0.0, 0.0).scale(scale_factor, scale_factor),
                      &mut backend);
            }
            target.finish().unwrap();
        });
    }
}
