/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use buffer::Buffer;
use terminal;

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
            x: 0, // from 0 to terminal::width() - 1
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
                if self.x+1 < terminal::width() {
                    self.x += 1;
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
        let top_line = if self.y < terminal::height() / 2usize {
            0
        } else if self.y + terminal::height() / 2usize > buf.lines.len() {
            buf.lines.len() - terminal::height()
        } else {
            self.y - terminal::height() / 2usize
        };

        if buf.is_valid_pos((self.x, self.y)) {
            terminal::move_to(self.x, self.y - top_line);
        } else {
            terminal::move_to(buf.line(self.y).len(), self.y - top_line);
        }
    }

    pub fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

