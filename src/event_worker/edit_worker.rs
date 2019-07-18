use termion::event::{Event, Key};

use super::{command_worker::CommandWorker, EventWorker};
use state::State;
use util::Direction;

#[derive(Debug)]
pub struct EditWorker {}

impl Default for EditWorker {
    fn default() -> Self {
        EditWorker {}
    }
}

impl EventWorker for EditWorker {
    fn update(&mut self, state: &mut State, e: Event) -> Option<Box<dyn EventWorker>> {
        match e {
            Event::Key(Key::Char(c)) => {
                let x = state.cursor.x;
                let y = state.cursor.y;
                state.current_buffer_mut().insert((x, y), c);
                state.cursor.go(Direction::Right, 1);
            }
            Event::Key(Key::Esc) => return Some(Box::new(CommandWorker::default())),
            _ => (),
        }
        None
    }
}
