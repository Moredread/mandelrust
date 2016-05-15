use carboxyl_window::{Context, Event};
use carboxyl_window::Event::Press;
use input::Button::Keyboard;
use input::Key;
use image::RgbImage;

#[derive(Clone)]
pub struct Action {

}

pub fn intent(_: Context, event: Event) -> Option<Action> {
    None
}

pub type State = RgbImage;
pub type View = RgbImage;

pub fn init(init: RgbImage) -> RgbImage {
    init.clone()
}

pub fn update(current: RgbImage, action: Action) -> State {
    current
}

pub fn view(context: Context, state: RgbImage) -> View {
    let (width, height) = context.window.size;
    state
}
