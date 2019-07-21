use std::io::{stdout, Stdout, Write};

use termion::clear;
use termion::cursor::Goto;
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use frame::Frame;
use layout::Layout;
use state::State;

pub struct Drawer {
    out: MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>,
}

impl Drawer {
    pub fn draw(&mut self, state: &State) {
        // `+1` means convertion from 0-origin position to 1-origin position

        write!(self.out, "{}", clear::All).unwrap();
        let mut draw_layout = |layout: &Layout, frame: &Frame| {
            use self::Layout::*;
            match layout {
                Buffer(name) => {
                    let buf = state
                        .buffers
                        .get(name)
                        .expect(format!("internal error: unknown buffer name {}", name).as_str());
                    for (i, line) in buf.data.iter().enumerate() {
                        write!(
                            self.out,
                            "{}{}",
                            Goto(frame.x as u16 + 1, frame.y as u16 + 1 + i as u16),
                            line
                        )
                        .unwrap();
                    }
                }
            }
        };

        draw_layout(&state.layout, &Frame::screen());

        let (_, height) = ::termion::terminal_size().unwrap();
        write!(
            self.out,
            "{}{}{}",
            Goto(1, height),
            clear::CurrentLine,
            state.message
        )
        .unwrap();
        // write!(self.out, "{}", Goto(state.cursor.x + 1, state.cursor.y + 1)).unwrap();
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
