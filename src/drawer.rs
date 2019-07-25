use std::convert::TryInto;
use std::io::{stdout, Stdout, Write};

use termion::clear;
use termion::cursor::{Goto, Restore, Save};
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use buffer::Buffer;
use cursor::Cursor;
use frame::Frame;
use layout::Layout;
use state::State;
use util::clamp;

pub struct Drawer {
    out: MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>,
}

fn draw_buffer(out: &mut impl Write, buffer: &Buffer, cursor: &Cursor, frame: &Frame) {
    // `+1` means convertion from 0-origin position to 1-origin position
    let frame_x: u16 = (frame.x + 1).try_into().unwrap();
    let frame_y: u16 = (frame.y + 1).try_into().unwrap();

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

    // cursor
    let x = frame.x + cursor.x;
    let y = frame.y + cursor.y;

    let x: u16 = (clamp(x, 0, buffer.data[cursor.y as usize].len() as i32) + 1)
        .try_into()
        .unwrap();
    let y: u16 = (y - top_line + 1).try_into().unwrap();

    write!(out, "{}", Goto(x, y)).unwrap();
}

impl Drawer {
    pub fn draw(&mut self, state: &State) {
        write!(self.out, "{}", clear::All).unwrap();

        fn draw_layout(out: &mut impl Write, state: &State, layout: &Layout, frame: &Frame) {
            use self::Layout::*;
            match layout {
                Panel(panel, panel_name) => {
                    let ref buf = state.buffers.get(&panel.buffer_id).expect(
                        format!("internal error: unknown buffer name {}", panel_name).as_str(),
                    );

                    draw_buffer(out, buf, &panel.cursor, frame);

                    // save cursor pos
                    if panel_name == &state.current_panel_name {
                        write!(out, "{}", Save).unwrap();
                    }
                }
                Lined(dir, line_width, line, body) => {
                    let (line_frame, body_frame) = frame.split(dir, *line_width);
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
