use termion::color::{Bg, Cyan, Reset};
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
    fn mode(&self) -> String {
        format!("{} Edit {}", Bg(Cyan), Bg(Reset))
    }

    fn update(&mut self, state: &mut State, e: Event) -> Option<Box<dyn EventWorker>> {
        match e {
            Event::Key(Key::Char('\n')) => {
                let cursor = state.current_panel().cursor.clone();
                let buffer_id = state.current_panel().buffer_id;
                state
                    .buffers
                    .get_mut(&buffer_id)
                    .expect("internal error: missing current buffer")
                    .insert_line_at_cursor(&cursor);
                let ref mut cursor = state.current_panel_mut().cursor;
                cursor.x = 0;
                cursor.go(Direction::Down, 1);
            }
            Event::Key(Key::Backspace) => {
                let cursor = state.current_panel().cursor.clone();
                state.current_buffer_mut().erase_at_cursor(&cursor);
                state.current_panel_mut().cursor.go(Direction::Left, 1);
            }
            Event::Key(Key::Char(c)) => {
                let cursor = state.current_panel().cursor.clone();
                let buffer_id = state.current_panel().buffer_id;
                state
                    .buffers
                    .get_mut(&buffer_id)
                    .expect("internal error: missing current buffer")
                    .insert_at_cursor(c, &cursor);
                state.current_panel_mut().cursor.go(Direction::Right, 1);
            }
            Event::Key(Key::Esc) => return Some(Box::new(CommandWorker::default())),
            _ => (),
        }
        None
    }
}
