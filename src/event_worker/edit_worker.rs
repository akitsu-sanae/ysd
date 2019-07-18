use termion::event::{Event, Key};

use super::EventWorker;
use crate::state::State;
use util::Direction;

#[derive(Debug)]
pub struct EditWorker {}

impl Default for EditWorker {
    fn default() -> Self {
        EditWorker {}
    }
}

impl EventWorker for EditWorker {
    fn update(&self, state: &mut State, e: Event) {
        match e {
            Event::Key(Key::Char(c)) => {
                let x = state.cursor.x;
                let y = state.cursor.y;
                state.current_buffer_mut().insert((x, y), c);
                state.cursor.go(Direction::Right);
            }
            Event::Key(Key::Esc) => state.is_quit = true,
            _ => (),
        }
    }
}
