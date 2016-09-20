/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::io::Read;
use std::fs::File;

use ncurses::*;

pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {

    pub fn from_file(filename: &str) -> Self {
        let mut text = String::new();
        File::open(filename).and_then(|mut f| {
            f.read_to_string(&mut text)
        }).expect("can not open file");

        Buffer {
            lines: text.lines().map(str::to_string).collect()
        }
    }

    pub fn insert_line(&mut self, n: usize, str: String) {
        self.lines.insert(n, str);
    }

    pub fn append_line(&mut self, str: String) {
        self.lines.push(str);
    }

    pub fn remove_line(&mut self, n: usize) {
        self.lines.remove(n);
    }

    pub fn erase(&mut self, (x, y): (u32, u32)) {
        if y >= self.lines.len() as u32 {
            return;
        }
        if self.lines[y as usize].len() == 0 {
            return;
        }
        if x < self.lines[y as usize].len() as u32 {
            self.lines[y as usize].remove(x as usize);
        } else {
            self.lines[y as usize].pop().unwrap();
        }
    }
    pub fn insert(&mut self, (x, y): (u32, u32), ch: char) {
        if y >= self.lines.len() as u32 {
            return;
        }
        if x > self.lines[y as usize].len() as u32 {
            self.lines[y as usize].push(ch);
        } else {
            self.lines[y as usize].insert(x as usize, ch);
        }
    }

    pub fn draw(&self, current_line: usize) {
        let top_line = if current_line < getmaxy(stdscr) as usize / 2usize {
            0
        } else if current_line + getmaxy(stdscr) as usize / 2usize > self.lines.len() {
            self.lines.len() - getmaxy(stdscr) as usize
        } else {
            current_line - getmaxy(stdscr) as usize / 2usize
        };
        for i in {0 .. getmaxy(stdscr) as usize} {
            if i + top_line >= self.lines.len() {
                mv(i as i32, 0);
            } else {
                mvprintw(i as i32, 0, self.lines[i + top_line].as_str());
            }
            clrtoeol();
        }
    }

    pub fn is_valid_pos(&self, (x, y): (u32, u32)) -> bool {
        if x >= getmaxx(stdscr) as u32 {
            return false;
        }
        if y >= getmaxy(stdscr) as u32 {
            return false;
        }
        if x >= self.lines[y as usize].len() as u32 {
            return false;
        }
        return true;
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn line(&self, i: u32) -> &String {
        &self.lines[i as usize]
    }
}

