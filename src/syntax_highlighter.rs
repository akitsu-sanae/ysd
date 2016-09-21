/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use ncurses::*;
use colors;

const COLOR_KEYWORD: i16 = COLOR_BLUE;
const COLOR_TYPE: i16 = COLOR_RED;
const COLOR_LITERAL: i16 = COLOR_GREEN;
const COLOR_OPERATOR: i16 = COLOR_BLUE;

const COLOR_PAIR_KEYWORD: i16 = 10;
const COLOR_PAIR_TYPE: i16 = 11;
const COLOR_PAIR_LITERAL: i16 = 12;
const COLOR_PAIR_OPERATOR: i16 = 13;

pub fn init() {
    start_color();
    init_pair(COLOR_PAIR_KEYWORD, COLOR_KEYWORD, COLOR_BLACK);
    init_pair(COLOR_PAIR_TYPE, COLOR_TYPE, COLOR_BLACK);
    init_pair(COLOR_PAIR_LITERAL, COLOR_LITERAL, COLOR_BLACK);
    init_pair(COLOR_PAIR_OPERATOR, COLOR_OPERATOR, COLOR_BLACK);
}

pub fn draw(y: usize, str: &str) {
    let mut word = String::new();
    for (i, ch) in str.char_indices() {
        word.push(ch);
        if !check_char(ch) {
            let color = highlight_attr(word.as_str());
            attron(color);
            mvprintw(y as i32, (1 + i - word.len()) as i32, word.as_str());
            attroff(color);
            word.clear();
            word.push(ch);
        }
    }
}

fn check_char(ch: char) -> bool {
    ch.is_digit(10) || ch.is_alphabetic()
}

fn highlight_attr(word: &str) -> attr_t {
    match word.trim() {
        "break" | "continue" | "do" | "else" | "extern" |
        "in" | "if" | "impl" | "let" | "log" | "loop" |
        "match" | "once" | "priv" | "pub" | "return" | "unsafe" |
        "white" | "use" | "mod" | "trait" | "struct" | "enum" |
        "type" | "fn" | "const" | "mut" | "ref" | "static"
        => COLOR_PAIR(COLOR_PAIR_KEYWORD),
        "char" | "bool" |
        "u8" | "u16" | "u32" | "u64" | "usize" |
        "i8" | "i16" | "i32" | "i64" |
        "f32" | "f64" | "str" | "self" | "Self"
        => COLOR_PAIR(COLOR_PAIR_TYPE),
        _ => {
            match word.trim().parse::<i32>() {
                Ok(_) => COLOR_PAIR(COLOR_PAIR_LITERAL),
                Err(_) => {
                    if word.chars().all(check_char) {
                        COLOR_PAIR(COLOR_PAIR_OPERATOR)
                    } else {
                        COLOR_PAIR(colors::COLOR_PAIR_DEFAULT)
                    }
                },
            }
        },
    }
}


