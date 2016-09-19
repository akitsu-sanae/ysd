/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

extern crate ncurses;

use std::env;

mod buffer;
mod cursor;
mod editor;
mod status;
use editor::Editor;
use buffer::Buffer;

fn main() {

    let mut editor = Editor::new();

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("error: filename was not given.");
    }

    editor.add_buffer(Buffer::from_file(args[1].as_str()));
    editor.draw();

    loop {
        if editor.is_quit() {
            break;
        }
        editor.update();
        editor.draw();
    }
}

