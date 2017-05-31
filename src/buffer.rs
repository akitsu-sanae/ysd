/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::io::Read;
use std::io::Write;
use std::fs::File;

use ncurses::*;
use syntax_highlighter;

pub struct Buffer {
    pub lines: Vec<String>
}

impl Buffer {

    pub fn from_file(filename: &str) -> Self {
        let mut text = String::new();
        File::open(filename).and_then(|mut f| {
            f.read_to_string(&mut text)
        }).expect("can not open file");

        Buffer {
            lines: text.lines().map(str::to_string).collect(),
        }
    }

    pub fn erase(&mut self, (x, y): (usize, usize)) {
        if y >= self.lines.len() {
            return;
        }
        if self.lines[y].len() == 0 {
            return;
        }
        if x < self.lines[y].len() {
            self.lines[y].remove(x);
        } else {
            self.lines[y].pop().unwrap();
        }
    }
    pub fn insert(&mut self, (x, y): (usize, usize), ch: char) {
        if y >= self.lines.len() {
            return;
        }
        if x > self.lines[y].len() {
            self.lines[y].push(ch);
        } else {
            self.lines[y].insert(x, ch);
        }
    }

    pub fn draw(&self, current_line: usize) {
        unsafe {
            let top_line = if current_line < getmaxy(stdscr) as usize / 2usize {
                0
            } else if current_line + getmaxy(stdscr) as usize / 2usize > self.lines.len() {
                self.lines.len() - getmaxy(stdscr) as usize
            } else {
                current_line - getmaxy(stdscr) as usize / 2usize
            };
            for i in {0 .. getmaxy(stdscr) as usize} {
                mv(i as i32, 0);
                clrtoeol();
                if i + top_line < self.lines.len() {
                    syntax_highlighter::draw(i, self.lines[i + top_line].as_str());
                }
            }
        }
    }

    pub fn is_valid_pos(&self, (x, y): (usize, usize)) -> bool {
        if y >= self.lines.len() {
            return false;
        }
        if x >= self.lines[y].len() {
            return false;
        }
        return true;
    }
    pub fn line(&self, i: usize) -> &String {
        &self.lines[i]
    }

    pub fn save(&self, filename: &str) {
        let current_pos = unsafe {
            (getcurx(stdscr), getcury(stdscr))
        };

        File::create(filename.clone()).and_then(|mut f|{
            f.write(self.lines.join("\n").as_bytes())
        }).expect("can not create file");
        mv(current_pos.1, current_pos.0);
    }
}

