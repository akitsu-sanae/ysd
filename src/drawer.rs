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
use util::{clamp, Direction};

pub struct Drawer {
    out: MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>,
}

fn draw_buffer(out: &mut impl Write, buffer: &Buffer, cursor: &Cursor, frame: &Frame) {
    // `+1` means convertion from 0-origin position to 1-origin position
    let frame_x = frame.x + 1;
    let frame_y = frame.y + 1;

    let top_line = if cursor.y < frame.height / 2 {
        0
    } else if cursor.y + frame.height / 2 > buffer.height() {
        buffer.height() - frame.height
    } else {
        cursor.y - frame.height / 2
    };

    for i in { 0..frame.height } {
        write!(out, "{}", Goto(frame_x as u16, frame_y as u16 + i as u16)).unwrap();
        if i + top_line < buffer.height() {
            write!(out, "{}", buffer.line_at(top_line + i).as_str()).unwrap();
        }
    }

    // cursor
    let x = clamp(cursor.x, 0, buffer.line_at(cursor.y).len() - 1) + frame.x + 1;
    let y = frame.y + cursor.y - top_line + 1;

    write!(out, "{}", Goto(x as u16, y as u16)).unwrap();
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

                    let buffer_frame = if panel.is_visible_line_number {
                        let (line_frame, buffer_frame) = frame.split(&Direction::Left, 3);
                        let line_buf = Buffer::line_number(buf.height());
                        draw_buffer(out, &line_buf, &panel.cursor, &line_frame);
                        buffer_frame
                    } else {
                        frame.clone()
                    };

                    draw_buffer(out, buf, &panel.cursor, &buffer_frame);

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
