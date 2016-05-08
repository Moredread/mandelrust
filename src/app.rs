use benzene::{Application, Component};
use carboxyl_window::{Context, Event};
use carboxyl_window::Event::Press;
use input::Button::Keyboard;
use input::Key;
use image::RgbImage;

pub type Image = RgbImage;

pub struct App {
    initial_image: Image,
}

impl App {
    pub fn new(image: Image) -> App {
        App { initial_image: image }
    }
}

#[derive(Clone)]
pub struct Action {
}

impl Application for App {
    type Event = Event;

    fn intent(&self, _: Context, event: Event) -> Option<Action> {
        None
    }
}

pub type State = Image;

impl Component for App {
    type Context = Context;
    type Action = Action;
    type State = State;
    type View = Image;

    fn init(&self) -> State {
        self.initial_image.clone()
    }

    fn update(&self, current: State, action: Action) -> State {
        current
    }

    fn view(&self, context: Context, state: State) -> Image {
        let (width, height) = context.window.size;
        state
    }
}
