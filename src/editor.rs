use termion::event::Event;

use crate::drawer::Drawer;
use crate::event_worker::{command_worker::CommandWorker, EventWorker};
use crate::state::State;

pub struct Editor {
    pub event_worker: Box<dyn EventWorker>,
    pub state: State,
    pub drawer: Drawer,
}

impl Editor {
    pub fn from_file(filename: &str) -> Self {
        let event_worker = Box::new(CommandWorker::default());
        let drawer = Drawer::default();
        let mut state = State::from_file(filename);
        state.update_mode(event_worker.mode());
        Editor {
            event_worker: event_worker,
            state: state,
            drawer: drawer,
        }
    }

    pub fn update(&mut self, e: Event) {
        if let Some(next_worker) = self.event_worker.update(&mut self.state, e) {
            self.state.update_mode(next_worker.mode());
            self.event_worker = next_worker;
        }
        self.state.clamp_cursor();
    }

    pub fn draw(&mut self) {
        self.drawer.draw(&self.state);
    }
}
