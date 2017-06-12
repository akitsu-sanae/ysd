/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::str::FromStr;

use ncurses::*;
use status::Mode;
use syntax_highlighter;

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

impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        use terminal::Color::*;
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

#[derive(Clone, Copy)]
pub enum ColorPair {
    Default = 1,

    ModeMove = 5,
    ModeEdit,
    ModeCommand,

    SyntaxKeyword = 10,
    SyntaxType,
    SyntaxNumber,
    SyntaxString,
    SyntaxChar,
    SyntaxOperator,
}

use std::ops::BitOr;
impl BitOr<u64> for ColorPair {
    type Output = u64;
    fn bitor(self, rhs: u64) -> u64 {
        self as u64 | rhs
    }
}


const MOVE_COLOR: Color = Color::Blue;
const EDIT_COLOR: Color = Color::Green;
const COMMAND_COLOR: Color = Color::Magenta;

pub fn mode_to_color_pair(mode: &Mode) -> ColorPair {
    match *mode {
        Mode::Move => ColorPair::ModeMove,
        Mode::Edit => ColorPair::ModeEdit,
        Mode::Command => ColorPair::ModeCommand,
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

    use self::ColorPair::*;

    init_pair(Default as i16, Color::White as i16, Color::Trans as i16);

    init_pair(ModeMove as i16, Color::White as i16, MOVE_COLOR as i16);
    init_pair(ModeEdit as i16, Color::White as i16, EDIT_COLOR as i16);
    init_pair(ModeCommand as i16, Color::White as i16, COMMAND_COLOR as i16);

    let colors = syntax_highlighter::colors();
    init_pair(SyntaxKeyword as i16, colors.keyword as i16, Color::Trans as i16);
    init_pair(SyntaxType as i16, colors.type_ as i16, Color::Trans as i16);
    init_pair(SyntaxNumber as i16, colors.number as i16, Color::Trans as i16);
    init_pair(SyntaxString as i16, colors.string as i16, Color::Trans as i16);
    init_pair(SyntaxChar as i16, colors.char as i16, Color::Trans as i16);
    init_pair(SyntaxOperator as i16, colors.operator as i16, Color::Trans as i16);

    bkgd(' ' as chtype | COLOR_PAIR(Default as i16) as chtype);
}

pub fn terminate() {
    endwin();
}

pub fn color_pair(color: ColorPair) -> u64 {
    COLOR_PAIR(color as i16)
}

pub fn clear_to_eol() {
    clrtoeol();
}

pub fn print(x: usize, y: usize, str: &str) {
    mvprintw(y as i32, x as i32, str);
}

pub fn move_to(x: usize, y: usize) {
    mv(y as i32, x as i32);
}

pub fn cursor_pos() -> (usize, usize) {
    unsafe {
        (getcurx(stdscr) as usize, getcury(stdscr) as usize)
    }
}

pub fn width() -> usize {
    unsafe {
        getmaxx(stdscr) as usize
    }
}
pub fn height() -> usize {
    unsafe {
        getmaxy(stdscr) as usize
    }
}

pub enum Key {
    F1,
    Backspace,
    Delete,
    Return,
    Escape,
    Char(char),
}

impl Key {
    pub fn read() -> Self {
        let ch = getch();
        match ch {
            KEY_F1 => Key::F1,
            KEY_BACKSPACE => Key::Backspace,
            27 => Key::Escape,
            127 => Key::Delete,
            10 => Key::Return,
            _ => Key::Char(ch as u8 as char)
        }
    }
}

pub fn attribute<F>(mode: u64, f: F)
    where F: FnOnce() -> ()
{
    attron(mode);
    f();
    attroff(mode);
}

pub struct Attribute {
}

impl Attribute {
    pub fn bold() -> u64 {
        A_BOLD()
    }
}

