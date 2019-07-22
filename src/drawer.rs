use std::io::{stdout, Stdout, Write};

use termion::clear;
use termion::cursor::{Goto, Restore, Save};
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use frame::Frame;
use layout::Layout;
use state::State;
use util::Direction;

pub struct Drawer {
    out: MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>,
}

fn splited_frames(dir: &Direction, frame: &Frame) -> (Frame, Frame) {
    match dir {
        Direction::Up => (
            Frame {
                x: 0,
                y: 0,
                width: frame.width,
                height: 1,
            },
            Frame {
                x: 0,
                y: 1,
                width: frame.width,
                height: frame.height - 1,
            },
        ),
        Direction::Down => (
            Frame {
                x: 0,
                y: frame.height - 1,
                width: frame.width,
                height: 1,
            },
            Frame {
                x: 0,
                y: 0,
                width: frame.width,
                height: frame.height - 1,
            },
        ),
        Direction::Left => unimplemented!(),
        Direction::Right => unimplemented!(),
    }
}

impl Drawer {
    pub fn draw(&mut self, state: &State) {
        // `+1` means convertion from 0-origin position to 1-origin position

        write!(self.out, "{}", clear::All).unwrap();

        fn draw_layout(out: &mut impl Write, state: &State, layout: &Layout, frame: &Frame) {
            use self::Layout::*;
            match layout {
                Buffer(name) => {
                    let buf = state
                        .buffers
                        .get(name)
                        .expect(format!("internal error: unknown buffer name {}", name).as_str());
                    for (i, line) in buf.data.iter().enumerate() {
                        if i as i32 >= frame.height {
                            break;
                        }
                        write!(
                            out,
                            "{}{}",
                            Goto(frame.x as u16 + 1, frame.y as u16 + 1 + i as u16),
                            line
                        )
                        .unwrap();
                    }
                    if name == &state.current_buffer_name {
                        let x = frame.x + buf.cursor.x;
                        let y = frame.y + buf.cursor.y;
                        write!(out, "{}{}", Goto(x as u16 + 1, y as u16 + 1), Save).unwrap();
                    }
                }
                Lined(dir, line, body) => {
                    let (line_frame, body_frame) = splited_frames(dir, frame);
                    draw_layout(out, state, line, &line_frame);
                    draw_layout(out, state, body, &body_frame);
                }
            }
        }

        draw_layout(&mut self.out, state, &state.layout, &Frame::screen());
        write!(self.out, "{}", Restore).unwrap();
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
