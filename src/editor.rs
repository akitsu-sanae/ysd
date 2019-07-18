use termion::event::Event;

use crate::drawer::Drawer;
use crate::event_worker::{edit_worker::EditWorker, EventWorker};
use crate::state::State;

pub struct Editor {
    pub event_worker: Box<dyn EventWorker>,
    pub state: State,
    pub drawer: Drawer,
}

impl Editor {
    pub fn from_file(filename: &str) -> Self {
        let event_worker = Box::new(EditWorker::default());
        let drawer = Drawer::default();
        let state = State::from_file(filename);
        Editor {
            event_worker: event_worker,
            state: state,
            drawer: drawer,
        }
    }

    pub fn update(&mut self, e: Event) {
        self.event_worker.update(&mut self.state, e)
    }

    pub fn draw(&mut self) {
        self.drawer.draw(&self.state);
    }
}
