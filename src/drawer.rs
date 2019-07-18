use std::io::{stdout, Stdout, Write};

use termion::cursor::Goto;
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use crate::state::State;

pub struct Drawer {
    out: MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>,
}

impl Drawer {
    pub fn draw(&mut self, state: &State) {
        for (ref buf_name, ref frame) in &state.frames {
            let buf = state
                .buffers
                .get(buf_name)
                .expect(format!("internal error: unknown buffer name {}", buf_name).as_str());
            for (i, line) in buf.data.iter().enumerate() {
                write!(self.out, "{}{}", Goto(frame.x + 1, frame.y + 1 + i as u16), line).unwrap();
            }
        }
        write!(self.out, "{}", Goto(state.cursor.x + 1, state.cursor.y + 1)).unwrap();
        self.out.flush().unwrap();
    }
}

impl Default for Drawer {
    fn default() -> Self {
        let out = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        let mut out = MouseTerminal::from(out);
        out.flush().unwrap();
        Drawer { out: out }
    }
}
