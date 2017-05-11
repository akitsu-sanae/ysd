/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use ncurses::*;
use buffer::Buffer;

pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            x: 0, // from 0 to getmaxx(stdscr) - 1
            y: 0, // from 0 to buffer.lines.len() - 1
        }
    }

    pub fn go(&mut self, dir: Direction, buf: &Buffer) {
        match dir {
            Direction::Left => {
                if self.x >= 1 {
                    self.x -= 1;
                }
            },
            Direction::Right => {
                unsafe {
                    if self.x+1 < getmaxx(stdscr) as usize {
                        self.x += 1;
                    }
                }
            },
            Direction::Up => {
                if self.y >= 1 {
                    self.y -= 1;
                }
            },
            Direction::Down => {
                if self.y+1 <= buf.lines.len() - 1 {
                    self.y += 1;
                }
            },
        }
    }

    pub fn draw(&self, buf: &Buffer) {
        unsafe {
            let top_line = if self.y < getmaxy(stdscr) as usize / 2usize {
                0
            } else if self.y + getmaxy(stdscr) as usize / 2usize > buf.lines.len() {
                buf.lines.len() - getmaxy(stdscr) as usize
            } else {
                self.y - getmaxy(stdscr) as usize / 2usize
            } as i32;

            if buf.is_valid_pos((self.x, self.y)) {
                mv(self.y as i32 - top_line, self.x as i32);
            } else {
                mv(self.y as i32 - top_line, buf.line(self.y).len() as i32);
            }
        }
    }

    pub fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

