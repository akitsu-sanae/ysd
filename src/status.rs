/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use ncurses::*;
use colors;

pub enum Mode {
    Move,
    Edit,
}

pub struct Status {
    pub mode: Mode,
}

impl Status {

    pub fn new() -> Self {
        Status {
            mode: Mode::Move,
        }
    }

    pub fn draw(&self) {
        let mut str = String::new();
        match self.mode {
            Mode::Move => str += "Move",
            Mode::Edit => str += "Edit",
        }
        unsafe {
            str += format!("({}, {})", getcurx(stdscr), getcury(stdscr)).as_str();
            let color = colors::mode(&self.mode);
            attron(color | A_BOLD());
            mvprintw(getmaxy(stdscr)-1, 0, str.as_str());
            clrtoeol();
            attroff(color | A_BOLD());
        }
    }
}

