/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::str::FromStr;

use ncurses::*;
use syntax_highlighter;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Trans = -1,
    Black, Red, Green, Blue, Yellow, Magenta, Cyan, White,
    Gray, DarkRed, DarkGreen, DarkBlue, DarkYellow, DarkMagenta, DarkCyan, DarkGray,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorPair {
    Normal = 1,

    ModeMove = 5,
    ModeEdit,
    ModeCommand,

    SyntaxKeyword = 10,
    SyntaxIdentifer,
    SyntaxType,
    SyntaxNumber,
    SyntaxString,
    SyntaxChar,
    SyntaxOperator,
    SyntaxComment,

    LineNumber,
}

const MOVE_COLOR: Color = Color::Blue;
const EDIT_COLOR: Color = Color::Green;
const COMMAND_COLOR: Color = Color::Magenta;

pub fn init() {
    initscr();
    raw();
    keypad(stdscr(), true);
    scrollok(stdscr(), true);
    noecho();

    init_colors();
}

fn init_colors() {
    use_default_colors();
    start_color();
    const MAX : i16 = 1000;
    init_color(Color::Black as i16, 0, 0, 0);
    init_color(Color::Red as i16, MAX, 0, 0);
    init_color(Color::Green as i16, 0, MAX, 0);
    init_color(Color::Blue as i16, 0, 0, MAX);
    init_color(Color::Yellow as i16, MAX, MAX, 0);
    init_color(Color::Magenta as i16, MAX, 0, MAX);
    init_color(Color::Cyan as i16, 0, MAX, MAX);
    init_color(Color::White as i16, MAX, MAX, MAX);
    init_color(Color::Gray as i16, 2*MAX/3, 2*MAX/3, 2*MAX/3);
    init_color(Color::DarkRed as i16, MAX/2, 0, 0);
    init_color(Color::DarkGreen as i16, 0, MAX/2, 0);
    init_color(Color::DarkBlue as i16, 0, 0, MAX/2);
    init_color(Color::DarkYellow as i16, MAX/2, MAX/2, 0);
    init_color(Color::DarkMagenta as i16, MAX/2, 0, MAX/2);
    init_color(Color::DarkCyan as i16, 0, MAX/2, MAX/2);
    init_color(Color::DarkGray as i16, MAX/3, MAX/3, MAX/3);

    init_pair(ColorPair::Normal as i16, Color::White as i16, Color::Trans as i16);

    init_pair(ColorPair::ModeMove as i16, Color::White as i16, MOVE_COLOR as i16);
    init_pair(ColorPair::ModeEdit as i16, Color::White as i16, EDIT_COLOR as i16);
    init_pair(ColorPair::ModeCommand as i16, Color::White as i16, COMMAND_COLOR as i16);

    let hi = &syntax_highlighter::data().data;
    use syntax_highlighter::TokenType::*;
    use self::ColorPair::*;
    init_pair(SyntaxKeyword as i16, hi[&Keyword].color as i16, Color::Trans as i16);
    init_pair(SyntaxIdentifer as i16, hi[&Identifier].color as i16, Color::Trans as i16);
    init_pair(SyntaxType as i16, hi[&Type].color as i16, Color::Trans as i16);
    init_pair(SyntaxNumber as i16, hi[&Number].color as i16, Color::Trans as i16);
    init_pair(SyntaxString as i16, hi[&String].color as i16, Color::Trans as i16);
    init_pair(SyntaxChar as i16, hi[&Char].color as i16, Color::Trans as i16);
    init_pair(SyntaxOperator as i16, hi[&Operator].color as i16, Color::Trans as i16);
    init_pair(SyntaxComment as i16, hi[&Comment].color as i16, Color::Trans as i16);

    init_pair(LineNumber as i16, Color::Gray as i16, Color::DarkGray as i16);

    bkgd(' ' as chtype | color_pair(ColorPair::Normal) as chtype);
}

fn color_pair(color: ColorPair) -> u64 {
    COLOR_PAIR(color as i16)
}

pub fn terminate() {
    use_default_colors();
    endwin();
}

pub struct Text {
    pub x: usize,
    pub y: usize,
    pub content: String,
}

pub struct Frame {
    pub x: usize,
    pub y: usize,
    pub texts: Vec<Text>,
    pub color: ColorPair,
    pub attrs: Vec<Attribute>,
}

impl Frame {
    pub fn new(color: ColorPair) -> Self {
        Frame {
            x: 0,
            y: 0,
            texts: vec![],
            color: color,
            attrs: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Attribute {
    Bold
}

impl Attribute {
    fn to_u64(&self) -> u64 {
        match *self {
            Attribute::Bold => A_BOLD(),
        }
    }
}

pub fn draw(frames: Vec<Frame>) {
    erase();
    for frame in frames {
        let mode = frame.attrs.iter().fold(
            color_pair(frame.color),
            |acc, attr| {acc | attr.to_u64()});
        attron(mode);
        for text in frame.texts {
            let x = frame.x + text.x;
            let y = frame.y + text.y;
            mvprintw(y as i32, x as i32, &text.content);
        }
        attroff(mode);
    }
    refresh();
}

pub fn read_command(x: usize, y: usize) -> String {
    let mut result = String::new();
    mv(x as i32, y as i32);
    clrtoeol();
    loop {
        match Key::read() {
            Key::Return => break,
            Key::Backspace if !result.is_empty() => {
                result.pop().unwrap();
            },
            Key::Char(c) => result.push(c),
            _ => (),
        }
        mvprintw(y as i32, x as i32, &result);
        clrtoeol();
    }
    result.as_str().trim().to_string()
}

pub fn move_to(x: usize, y: usize) {
    mv(y as i32, x as i32);
}

pub fn cursor_pos() -> (usize, usize) {
    (getcurx(stdscr()) as usize, getcury(stdscr()) as usize)
}

pub fn width() -> usize {
    getmaxx(stdscr()) as usize
}
pub fn height() -> usize {
    getmaxy(stdscr()) as usize
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

