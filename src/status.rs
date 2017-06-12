/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use cursor::Cursor;
use terminal::ColorPair;
use terminal;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Move,
    Edit,
    Command,
}

pub struct Status {
    pub mode: Mode,
    pub message: String,
    pub visible_line_number: bool,
}

impl Status {

    pub fn new() -> Self {
        Status {
            mode: Mode::Move,
            message: String::new(),
            visible_line_number: false,
        }
    }

    pub fn draw(&self, cur: &Cursor) {

        use terminal::Attribute;
        let mut str = String::new();
        match self.mode {
            Mode::Move => str += "Move",
            Mode::Edit => str += "Edit",
            Mode::Command => str += "Command",
        }
        str += format!("({}, {})", cur.x, cur.y).as_str();

        let mode_color = terminal::color_pair(terminal::mode_to_color_pair(&self.mode));
        terminal::attribute(mode_color | Attribute::bold(), || {
            terminal::print(0, terminal::height() - 1, &str);
        });
        terminal::clear_to_eol();

        terminal::attribute(mode_color | Attribute::bold(), || {
            terminal::print(0, terminal::height() - 1, &str)
        });
        terminal::clear_to_eol();

        terminal::attribute(ColorPair::Default as u64 | Attribute::bold(), || {
            terminal::print(str.len() + 1, terminal::height() - 1, &self.message)
        });
        terminal::clear_to_eol();
    }
}

