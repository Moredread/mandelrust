use benzene::{Application, Component};
use carboxyl_window::{Context, Event};
use carboxyl_window::Event::Press;
use input::Button::Keyboard;
use input::Key;

pub struct App {
}

impl App {
    pub fn new() -> App {
        App {}
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


type State = ();

impl Component for App {
    type Context = Context;
    type Action = Action;
    type State = ();
    type View = ();

    fn init(&self) -> State {}

    fn update(&self, current: State, action: Action) -> State {
        current
    }

    fn view(&self, context: Context, state: State) -> () {
        let (width, height) = context.window.size;
    }
}
