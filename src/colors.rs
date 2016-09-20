/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use ncurses::*;
use status::Mode;

// red, green, blue, yellow, magenta, cyan, black, white were pre defined
const COLOR_GRAY: i16 = 16;
const COLOR_DARK_RED: i16 = 17;
const COLOR_DARK_GREEN: i16 = 18;
const COLOR_DARK_BLUE: i16 = 19;
const COLOR_DARK_YELLOW: i16 = 20;
const COLOR_DARK_MAGENTA: i16 = 21;
const COLOR_DARK_CYAN: i16 = 22;
const COLOR_DARK_GRAY: i16 = 23;

static COLOR_PAIR_DEFAULT: i16 = 1;
static COLOR_PAIR_MOVE_MODE: i16 = 2;
static COLOR_PAIR_EDIT_MODE: i16 = 3;

pub fn init() {
    start_color();
    init_color(COLOR_GRAY, 160, 160, 160);
    init_color(COLOR_DARK_RED, 160, 0, 0);
    init_color(COLOR_DARK_GREEN, 0, 160, 0);
    init_color(COLOR_DARK_BLUE, 0, 0, 160);
    init_color(COLOR_DARK_YELLOW, 160, 160, 0);
    init_color(COLOR_DARK_MAGENTA, 160, 0, 160);
    init_color(COLOR_DARK_CYAN, 0, 160, 160);
    init_color(COLOR_DARK_GRAY, 80, 80, 80);

    init_pair(COLOR_PAIR_DEFAULT, COLOR_WHITE, COLOR_BLACK);
    init_pair(COLOR_PAIR_MOVE_MODE, COLOR_WHITE, COLOR_BLUE);
    init_pair(COLOR_PAIR_EDIT_MODE, COLOR_WHITE, COLOR_GREEN);

    bkgd(' ' as chtype | COLOR_PAIR(COLOR_PAIR_DEFAULT) as chtype);
}

pub fn mode(mode: &Mode) -> u64 {
    match mode {
        &Mode::Move => COLOR_PAIR(COLOR_PAIR_MOVE_MODE),
        &Mode::Edit => COLOR_PAIR(COLOR_PAIR_EDIT_MODE),
    }
}

