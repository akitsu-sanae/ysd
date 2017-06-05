/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use ncurses::*;
use colors::{Color, ModeColor, DEFAULT};
use status::Mode;

const MOVE_COLOR: ModeColor = ModeColor{ id: 2, color: Color::Blue };
const EDIT_COLOR: ModeColor = ModeColor{ id: 3, color: Color::Green };
const COMMAND_COLOR: ModeColor = ModeColor{ id: 4, color: Color::Magenta };

pub fn mode_to_modecolor(mode: Mode) -> ModeColor {
    match mode {
        Mode::Move => MOVE_COLOR,
        Mode::Edit => EDIT_COLOR,
        Mode::Command => COMMAND_COLOR,
    }
}

pub fn init() {
    unsafe {
        initscr();
        raw();
        keypad(stdscr, true);
        scrollok(stdscr, true);
        noecho();

        init_colors();
    }
}

fn init_colors() {
    use_default_colors();
    start_color();
    init_color(Color::Gray as i16, 160, 160, 160);
    init_color(Color::DarkRed as i16, 160, 0, 0);
    init_color(Color::DarkGreen as i16, 0, 160, 0);
    init_color(Color::DarkBlue as i16, 0, 0, 160);
    init_color(Color::DarkYellow as i16, 160, 160, 0);
    init_color(Color::DarkMagenta as i16, 160, 0, 160);
    init_color(Color::DarkCyan as i16, 0, 160, 160);
    init_color(Color::DarkGray as i16, 80, 80, 80);

    init_pair(DEFAULT as i16, Color::White as i16, Color::Trans as i16);

    init_pair(MOVE_COLOR.id, Color::White as i16, MOVE_COLOR.color as i16);
    init_pair(EDIT_COLOR.id, Color::White as i16, EDIT_COLOR.color as i16);
    init_pair(COMMAND_COLOR.id, Color::White as i16, COMMAND_COLOR.color as i16);

    bkgd(' ' as chtype | COLOR_PAIR(DEFAULT as i16) as chtype);
}


