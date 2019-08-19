use std::io::{stdout, Stdout, Write};

use termion::clear;
use termion::color;
use termion::cursor::{Goto, Restore, Save};
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use buffer::Buffer;
use config::syntax_highlight;
use cursor::Cursor;
use frame::Frame;
use layout::Layout;
use state::State;
use util::{clamp, Direction, Rgb};

pub struct Drawer {
    out: MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>,
}

fn draw_plain_buffer(out: &mut impl Write, buffer: &Buffer, cursor: &Cursor, frame: &Frame) {
    // `+1` means convertion from 0-origin position to 1-origin position
    let frame_x = frame.x + 1;
    let frame_y = frame.y + 1;

    let top_line = if buffer.height() < frame.height || cursor.y < frame.height / 2 {
        0
    } else if cursor.y + frame.height / 2 > buffer.height() {
        buffer.height() - frame.height
    } else {
        cursor.y - frame.height / 2
    };

    for i in { 0..frame.height } {
        write!(out, "{}", Goto(frame_x as u16, frame_y as u16 + i as u16)).unwrap();
        if i + top_line < buffer.height() {
            let line: String = buffer.line_at(top_line + i).into_iter().collect();
            write!(out, "{}", line.as_str()).unwrap();
        }
    }

    // cursor
    let x = clamp(cursor.x, 0, buffer.line_at(cursor.y).len()) + frame.x + 1;
    let y = frame.y + cursor.y - top_line + 1;

    write!(out, "{}", Goto(x as u16, y as u16)).unwrap();
}

fn print_comment_part(out: &mut impl Write, word: String, color: Rgb) {
    write!(
        out,
        "{}{}{}",
        color::Fg(color::Rgb(color.0, color.1, color.2)),
        word,
        color::Fg(color::Reset)
    )
    .unwrap();
}

fn print_non_comment_part(out: &mut impl Write, line: String, keyword: &syntax_highlight::Keyword) {
    let mut word = String::new();
    for c in line.chars() {
        if c.is_alphabetic() || c == '_' {
            word.push(c);
        } else {
            if keyword.keywords.contains(&word) {
                let Rgb(r, g, b) = keyword.color;
                write!(
                    out,
                    "{}{}{}",
                    color::Fg(color::Rgb(r, g, b)),
                    word,
                    color::Fg(color::Reset)
                )
                .unwrap();
            } else {
                write!(out, "{}", word).unwrap();
            }
            word = String::new();
            write!(out, "{}", c).unwrap();
        }
    }
}

fn draw_syntax_highlighted_buffer(
    out: &mut impl Write,
    buffer: &Buffer,
    cursor: &Cursor,
    frame: &Frame,
) {
    // `+1` means convertion from 0-origin position to 1-origin position
    let frame_x = frame.x + 1;
    let frame_y = frame.y + 1;

    let top_line = if buffer.height() < frame.height || cursor.y < frame.height / 2 {
        0
    } else if cursor.y + frame.height / 2 > buffer.height() {
        buffer.height() - frame.height
    } else {
        cursor.y - frame.height / 2
    };

    syntax_highlight(
        &mut |syntax_highlight: &syntax_highlight::SyntaxHighlight| {
            let ref keyword = syntax_highlight.keyword;
            let ref comment = syntax_highlight.comment;

            let mut is_comment = false;
            for i in { 0..frame.height } {
                if i + top_line >= buffer.height() {
                    break;
                }
                write!(out, "{}", Goto(frame_x as u16, frame_y as u16 + i as u16)).unwrap();

                let mut line: String = buffer.line_at(top_line + i).iter().collect();

                loop {
                    if is_comment {
                        if let Some((_, ref end_comment_mark)) = comment.multi_comment_mark {
                            if let Some(mut comment_pos) = line.find(end_comment_mark) {
                                comment_pos += end_comment_mark.len();
                                let mut left = line;
                                let right = left.split_off(comment_pos);
                                print_comment_part(out, left, comment.color);
                                is_comment = false;
                                line = right;
                            }
                        }
                    };

                    let line_comment_pos =
                        if let Some(ref line_comment_mark) = comment.line_comment_mark {
                            line.find(line_comment_mark)
                        } else {
                            None
                        };

                    let multi_comment_pos =
                        if let Some((ref begin_comment_mark, _)) = comment.multi_comment_mark {
                            line.find(begin_comment_mark)
                        } else {
                            None
                        };

                    match (line_comment_pos, multi_comment_pos) {
                        (Some(line_pos), Some(multi_pos)) => {
                            if line_pos < multi_pos {
                                let mut left = line;
                                let right = left.split_off(line_pos);
                                print_non_comment_part(out, left, keyword);
                                print_comment_part(out, right, comment.color);
                                line = String::new();
                                break;
                            } else {
                                let mut left = line;
                                let mut mid = left.split_off(line_pos);
                                let right = mid.split_off(
                                    comment.multi_comment_mark.as_ref().unwrap().0.len(),
                                );
                                print_non_comment_part(out, left, keyword);
                                print_comment_part(out, mid, comment.color);
                                line = right;
                                is_comment = true;
                                continue;
                            }
                        }
                        (Some(line_pos), None) => {
                            let mut left = line;
                            let right = left.split_off(line_pos);
                            print_non_comment_part(out, left, keyword);
                            print_comment_part(out, right, comment.color);
                            line = String::new();
                            break;
                        }
                        (None, Some(multi_pos)) => {
                            let mut left = line;
                            let mut mid = left.split_off(multi_pos);
                            let right =
                                mid.split_off(comment.multi_comment_mark.as_ref().unwrap().0.len());
                            print_non_comment_part(out, left, keyword);
                            print_comment_part(out, mid, comment.color);
                            line = right;
                            is_comment = true;
                            continue;
                        }
                        (None, None) => break,
                    }
                }
                if is_comment {
                    print_comment_part(out, line, comment.color);
                } else {
                    print_non_comment_part(out, line, keyword);
                }
            }
        },
    );
    // cursor
    let x = clamp(cursor.x, 0, buffer.line_at(cursor.y).len()) + frame.x + 1;
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
                        let line_buf = Buffer::line_number(buf.height());
                        let frame_width = line_buf.line_width_at(0);
                        let (line_frame, buffer_frame) = frame.split(&Direction::Left, frame_width);
                        draw_plain_buffer(out, &line_buf, &panel.cursor, &line_frame);
                        buffer_frame
                    } else {
                        frame.clone()
                    };

                    if panel.enable_syntax_highlight {
                        draw_syntax_highlighted_buffer(out, buf, &panel.cursor, &buffer_frame);
                    } else {
                        draw_plain_buffer(out, buf, &panel.cursor, &buffer_frame);
                    }

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
