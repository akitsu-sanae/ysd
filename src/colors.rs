/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use ncurses::*;
use status::Mode;

// red, green, blue, yellow, magenta, cyan, black, white were pre defined
pub const COLOR_TRANS: i16 = -1;
pub const COLOR_GRAY: i16 = 16;
pub const COLOR_DARK_RED: i16 = 17;
pub const COLOR_DARK_GREEN: i16 = 18;
pub const COLOR_DARK_BLUE: i16 = 19;
pub const COLOR_DARK_YELLOW: i16 = 20;
pub const COLOR_DARK_MAGENTA: i16 = 21;
pub const COLOR_DARK_CYAN: i16 = 22;
pub const COLOR_DARK_GRAY: i16 = 23;

pub const COLOR_PAIR_DEFAULT: i16 = 1;
pub const COLOR_PAIR_MOVE_MODE_LEFT: i16 = 2;
pub const COLOR_PAIR_MOVE_MODE_CENTER: i16 = 3;
pub const COLOR_PAIR_MOVE_MODE_RIGHT: i16 = 4;
pub const COLOR_PAIR_EDIT_MODE_LEFT: i16 = 5;
pub const COLOR_PAIR_EDIT_MODE_CENTER: i16 = 6;
pub const COLOR_PAIR_EDIT_MODE_RIGHT: i16 = 7;

pub fn init() {
    use_default_colors();
    start_color();
    init_color(COLOR_GRAY, 160, 160, 160);
    init_color(COLOR_DARK_RED, 160, 0, 0);
    init_color(COLOR_DARK_GREEN, 0, 160, 0);
    init_color(COLOR_DARK_BLUE, 0, 0, 160);
    init_color(COLOR_DARK_YELLOW, 160, 160, 0);
    init_color(COLOR_DARK_MAGENTA, 160, 0, 160);
    init_color(COLOR_DARK_CYAN, 0, 160, 160);
    init_color(COLOR_DARK_GRAY, 80, 80, 80);

    init_pair(COLOR_PAIR_DEFAULT, COLOR_WHITE, COLOR_TRANS);
    init_pair(COLOR_PAIR_MOVE_MODE_LEFT, COLOR_WHITE, COLOR_BLUE);
    init_pair(COLOR_PAIR_MOVE_MODE_CENTER, COLOR_WHITE, COLOR_GRAY);
    init_pair(COLOR_PAIR_MOVE_MODE_RIGHT, COLOR_WHITE, COLOR_TRANS);
    init_pair(COLOR_PAIR_EDIT_MODE_LEFT, COLOR_WHITE, COLOR_GREEN);
    init_pair(COLOR_PAIR_EDIT_MODE_CENTER, COLOR_WHITE, COLOR_GRAY);
    init_pair(COLOR_PAIR_EDIT_MODE_RIGHT, COLOR_WHITE, COLOR_TRANS);

    bkgd(' ' as chtype | COLOR_PAIR(COLOR_PAIR_DEFAULT) as chtype);
}

pub fn mode(mode: &Mode) -> (u64, u64, u64) {
    match mode {
        &Mode::Move => (
            COLOR_PAIR(COLOR_PAIR_MOVE_MODE_LEFT),
            COLOR_PAIR(COLOR_PAIR_MOVE_MODE_CENTER),
            COLOR_PAIR(COLOR_PAIR_MOVE_MODE_RIGHT)),
        &Mode::Edit => (
            COLOR_PAIR(COLOR_PAIR_EDIT_MODE_LEFT),
            COLOR_PAIR(COLOR_PAIR_EDIT_MODE_CENTER),
            COLOR_PAIR(COLOR_PAIR_EDIT_MODE_RIGHT)),
    }
}

