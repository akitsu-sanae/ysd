use termion::event::{Event, Key};

use super::{edit_worker::EditWorker, EventWorker};
use state::State;

#[derive(Debug)]
pub struct CommandWorker {
    input: String,
}

impl CommandWorker {
    fn apply_buildin_command(&mut self, state: &mut State) -> Option<Box<dyn EventWorker>> {
        let mut inputs = self.input.split_whitespace();
        if let Some(command) = inputs.next() {
            let inputs: Vec<&str> = inputs.collect();
            match (command, inputs.as_slice()) {
                (":go", [dir, distance]) => {
                    if let (Ok(dir), Ok(distance)) = (dir.parse(), distance.parse()) {
                        state.current_buffer_mut().cursor.go(dir, distance);
                    } else {
                        state.update_message("usage :go <direction> <distance>");
                    }
                }
                (":edit", []) => return Some(Box::new(EditWorker::default())),
                (":quit", []) => state.is_quit = true,
                _ => {
                    // FIXME: do not use `{:?}`
                    state.update_message(
                        format!("invalid args for command {}: {:?}", command, inputs).as_str(),
                    );
                }
            }
        }
        None
    }
}

impl Default for CommandWorker {
    fn default() -> Self {
        CommandWorker {
            input: String::new(),
        }
    }
}

impl EventWorker for CommandWorker {
    fn update(&mut self, state: &mut State, e: Event) -> Option<Box<dyn EventWorker>> {
        match e {
            Event::Key(Key::Char('\n')) => {
                let next_worker = self.apply_buildin_command(state);
                self.input = String::new();
                return next_worker;
            }
            Event::Key(Key::Backspace) => {
                self.input.pop();
            }
            Event::Key(Key::Char(c)) => {
                self.input.push(c);
                state.update_message(self.input.as_str());
            }
            _ => (),
        }
        None
    }
}
