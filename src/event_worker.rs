use crate::state::State;
use termion::event::Event;

pub mod command_worker;
pub mod edit_worker;

pub trait EventWorker {
    fn update(&mut self, state: &mut State, e: Event) -> Option<Box<dyn EventWorker>>;
}
