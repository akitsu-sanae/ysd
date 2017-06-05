/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use toml::{Parser, Value};
use ncurses::*;
use colors::Color;

const COLOR_PAIR_KEYWORD: i16 = 10;
const COLOR_PAIR_TYPE: i16 = 11;
const COLOR_PAIR_NUMBER: i16 = 12;
const COLOR_PAIR_STRING: i16 = 13;
const COLOR_PAIR_CHAR: i16 = 14;
const COLOR_PAIR_OPERATOR: i16 = 15;

#[derive(Debug, Clone)]
struct HighlightColors {
    pub keyword: Color,
    pub type_: Color,
    pub number: Color,
    pub string: Color,
    pub char: Color,
    pub operator: Color,
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

impl HighlightColors {
    fn new() -> Result<Self, String> {
        let toml = load(format!("{}/.ysd.hi", env::var("HOME").unwrap()).as_str())?;
        let toml = toml
            .as_table().ok_or("invalid highlight file".to_string())?;

        let keyword_color = toml
            .get("keyword").ok_or("can not find keyword color".to_string())?
            .as_str().ok_or("keyword color must be string".to_string())?;
        let keyword_color = Color::from_str(keyword_color)?;

        let type_color = toml
            .get("type").ok_or("can not find type color".to_string())?
            .as_str().ok_or("type color must be string".to_string())?;
        let type_color = Color::from_str(type_color)?;

        let number_color = toml
            .get("number").ok_or("can not find number color".to_string())?
            .as_str().ok_or("number color must be string".to_string())?;
        let number_color = Color::from_str(number_color)?;

        let string_color = toml
            .get("string").ok_or("can not find string color".to_string())?
            .as_str().ok_or("string color must be string".to_string())?;
        let string_color = Color::from_str(string_color)?;

        let char_color = toml
            .get("char").ok_or("can not find char color".to_string())?
            .as_str().ok_or("char color must be string".to_string())?;
        let char_color = Color::from_str(char_color)?;

        let operator_color = toml
            .get("operator").ok_or("can not find operator color".to_string())?
            .as_str().ok_or("operator color must be string".to_string())?;
        let operator_color = Color::from_str(operator_color)?;
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
            keyword: Color::White,
            type_: Color::White,
            number: Color::White,
            string: Color::White,
            char: Color::White,
            operator: Color::White,
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
            .into_iter()
               .map(|e| e.as_str().map(str::to_string).ok_or("keyword must be string".to_string()))
            .collect::<Result<Vec<_>, _>>()?;

        let types = toml
            .get("type").ok_or("can not find type".to_string())?
            .as_slice().ok_or("type must be array".to_string())?
            .into_iter()
               .map(|e| e.as_str().map(str::to_string).ok_or("keyword must be string".to_string()))
            .collect::<Result<Vec<_>, _>>()?;

       let operators = toml
            .get("operator").ok_or("can not find operator".to_string())?
            .as_slice().ok_or("operator must be array".to_string())?
            .into_iter()
               .map(|e| e.as_str().map(|e| e.as_bytes()[0] as char).ok_or("element of operator must be string".to_string()))
            .collect::<Result<Vec<_>, _>>()?;

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
    init_pair(COLOR_PAIR_KEYWORD, colors.keyword as i16, Color::Trans as i16);
    init_pair(COLOR_PAIR_TYPE, colors.type_ as i16, Color::Trans as i16);
    init_pair(COLOR_PAIR_NUMBER, colors.number as i16, Color::Trans as i16);
    init_pair(COLOR_PAIR_STRING, colors.string as i16, Color::Trans as i16);
    init_pair(COLOR_PAIR_CHAR, colors.char as i16, Color::Trans as i16);
    init_pair(COLOR_PAIR_OPERATOR, colors.operator as i16, Color::Trans as i16);
}

fn is_identifier_char(c: &char) -> bool {
    c.is_digit(10) || c.is_alphabetic() || c.clone() == '_'
}

pub fn draw(y: usize, str: &str, visible_line_numbers: bool) {

    let mut word = String::new();
    let mut is_in_string = false;
    let mut is_in_char = false;
    let mut is_in_number = false;
    let mut is_in_identifier = false;
    let x = if visible_line_numbers {
        let linenum = format!("{0:<3}", y+1);
        attron(COLOR_PAIR(COLOR_PAIR_STRING));
        mvprintw(y as i32, 0, linenum.as_str());
        attroff(COLOR_PAIR(COLOR_PAIR_STRING));
        3
    } else {
        0
    };
    for (i, ch) in format!("{} ", str).as_str().char_indices() {
        let i = i + x;
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

