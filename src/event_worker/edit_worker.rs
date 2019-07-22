use termion::color::{Bg, Cyan, Reset};
use termion::event::{Event, Key};

use super::{command_worker::CommandWorker, EventWorker};
use state::State;

#[derive(Debug)]
pub struct EditWorker {}

impl Default for EditWorker {
    fn default() -> Self {
        EditWorker {}
    }
}

impl EventWorker for EditWorker {
    fn mode(&self) -> String {
        format!("{} Edit {}", Bg(Cyan), Bg(Reset))
    }

    fn update(&mut self, state: &mut State, e: Event) -> Option<Box<dyn EventWorker>> {
        match e {
            Event::Key(Key::Char('\n')) => state.current_buffer_mut().insert_line_at_cursor(),
            Event::Key(Key::Char(c)) => state.current_buffer_mut().insert_at_cursor(c),
            Event::Key(Key::Esc) => return Some(Box::new(CommandWorker::default())),
            _ => (),
        }
        None
    }
}
