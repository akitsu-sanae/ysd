use crate::state::State;
use termion::event::Event;

pub mod edit_worker;

pub trait EventWorker {
    fn update(&self, state: &mut State, e: Event);
}
