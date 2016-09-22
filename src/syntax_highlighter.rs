/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::process::exit;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use regex::Regex;
use toml::{Parser, Value};
use ncurses::*;

const COLOR_KEYWORD: i16 = COLOR_BLUE;
const COLOR_TYPE: i16 = COLOR_RED;
const COLOR_LITERAL: i16 = COLOR_GREEN;
const COLOR_OPERATOR: i16 = COLOR_CYAN;

const COLOR_PAIR_KEYWORD: i16 = 10;
const COLOR_PAIR_TYPE: i16 = 11;
const COLOR_PAIR_LITERAL: i16 = 12;
const COLOR_PAIR_OPERATOR: i16 = 13;

#[derive(Debug)]
struct HighlightPattern {
    pub keyword: Vec<String>,
    pub type_: Vec<String>,
    pub literal: Vec<String>,
    pub operator: Vec<char>,
}

fn load() -> Value {
    let filename = format!("{}/.ysd.hi", env::var("HOME").unwrap());
    let mut data = String::new();
    File::open(filename.as_str()).and_then(|mut f| {
        f.read_to_string(&mut data)
    }).expect("can not open ~/.ysd.hi");
    let mut parser = Parser::new(&data);
    match parser.parse() {
        Some(toml) => Value::Table(toml),
            None => {
                for err in &parser.errors {
                    let (low_line, low_col) = parser.to_linecol(err.lo);
                    let (hi_line, hi_col) = parser.to_linecol(err.hi);
                    println!("fail parsing packages.toml at {}:{}-{}:{} : {}",
                             low_line, low_col, hi_line, hi_col, err.desc);
                }
                exit(-1);
            },
    }
}

impl HighlightPattern {
    pub fn new() -> Self {
        let toml = load();
        let toml = toml
            .as_table().expect("invalid highlight file");

        let keywords = toml
            .get("keyword").expect("can not find keyword")
            .as_slice().expect("keyword must be array")
            .into_iter().map(|e| {
                e.as_str().expect("element of keyword must be string")
                .to_string()
            }).collect::<Vec<_> >();

        let types = toml
            .get("type").expect("can not find type")
            .as_slice().expect("type must be array")
            .into_iter().map(|e| {
                e.as_str().expect("element of type must be string")
                .to_string()
            }).collect::<Vec<_> >();

        let literals = toml
            .get("literal").expect("can not find literal")
            .as_slice().expect("literal must be array")
            .into_iter().map(|e| {
                e.as_str().expect("element of literal must be string")
                .to_string()
            }).collect::<Vec<_> >();

        let operators = toml
            .get("operator").expect("can not find operator")
            .as_slice().expect("operator must be array")
            .into_iter().map(|e| {
                e.as_str().expect("element of operator must be string").as_bytes()[0] as char
            }).collect::<Vec<_> >();

        HighlightPattern {
            keyword: keywords,
            type_: types,
            literal: literals,
            operator: operators,
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


use std::sync::Mutex;
lazy_static! {
    static ref HIGHLIGHT_PATTERN: Mutex<HighlightPattern> =
        Mutex::new(HighlightPattern::new());
}

pub fn init() {
    start_color();
    init_pair(COLOR_PAIR_KEYWORD, COLOR_KEYWORD, COLOR_BLACK);
    init_pair(COLOR_PAIR_TYPE, COLOR_TYPE, COLOR_BLACK);
    init_pair(COLOR_PAIR_LITERAL, COLOR_LITERAL, COLOR_BLACK);
    init_pair(COLOR_PAIR_OPERATOR, COLOR_OPERATOR, COLOR_BLACK);

    File::create("tmp").and_then(|mut f| {
        f.write(HIGHLIGHT_PATTERN.lock().unwrap().keyword.join("\n").as_bytes())
    }).unwrap();
}

pub fn draw(y: usize, str: &str) {
    mvprintw(y as i32, 0, str);
    for pos in Regex::new(r"\d+").unwrap().find_iter(str) {
        attron(COLOR_PAIR(COLOR_PAIR_LITERAL));
        mvprintw(y as i32, pos.0 as i32, &str[pos.0 .. pos.1]);
        attroff(COLOR_PAIR(COLOR_PAIR_LITERAL));
    }
    for pos in Regex::new(r"[a-zA-Z0-9_]+").unwrap().find_iter(str) {
        let text = &str[pos.0 .. pos.1].to_string();
        if keywords().contains(text) {
            attron(COLOR_PAIR(COLOR_PAIR_KEYWORD));
            mvprintw(y as i32, pos.0 as i32, text.as_str());
            attroff(COLOR_PAIR(COLOR_PAIR_KEYWORD));
        } else if types().contains(text) {
            attron(COLOR_PAIR(COLOR_PAIR_TYPE));
            mvprintw(y as i32, pos.0 as i32, text.as_str());
            attroff(COLOR_PAIR(COLOR_PAIR_TYPE));
        }
    }
    for (i, ch) in str.char_indices() {
        if operators().contains(&ch) {
            attron(COLOR_PAIR(COLOR_PAIR_OPERATOR));
            mvprintw(y as i32, i as i32, ch.to_string().as_str());
            attroff(COLOR_PAIR(COLOR_PAIR_OPERATOR));
        }
    }
    let mut in_string_literal = false;
    for (i, ch) in str.char_indices() {
        if ch == '"' && in_string_literal {
            in_string_literal = false;
            mvprintw(y as i32, i as i32, ch.to_string().as_str());
            attroff(COLOR_PAIR(COLOR_PAIR_LITERAL));
        } else if ch == '"' && !in_string_literal {
            in_string_literal = true;
            attron(COLOR_PAIR(COLOR_PAIR_LITERAL));
        }
        if in_string_literal {
            mvprintw(y as i32, i as i32, ch.to_string().as_str());
        }
    }
}


