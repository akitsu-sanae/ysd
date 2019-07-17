use termion::event::{Event, Key};

use crate::state::State;
use super::EventWorker;

#[derive(Debug)]
pub struct EditWorker {
}

impl Default for EditWorker {
    fn default() -> Self {
        EditWorker {
        }
    }
}

impl EventWorker for EditWorker {
    fn update(&self, state: &mut State, e: Event) {
        match e {
            Event::Key(Key::Char('q')) => state.is_quit = true,
            _ => (),
        }
    }
}

