use termion::event::Event;
use crate::state::State;

pub mod edit_worker;

pub trait EventWorker {
    fn update(&self, state: &mut State, e: Event);
}

