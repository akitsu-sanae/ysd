/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use ncurses::*;
use buffer::Buffer;
use cursor::{Cursor, Direction};
use status::{Status, Mode};
use display;
use syntax_highlighter;

pub struct Editor {
    cursor: Cursor,
    buffers: Vec<Buffer>,
    status: Status,
    is_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        display::init();
        syntax_highlighter::init();

        Editor {
            cursor: Cursor::new(),
            buffers: vec![],
            status: Status::new(),
            is_quit: false,
        }
    }

    pub fn is_quit(&self) -> bool {
        self.is_quit
    }

    pub fn add_buffer(&mut self, buf: Buffer) {
        self.buffers.push(buf)
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
        if self.status.mode != Mode::Command {
            for ref buf in &self.buffers {
                buf.draw(self.cursor.y);
            }
        }

        self.status.draw(&self.cursor);
        self.cursor.draw(&self.buffers[0]);
    }

    fn update_move(&mut self) {
        let ch = getch();
        if ch == KEY_F1 {
            self.is_quit = true;
            return;
        }
        match ch as u8 as char {
            'q' => self.is_quit = true,
            'a' => self.status.mode = Mode::Edit,
            'j' => self.cursor.go(Direction::Left, &self.buffers[0]),
            'l' => self.cursor.go(Direction::Right, &self.buffers[0]),
            'i' => self.cursor.go(Direction::Up, &self.buffers[0]),
            'k' => self.cursor.go(Direction::Down, &self.buffers[0]),
            ':' => self.status.mode = Mode::Command,
            _ => (),
        }
    }

    fn update_edit(&mut self) {
        let ch = getch();
        match ch {
            27 => self.status.mode = Mode::Move,
            127 | KEY_BACKSPACE => {
                if self.cursor.x == 0 || self.buffers[0].lines[self.cursor.y] == "" { // erase newline character
                    let y = self.cursor.y;
                    let x = self.buffers[0].lines[y-1].len();
                    self.buffers[0].lines[y-1] += self.buffers[0].lines[y].clone().as_str();
                    self.buffers[0].lines.remove(y);
                    self.cursor.y -= 1;
                    self.cursor.x = x;
                } else {
                    self.cursor.go(Direction::Left, &self.buffers[0]);
                    self.buffers[0].erase(self.cursor.pos());
                }
            },
            10 => {
                self.buffers[0].lines.insert(self.cursor.y + 1, "".to_string());
                self.cursor.go(Direction::Down, &self.buffers[0]);
            },
            _ => {
                self.buffers[0].insert(self.cursor.pos(), ch as u8 as char);
                self.cursor.go(Direction::Right, &self.buffers[0]);
            }
        }
    }

    fn get_command(&self) -> String {
        let mut result = String::new();
        unsafe {
            mv(getmaxy(stdscr) - 1, 14);
        }
        clrtoeol();
        loop {
            let ch = getch();
            if ch as u8 as char == '\n' {
                break;
            } if ch == KEY_BACKSPACE && !result.is_empty() {
                result.pop().unwrap();
            } else {
                result.push(ch as u8 as char);
            }
            unsafe {
                mvprintw(getmaxy(stdscr) - 1, 14, result.as_str());
            }
            clrtoeol();
        }
        result
    }

    fn update_command(&mut self) {
        match self.get_command().as_str().trim() {
            "move" => self.status.mode = Mode::Move,
            "edit" => self.status.mode = Mode::Edit,
            input => {
                let inputs: Vec<_> = input.split_whitespace().collect();
                match inputs[0] {
                    "save" => {
                        self.buffers[0].save(inputs[1]);
                        self.status.message = format!("saved at {}", inputs[1]);
                    }
                    _ => self.status.message = format!("unknown command: {}", inputs[0]),
                }
            },
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        endwin();
    }
}

