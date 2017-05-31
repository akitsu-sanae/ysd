/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::str::FromStr;
use ncurses::*;

// red, green, blue, yellow, magenta, cyan, black, white were pre defined
#[derive(Debug, Clone)]
pub enum Color {
    Trans = -1,

    // pre defined colors
    Red = COLOR_RED as isize,
    Green = COLOR_GREEN as isize,
    Blue = COLOR_BLUE as isize,
    Yellow = COLOR_YELLOW as isize,
    Magenta = COLOR_MAGENTA as isize,
    Cyan = COLOR_CYAN as isize,
    Black = COLOR_BLACK as isize,
    White = COLOR_WHITE as isize,

    // custom colors
    Gray = 16, DarkRed, DarkGreen, DarkBlue, DarkYellow, DarkMagenta, DarkCyan, DarkGray,
}

pub const DEFAULT: u64 = 1;

impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        use colors::Color::*;
        Ok(match s {
            "red" => Red,
            "green" => Green,
            "blue" => Blue,
            "yellow" => Yellow,
            "magenta" => Magenta,
            "cyan" => Cyan,
            "black" => Black,
            "white" => White,
            "trans" => Trans,
            "gray" => Gray,
            "dark red" => DarkRed,
            "dark green" => DarkGreen,
            "dark blue" => DarkBlue,
            "dark yellow" => DarkYellow,
            "dark magenta" => DarkMagenta,
            "dark cyan" => DarkCyan,
            "dark gray" => DarkGray,
            _ => Err(format!("no such a color: {}", s))?
        })
    }
}

pub struct ModeColor {
    pub id: i16,
    pub color: Color,
}

impl ModeColor {
    pub fn color(&self) -> u64 {
        COLOR_PAIR(self.id)
    }
}

