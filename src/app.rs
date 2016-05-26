use carboxyl_window::{Context, Event};
use carboxyl_window::Event::Press;
use input::Button::{Mouse, Keyboard};
use input::{MouseButton, Key};
use image::RgbImage;
use mandelbrot::*;

#[derive(Clone, Copy)]
pub enum Action {
    ZoomIn([f64; 2]),
    ZoomOut,
    MaxIterationsUp,
    MaxIterationsDown
}

pub fn intent(context: Context, event: Event) -> Option<Action> {
    match event {
        Press(Mouse(MouseButton::Left)) => {
            Some(Action::ZoomIn([context.cursor.position.0, context.cursor.position.1]))
        }
        Press(Mouse(MouseButton::Right)) => Some(Action::ZoomOut),
        Press(Keyboard(Key::PageUp)) => Some(Action::MaxIterationsUp),
        Press(Keyboard(Key::PageDown)) => Some(Action::MaxIterationsDown),
        _ => None,
    }
}

#[derive(Clone)]
pub struct State {
    image: RgbImage,
    canvas: CanvasSize,
    max: u32,
}

impl State {
    fn calc(canvas: CanvasSize, max: u32) -> State {
        let v = calculate_all(canvas.clone(), max);
        let imgbuf = make_image(v, canvas.clone(), max);

        State {
            image: imgbuf,
            canvas: canvas.clone(),
            max: max,
        }
    }
}

pub type View = RgbImage;

pub fn init(canvas: CanvasSize, max: u32) -> State {
    State::calc(canvas, max)
}

pub fn update(current: State, action: Action) -> State {
    match action {
        Action::ZoomIn(l) => {
            State::calc(current.canvas.move_center_to_pixel(l).zoom(mpfr!(8.0)),
                        current.max)
        }
        Action::ZoomOut => State::calc(current.canvas.zoom(mpfr!(1.0) / 8.0), current.max),
        Action::MaxIterationsUp => {
            println!("Max. iterations: {}", current.max + 1000);
            State::calc(current.canvas, current.max + 1000)
        },
        Action::MaxIterationsDown => {
            if current.max > 1000 {
                println!("Max. iterations: {}", current.max - 1000);
                State::calc(current.canvas, current.max - 1000)
            } else { current }
        }
    }
}

pub fn view(_: Context, state: State) -> View {
    state.image
}
