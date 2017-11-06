/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

extern crate toml;
extern crate ncurses;
extern crate regex;

use std::env;

mod buffer;
mod cursor;
mod editor;
mod status;
mod config;
mod terminal;
use editor::Editor;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("error: filename was not given.");
    }

    let mut editor = Editor::new(&args[1]);
    editor.draw();

    loop {
        if editor.is_quit() {
            break;
        }
        editor.update();
        editor.draw();
    }
}

