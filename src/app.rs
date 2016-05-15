use carboxyl_window::{Context, Event};
use carboxyl_window::Event::Press;
use input::Button::Mouse;
use input::Key;
use image::RgbImage;
use mandelbrot::*;

#[derive(Clone)]
pub struct Action {
    zoom_location: [f64; 2],
}

pub fn intent(context: Context, event: Event) -> Option<Action> {
    match event {
        Press(Mouse(Left)) => { Some(Action { zoom_location: [context.cursor.position.0, context.cursor.position.1] }) },
        _ => None
    }
}

#[derive(Clone)]
pub struct State {
    image: RgbImage,
    canvas: CanvasSize,
    max: u32
}

impl State {
    fn calc(canvas: CanvasSize, max: u32) -> State {
       let v = calculate_all(canvas, max);
       let imgbuf = make_image(v, canvas, max);

        State {
            image: imgbuf,
            canvas: canvas,
            max: max,
        }
    }
}

pub type View = RgbImage;

pub fn init(canvas: CanvasSize, max: u32) -> State {
    State::calc(canvas, max)
}

pub fn update(current: State, action: Action) -> State {
    State::calc(current.canvas.zoom(1.5).move_center_to_pixel(action.zoom_location), current.max)
}

pub fn view(context: Context, state: State) -> View {
    let (width, height) = context.window.size;
    state.image
}
