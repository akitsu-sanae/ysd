extern crate termion;

use std::io::stdin;

use termion::input::TermRead;

mod buffer;
mod drawer;
mod editor;
mod event_worker;
mod frame;
mod state;
mod cursor;
mod util;

fn main() {
    let args: Vec<_> = ::std::env::args().collect();
    if args.len() != 2 {
        panic!("error: filename was not given.");
    }
    let stdin = stdin();
    let mut editor = editor::Editor::from_file(&args[1]);
    editor.draw();
    for e in stdin.events() {
        let e = e.unwrap();
        editor.update(e);
        if editor.state.is_quit {
            break;
        }
        editor.draw();
    }
}
