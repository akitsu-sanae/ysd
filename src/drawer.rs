use std::convert::TryInto;
use std::io::{stdout, Stdout, Write};

use termion::clear;
use termion::cursor::{Goto, Restore, Save};
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use buffer::Buffer;
use frame::Frame;
use layout::Layout;
use state::State;
use util::Direction;

pub struct Drawer {
    out: MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>,
}

fn splited_frames(dir: &Direction, line_width: i32, frame: &Frame) -> (Frame, Frame) {
    match dir {
        Direction::Up => (
            Frame {
                x: frame.x,
                y: frame.y,
                width: frame.width,
                height: line_width,
            },
            Frame {
                x: frame.x,
                y: frame.y + line_width,
                width: frame.width,
                height: frame.height - line_width,
            },
        ),
        Direction::Down => (
            Frame {
                x: frame.x,
                y: frame.y + frame.height - line_width,
                width: frame.width,
                height: line_width,
            },
            Frame {
                x: frame.x,
                y: frame.y,
                width: frame.width,
                height: frame.height - line_width,
            },
        ),
        Direction::Left => (
            Frame {
                x: frame.x,
                y: frame.y,
                width: line_width,
                height: frame.height,
            },
            Frame {
                x: frame.x + line_width,
                y: frame.y,
                width: frame.width - line_width,
                height: frame.height,
            },
        ),
        Direction::Right => (
            Frame {
                x: frame.x + frame.width - line_width,
                y: frame.y,
                width: line_width,
                height: frame.height,
            },
            Frame {
                x: frame.x,
                y: frame.y,
                width: frame.width - line_width,
                height: frame.height,
            },
        ),
    }
}

fn draw_buffer(out: &mut impl Write, buffer: &Buffer, frame: &Frame) {
    // `+1` means convertion from 0-origin position to 1-origin position
    let frame_x: u16 = (frame.x + 1).try_into().unwrap();
    let frame_y: u16 = (frame.y + 1).try_into().unwrap();

    let ref cursor = buffer.cursor;

    let top_line = if cursor.y < frame.height / 2 {
        0
    } else if cursor.y + frame.height / 2 > buffer.data.len() as i32 {
        buffer.data.len() as i32 - frame.height
    } else {
        cursor.y - frame.height / 2
    };

    for i in { 0..frame.height } {
        write!(
            out,
            "{}{}",
            Goto(frame_x, frame_y + i as u16),
            if i + top_line < buffer.data.len() as i32 {
                buffer.data[(top_line + i) as usize].as_str()
            } else {
                ""
            }
        )
        .unwrap();
    }
}

impl Drawer {
    pub fn draw(&mut self, state: &State) {
        write!(self.out, "{}", clear::All).unwrap();

        fn draw_layout(out: &mut impl Write, state: &State, layout: &Layout, frame: &Frame) {
            use self::Layout::*;
            match layout {
                Buffer(name) => {
                    let buf = state
                        .buffers
                        .get(name)
                        .expect(format!("internal error: unknown buffer name {}", name).as_str());

                    draw_buffer(out, buf, frame);

                    if name == &state.current_buffer_name {
                        let x: u16 = (frame.x + buf.cursor.x + 1).try_into().unwrap();
                        let y: u16 = (frame.y + buf.cursor.y + 1).try_into().unwrap();
                        write!(out, "{}{}", Goto(x, y), Save).unwrap();
                    }
                }
                Lined(dir, line_width, line, body) => {
                    let (line_frame, body_frame) = splited_frames(dir, *line_width, frame);
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
