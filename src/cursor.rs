/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use ncurses::*;
use buffer::Buffer;

pub struct Cursor {
    x: u32,
    y: u32,
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
                    if self.x+1 < getmaxx(stdscr) as u32 {
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
                if self.y+1 <= buf.lines.len() as u32 - 1 {
                    self.y += 1;
                }
            },
        }
    }

    pub fn draw(&self, buf: &Buffer) {
        if buf.is_valid_pos((self.x, self.y)) {
            mv(self.y as i32, self.x as i32);
        } else {
            mv(self.y as i32, buf.line(self.y).len() as i32);
        }
    }

    pub fn get(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

