/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use cursor::Cursor;
use terminal::{ColorPair, Frame};
use config::Config;
use terminal;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Move,
    Edit,
    Command,
}

impl Mode {
    pub fn to_color(&self) -> ColorPair {
        match *self {
            Mode::Move => ColorPair::ModeMove,
            Mode::Edit => ColorPair::ModeEdit,
            Mode::Command => ColorPair::ModeCommand,
        }
    }
}

pub struct Status {
    pub mode: Mode,
    pub message: String,
    pub config: Config,
}

impl Status {

    pub fn new() -> Self {
        Status {
            mode: Mode::Move,
            message: String::new(),
            config: Config::new(),
        }
    }

    pub fn make_frames(&self, cur: &Cursor) -> Vec<Frame> {
        let mut lines = vec![];
        for _ in 0 .. terminal::height() - 1 {
            lines.push(String::new());
        }
        let status_line = format!("{} ({}, {})",
            match self.mode {
                Mode::Move =>    "   Move  ",
                Mode::Edit =>    "   Edit  ",
                Mode::Command => " Command ",
            }, cur.x, cur.y);
        vec![Frame {
            texts: vec![
                terminal::Text {
                    x: 0, y: terminal::height() - 1,
                    content: status_line,
                },
            ],
            color: self.mode.to_color(),
            attrs: vec![],
        }]
    }
}

