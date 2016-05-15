use carboxyl_window::{Context, Event};
use carboxyl_window::Event::Press;
use input::Button::Keyboard;
use input::Key;
use image::RgbImage;
use mandelbrot::*;

#[derive(Clone)]
pub struct Action {

}

pub fn intent(_: Context, event: Event) -> Option<Action> {
    None
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
    current
}

pub fn view(context: Context, state: State) -> View {
    let (width, height) = context.window.size;
    state.image
}
