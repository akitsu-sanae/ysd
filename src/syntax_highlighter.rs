/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::env;
use std::fs::File;
use std::io::Read;
use toml::{Parser, Value};
use ncurses::*;
use colors;

const COLOR_PAIR_KEYWORD: i16 = 10;
const COLOR_PAIR_TYPE: i16 = 11;
const COLOR_PAIR_NUMBER: i16 = 12;
const COLOR_PAIR_STRING: i16 = 13;
const COLOR_PAIR_CHAR: i16 = 14;
const COLOR_PAIR_OPERATOR: i16 = 15;

#[derive(Debug, Clone)]
struct HighlightColors {
    pub keyword: i16,
    pub type_: i16,
    pub number: i16,
    pub string: i16,
    pub char: i16,
    pub operator: i16,
}

#[derive(Debug)]
struct HighlightPattern {
    pub keyword: Vec<String>,
    pub type_: Vec<String>,
    pub operator: Vec<char>,
    pub colors: HighlightColors,
}

fn load(filename: &str) -> Result<Value, String> {
    let mut data = String::new();
    File::open(filename).and_then(|mut f| {
        f.read_to_string(&mut data)
    }).map_err(|_| format!("can not find: {}", filename))?;
    let mut parser = Parser::new(&data);
    match parser.parse() {
        Some(toml) => Ok(Value::Table(toml)),
            None => {
                let mut msg = "".to_string();
                for err in &parser.errors {
                    let (low_line, low_col) = parser.to_linecol(err.lo);
                    let (hi_line, hi_col) = parser.to_linecol(err.hi);
                    msg += format!("fail parsing {} at {}:{}-{}:{} : {}",
                                   filename,
                                   low_line, low_col, hi_line, hi_col, err.desc).as_str();
                }
                Err(msg)
            },
    }
}

fn str_to_color(s: &str) -> i16 {
    match s {
        "red" => COLOR_RED,
        "green" => COLOR_GREEN,
        "blue" => COLOR_BLUE,
        "yellow" => COLOR_YELLOW,
        "magenta" => COLOR_MAGENTA,
        "cyan" => COLOR_CYAN,
        "black" => COLOR_BLACK,
        "white" => COLOR_WHITE,
        "trans" => colors::COLOR_TRANS,
        "gray" => colors::COLOR_GRAY,
        "dark red" => colors::COLOR_DARK_RED,
        "dark green" => colors::COLOR_DARK_GREEN,
        "dark blue" => colors::COLOR_DARK_BLUE,
        "dark yellow" => colors::COLOR_DARK_YELLOW,
        "dark magenta" => colors::COLOR_DARK_MAGENTA,
        "dark cyan" => colors::COLOR_DARK_CYAN,
        "dark gray" => colors::COLOR_DARK_GRAY,
        _ => panic!("no such color: {}", s),
    }
}

impl HighlightColors {
    fn new() -> Result<Self, String> {
        let toml = load(format!("{}/.ysd.hi", env::var("HOME").unwrap()).as_str())?;
        let toml = toml
            .as_table().ok_or("invalid highlight file".to_string())?;

        let keyword_color = toml
            .get("keyword").ok_or("can not find keyword color".to_string())?
            .as_str().ok_or("keyword color must be string".to_string())?;
        let keyword_color = str_to_color(keyword_color);

        let type_color = toml
            .get("type").ok_or("can not find type color".to_string())?
            .as_str().ok_or("type color must be string".to_string())?;
        let type_color = str_to_color(type_color);

        let number_color = toml
            .get("number").ok_or("can not find number color".to_string())?
            .as_str().ok_or("number color must be string".to_string())?;
        let number_color = str_to_color(number_color);

        let string_color = toml
            .get("string").ok_or("can not find string color".to_string())?
            .as_str().ok_or("string color must be string".to_string())?;
        let string_color = str_to_color(string_color);

        let char_color = toml
            .get("char").ok_or("can not find char color".to_string())?
            .as_str().ok_or("char color must be string".to_string())?;
        let char_color = str_to_color(char_color);

        let operator_color = toml
            .get("operator").ok_or("can not find operator color".to_string())?
            .as_str().ok_or("operator color must be string".to_string())?;
        let operator_color = str_to_color(operator_color);
        Ok(HighlightColors {
            keyword: keyword_color,
            type_: type_color,
            number: number_color,
            string: string_color,
            char: char_color,
            operator: operator_color,
        })
    }

    fn default() -> Self {
        HighlightColors {
            keyword: COLOR_WHITE,
            type_: COLOR_WHITE,
            number: COLOR_WHITE,
            string: COLOR_WHITE,
            char: COLOR_WHITE,
            operator: COLOR_WHITE,
        }
    }
}

impl HighlightPattern {
    pub fn new() -> Result<Self, String> {

        let toml = load(format!("{}/.ysd.syntax", env::var("HOME").unwrap()).as_str())?;
        let toml = toml
            .as_table().ok_or("invalid highlight file".to_string())?;

        let keywords = toml
            .get("keyword").ok_or("can not find keyword".to_string())?
            .as_slice().ok_or("keyword must be array".to_string())?
            .into_iter().map(|e| {
                e.as_str().expect("element of keyword must be string")
                .to_string()
            }).collect::<Vec<_> >();

        let types = toml
            .get("type").ok_or("can not find type".to_string())?
            .as_slice().ok_or("type must be array".to_string())?
            .into_iter().map(|e| {
                e.as_str().expect("element of type must be string")
                .to_string()
            }).collect::<Vec<_> >();

       let operators = toml
            .get("operator").ok_or("can not find operator".to_string())?
            .as_slice().ok_or("operator must be array".to_string())?
            .into_iter().map(|e| {
                e.as_str().expect("element of operator must be string").as_bytes()[0] as char
            }).collect::<Vec<_> >();

       let highlight_colors = HighlightColors::new().unwrap_or_else(|msg| {
           println!("{}", msg);
           HighlightColors::default()
       });

       Ok(HighlightPattern {
           keyword: keywords,
           type_: types,
           operator: operators,
           colors: highlight_colors,
        })
    }

