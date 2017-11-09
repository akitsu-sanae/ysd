/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use buffer::Buffer;
use cursor::{Cursor, Direction};
use status::{Status, Mode};
use terminal;

pub struct Editor {
    cursor: Cursor,
    buffer: Buffer,
    status: Status,
    is_quit: bool,
}

impl Editor {
    pub fn new(filename: &str) -> Self {
        terminal::init();

        Editor {
            cursor: Cursor::new(),
            buffer: Buffer::from_file(filename),
            status: Status::new(),
            is_quit: false,
        }
    }

    pub fn is_quit(&self) -> bool {
        self.is_quit
    }

    pub fn update(&mut self) {
        self.status.message = "".to_string();
        match self.status.mode {
            Mode::Move => self.update_move(),
            Mode::Edit => self.update_edit(),
            Mode::Command => self.update_command(),
        }
    }

    pub fn draw(&self) {
        use std::cmp;
        let top = cmp::max(cmp::min(
                self.cursor.y as i32 - terminal::height() as i32 / 2,
                self.buffer.lines.len() as i32 - terminal::height() as i32),
                0) as usize;

        let mut frames = self.buffer.make_frames(top, &self.status.config);
        frames.append(&mut self.status.make_frames(&self.cursor));
        terminal::draw(frames);
        self.cursor.draw(&self.buffer);
    }

    fn update_move(&mut self) {
        use terminal::Key;
        match Key::read() {
            Key::F1 | Key::Char('q') => self.is_quit = true,
            Key::Char('a') => self.status.mode = Mode::Edit,
            Key::Char(':') => self.status.mode = Mode::Command,
            Key::Char('j') => self.cursor.go(Direction::Left, &self.buffer),
            Key::Char('l') => self.cursor.go(Direction::Right, &self.buffer),
            Key::Char('i') => self.cursor.go(Direction::Up, &self.buffer),
            Key::Char('k') => self.cursor.go(Direction::Down, &self.buffer),
            _ => (),

        }
    }

    fn update_edit(&mut self) {
        use terminal::Key;
        match Key::read() {
            Key::Escape => self.status.mode = Mode::Move,
            Key::Delete | Key::Backspace => {
                if self.cursor.x == 0 || self.buffer.lines[self.cursor.y] == "" { // erase newline character
                    let y = self.cursor.y;
                    let x = self.buffer.lines[y-1].len();
                    self.buffer.lines[y-1] += self.buffer.lines[y].clone().as_str();
                    self.buffer.lines.remove(y);
                    self.cursor.y -= 1;
                    self.cursor.x = x;
                } else {
                    self.cursor.go(Direction::Left, &self.buffer);
                    self.buffer.erase(self.cursor.pos());
                }
            },
            Key::Return => {
                self.buffer.lines.insert(self.cursor.y + 1, "".to_string());
                self.cursor.go(Direction::Down, &self.buffer);
            },
            Key::Char(ch) => {
                self.buffer.insert(self.cursor.pos(), ch);
                self.cursor.go(Direction::Right, &self.buffer);
            },
            _ => (),
        }
    }

    fn update_command(&mut self) {
        match terminal::read_command(20, terminal::height()-1).as_str() {
            "move" => self.status.mode = Mode::Move,
            "edit" => self.status.mode = Mode::Edit,
            input => {
                let inputs: Vec<_> = input.split_whitespace().collect();
                match inputs[0] {
                    "save" => {
                        self.buffer.save(inputs[1]);
                        self.status.message = format!("saved at {}", inputs[1]);
                    }
                    "enable" | "disable" => {
                        match inputs[1].trim() {
                            "linenum" => self.status.config.line_number_visible = inputs[0] == "enable",
                            _ => self.status.message = format!("unknown flag: {}", inputs[1]),
                        }
                    },
                    _ => self.status.message = format!("unknown command: {}", inputs[0]),
                }
            },
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        terminal::terminate();
    }
}

