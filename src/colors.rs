/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use ncurses::*;
use status::Mode;

static COLOR_PAIR_DEFAULT: i16 = 1;
static COLOR_PAIR_MOVE_MODE: i16 = 2;
static COLOR_PAIR_EDIT_MODE: i16 = 3;

pub fn init() {
    start_color();
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