    fn default() -> Self {
        HighlightPattern {
            keyword: vec![],
            type_: vec![],
            operator: vec![],
            colors: HighlightColors::default()
        }
    }
}

fn keywords() -> Vec<String> {
    HIGHLIGHT_PATTERN.lock().unwrap().keyword.clone()
}

fn types() -> Vec<String> {
    HIGHLIGHT_PATTERN.lock().unwrap().type_.clone()
}
fn operators() -> Vec<char> {
    HIGHLIGHT_PATTERN.lock().unwrap().operator.clone()
}
fn colors() -> HighlightColors {
    HIGHLIGHT_PATTERN.lock().unwrap().colors.clone()
}


use std::sync::Mutex;
lazy_static! {
    static ref HIGHLIGHT_PATTERN: Mutex<HighlightPattern> =
        Mutex::new(HighlightPattern::new().unwrap_or_else(|msg| {
            println!("{}", msg);
            HighlightPattern::default()
        }));
}

pub fn init() {
    let colors = colors();
    start_color();
    init_pair(COLOR_PAIR_KEYWORD, colors.keyword, colors::COLOR_TRANS);
    init_pair(COLOR_PAIR_TYPE, colors.type_, colors::COLOR_TRANS);
    init_pair(COLOR_PAIR_NUMBER, colors.number, colors::COLOR_TRANS);
    init_pair(COLOR_PAIR_STRING, colors.string, colors::COLOR_TRANS);
    init_pair(COLOR_PAIR_CHAR, colors.char, colors::COLOR_TRANS);
    init_pair(COLOR_PAIR_OPERATOR, colors.operator, colors::COLOR_TRANS);
}

fn is_identifier_char(c: &char) -> bool {
    c.is_digit(10) || c.is_alphabetic() || c.clone() == '_'
}

pub fn draw(y: usize, str: &str) {

    let mut word = String::new();
    let mut is_in_string = false;
    let mut is_in_char = false;
    let mut is_in_number = false;
    let mut is_in_identifier = false;
    for (i, ch) in format!("{} ", str).as_str().char_indices() {
        match ch {
            '"' => {
                if is_in_char {
                    word.push('"');
                } else if is_in_string {
                    word.push(ch);
                    attron(COLOR_PAIR(COLOR_PAIR_STRING));
                    mvprintw(y as i32, (1 + i - word.len()) as i32, word.as_str());
                    attroff(COLOR_PAIR(COLOR_PAIR_STRING));
                    word.clear();
                    is_in_string = false;
                } else {
                    word.push(ch);
                    is_in_string = true;
                }
            },
            '\'' => {
                let is_escaped = word.len() == 2 && word.as_bytes()[1] == '\\' as u8;
                if is_in_char && !is_escaped {
                    word.push(ch);
                    attron(COLOR_PAIR(COLOR_PAIR_CHAR));
                    mvprintw(y as i32, (1 + i - word.len()) as i32, word.as_str());
                    attroff(COLOR_PAIR(COLOR_PAIR_CHAR));
                    word.clear();
                    is_in_char = false;
                } else {
                    word.push(ch);
                    is_in_char = true;
                }
            },
            _ => {
                if is_in_string || is_in_char {
                    word.push(ch);
                } else if is_identifier_char(&ch) {
                    if ch.is_digit(10) && !is_in_identifier {
                        is_in_number = true;
                    } else {
                        is_in_identifier = true;
                    }
                    word.push(ch);
                } else {
                    if is_in_identifier && keywords().contains(&word){
                        attron(COLOR_PAIR(COLOR_PAIR_KEYWORD));
                        mvprintw(y as i32, (i - word.len()) as i32, word.as_str());
                        attroff(COLOR_PAIR(COLOR_PAIR_KEYWORD));
                        is_in_identifier = false;
                    } else if is_in_identifier && types().contains(&word){
                        attron(COLOR_PAIR(COLOR_PAIR_TYPE));
                        mvprintw(y as i32, (i - word.len()) as i32, word.as_str());
                        attroff(COLOR_PAIR(COLOR_PAIR_TYPE));
                        is_in_identifier = false;
                    } else if is_in_identifier {
                        mvprintw(y as i32, (i - word.len()) as i32, word.as_str());
                        is_in_identifier = false;
                    } else if is_in_number {
                        attron(COLOR_PAIR(COLOR_PAIR_NUMBER));
                        mvprintw(y as i32, (i - word.len()) as i32, word.as_str());
                        attroff(COLOR_PAIR(COLOR_PAIR_NUMBER));
                        is_in_number = false;
                    } else {
                        mvprintw(y as i32, (i - word.len()) as i32, word.as_str());
                    }

                    if operators().contains(&ch) {
                        attron(COLOR_PAIR(COLOR_PAIR_OPERATOR));
                        mvprintw(y as i32, i as i32, ch.to_string().as_str());
                        attroff(COLOR_PAIR(COLOR_PAIR_OPERATOR));
                    } else {
                        mvprintw(y as i32, i as i32, ch.to_string().as_str());
                    }
                    word.clear();
                }
            }
        }
    }
}

