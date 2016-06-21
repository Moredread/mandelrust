use carboxyl_window::{Context, Event};
use carboxyl_window::Event::Press;
use input::Button::{Mouse, Keyboard};
use input::{MouseButton, Key};
use image::RgbImage;
use mandelbrot::*;

#[derive(Clone, Copy)]
pub enum Action {
    ZoomIn([f64; 2], (u32, u32)),
    ZoomOut,
    MaxIterationsUp,
    MaxIterationsDown,
    PrecisionUp,
    PrecisionDown,
}

pub fn intent(context: Context, event: Event) -> Option<Action> {
    match event {
        Press(Mouse(MouseButton::Left)) => {
            Some(Action::ZoomIn([context.cursor.position.0, context.cursor.position.1],
                                context.window.size))
        }
        Press(Mouse(MouseButton::Right)) => Some(Action::ZoomOut),
        Press(Keyboard(Key::PageUp)) => Some(Action::MaxIterationsUp),
        Press(Keyboard(Key::PageDown)) => Some(Action::MaxIterationsDown),
        Press(Keyboard(Key::Home)) => Some(Action::PrecisionUp),
        Press(Keyboard(Key::End)) => Some(Action::PrecisionDown),
        _ => None,
    }
}

#[derive(Clone)]
pub struct State {
    image: RgbImage,
    canvas: CanvasSize,
    max: u32,
    calculator: Calculator,
}

#[derive(Clone)]
enum Calculator {
    MPFR,
}

impl State {
    fn calc(canvas: CanvasSize, max: u32, calc: Calculator) -> State {
        let v = match calc {
            Calculator::MPFR => calculate_all(canvas.clone(), max),
        };
        let imgbuf = make_image(v, canvas.clone(), max);

        State {
            image: imgbuf,
            canvas: canvas.clone(),
            max: max,
            calculator: calc,
        }
    }
}

pub type View = RgbImage;

pub fn init(canvas: CanvasSize, max: u32) -> State {
    State::calc(canvas, max, Calculator::MPFR)
}

pub fn update(current: State, action: Action) -> State {
    match action {
        Action::ZoomIn(loc, win_size) => {
            let (x, _) = win_size;
            let scale_factor = x as f64 / current.image.dimensions().0 as f64;
            let scaled_loc: [f64; 2] = [loc[0] / scale_factor, loc[1] / scale_factor];
            State::calc(current.canvas.move_center_to_pixel(scaled_loc).zoom(mpfr!(8.0)),
                        current.max, current.calculator)
        }
        Action::ZoomOut => State::calc(current.canvas.zoom(mpfr!(1.0) / 8.0), current.max, current.calculator),
        Action::MaxIterationsUp => {
            println!("Max. iterations: {}", current.max + 1000);
            State::calc(current.canvas, current.max + 1000, current.calculator)
        }
        Action::MaxIterationsDown => {
            if current.max > 1000 {
                println!("Max. iterations: {}", current.max - 1000);
                State::calc(current.canvas, current.max - 1000, current.calculator)
            } else {
                current
            }
        }
        Action::PrecisionUp => {
            println!("{}", current.canvas.get_prec() * 2);
            let new = current.canvas.set_prec(current.canvas.get_prec() * 2);
            let a = State::calc(new, current.max, current.calculator);
            println!("a: {}", a.canvas.center()[0].get_prec());
            println!("b: {}", a.canvas.coordinates([0, 0])[0].get_prec());
            a
        }
        Action::PrecisionDown => {
            println!("{}", current.canvas.get_prec() / 2);
            let new = current.canvas.set_prec(current.canvas.get_prec() / 2);
            State::calc(new, current.max, current.calculator)
        }
    }
}

pub fn view(_: Context, state: State) -> View {
    state.image
}
