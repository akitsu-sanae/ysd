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
use status::Mode;
use colors;
use syntax_highlighter;

pub struct Editor {
    cursor: Cursor,
    buffers: Vec<Buffer>,
    status: Status,
    is_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        unsafe {
            initscr();
            raw();
            keypad(stdscr, true);
            scrollok(stdscr, true);
            noecho();
            colors::init();
            syntax_highlighter::init();
        }

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
        match self.status.mode {
            Mode::Move => self.update_move(),
            Mode::Edit => self.update_edit(),
        }
    }

    pub fn draw(&self) {
        for ref buf in &self.buffers {
            buf.draw(self.cursor.get().1 as usize);
        }

        self.status.draw(&self.cursor);
        self.cursor.draw(&self.buffers[0]);
    }

    fn update_move(&mut self) {
        let ch = getch();
        if ch == KEY_F1 {
            self.is_quit = true;
        } else {
            match ch as u8 as char {
                'q' => self.is_quit = true,
                'a' => self.status.mode = Mode::Edit,
                'j' => self.cursor.go(Direction::Left, &self.buffers[0]),
                'l' => self.cursor.go(Direction::Right, &self.buffers[0]),
                'i' => self.cursor.go(Direction::Up, &self.buffers[0]),
                'k' => self.cursor.go(Direction::Down, &self.buffers[0]),
                _ => (),
            }
        }
    }

    fn update_edit(&mut self) {
        let ch = getch();
        match ch {
            27 => self.status.mode = Mode::Move,
            127 | KEY_BACKSPACE => {
                self.buffers[0].erase(self.cursor.get());
                self.cursor.go(Direction::Left, &self.buffers[0]);
            },
            _ => {
                self.buffers[0].insert(self.cursor.get(), ch as u8 as char);
                self.cursor.go(Direction::Right, &self.buffers[0]);
            }
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        endwin();
    }
}

