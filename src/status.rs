/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use ncurses::*;
use colors;
use cursor::Cursor;

pub enum Mode {
    Move,
    Edit,
}

pub struct Status {
    pub mode: Mode,
    pub message: String,
}

impl Status {

    pub fn new() -> Self {
        Status {
            mode: Mode::Move,
            message: String::new(),
        }
    }

    pub fn draw(&self, cur: &Cursor) {
        let mut str = String::new();
        match self.mode {
            Mode::Move => str += "Move",
            Mode::Edit => str += "Edit",
        }
        str += format!("({}, {})", cur.get().0, cur.get().1).as_str();

        let (left, center, _) = colors::mode(&self.mode);
        attron(left | A_BOLD());
        unsafe {
            mvprintw(getmaxy(stdscr)-1, 0, str.as_str());
        }
        attroff(left | A_BOLD());
        clrtoeol();

        attron(center | A_BOLD());
        unsafe {
            mvprintw(getmaxy(stdscr)-1, str.len() as i32 + 1, self.message.as_str());
        }
        attroff(center | A_BOLD());
        clrtoeol();
    }
}

