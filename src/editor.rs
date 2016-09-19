/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use ncurses::*;
use buffer::Buffer;
use cursor::Cursor;
use cursor::Direction;
use status::Status;

pub struct Editor {
    cursor: Cursor,
    buffers: Vec<Buffer>,
    status: Status,
    is_quit: bool,
    mode: Mode,
}

enum Mode {
    Move,
    Edit,
}

impl Editor {
    pub fn new() -> Self {
        unsafe {
            initscr();
            raw();
            keypad(stdscr, true);
            scrollok(stdscr, true);
            noecho();
        }

        Editor {
            cursor: Cursor::new(),
            buffers: vec![],
            status: Status::new(),
            is_quit: false,
            mode: Mode::Move,
        }
    }

    pub fn is_quit(&self) -> bool {
        self.is_quit
    }

    pub fn add_buffer(&mut self, buf: Buffer) {
        self.buffers.push(buf)
    }

    pub fn update(&mut self) {
        match self.mode {
            Mode::Move => self.update_move(),
            Mode::Edit => self.update_edit(),
        }
    }

    pub fn draw(&self) {
        for ref buf in &self.buffers {
            buf.draw();
        }
        self.cursor.draw(&self.buffers[0]);
        self.status.draw();
        self.cursor.draw(&self.buffers[0]);
    }

    fn update_move(&mut self) {
        let ch = getch();
        if ch == KEY_F1 {
            self.is_quit = true;
        } else {
            match ch as u8 as char {
                'a' => self.mode = Mode::Edit,
                'j' => self.cursor.go(Direction::Left),
                'l' => self.cursor.go(Direction::Right),
                'i' => self.cursor.go(Direction::Up),
                'k' => self.cursor.go(Direction::Down),
                _ => (),
            }
        }
    }

    fn update_edit(&mut self) {
        let ch = getch();
        match ch {
            27 => self.mode = Mode::Move,
            127 | KEY_BACKSPACE => {
                self.buffers[0].erase(self.cursor.get());
                self.cursor.go(Direction::Left);
            },
            _ => {
                self.buffers[0].insert(self.cursor.get(), ch as u8 as char);
                self.cursor.go(Direction::Right);
            }
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        endwin();
    }
}

