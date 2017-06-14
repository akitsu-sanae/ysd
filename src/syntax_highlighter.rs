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
use regex::Regex;
use terminal::{self, Color};

#[derive(Debug)]
pub enum LoadError {
    OpenFile,
    Parsing(Vec<(usize, usize, usize, usize)>),
    InvalidFile,
    Keyword,
    KeywordColor,
    KeywordPatterns,
    Type,
    TypeColor,
    TypePatterns,
    Operator,
    OperatorColor,
    OperatorPattern,
    Comment,
    CommentColor,
    CommentPattern,
    Number,
    NumberColor,
    String,
    StringColor,
    Char,
    CharColor,
}


#[derive(Debug)]
pub struct HighlightInfo {
    pub color: Color,
    pub patterns: Vec<String>,
}

impl HighlightInfo {
    fn default() -> Self {
        HighlightInfo {
            color: Color::White,
            patterns: vec![],
        }
    }
}

#[derive(Debug)]
pub struct HighlightData {
    pub keyword: HighlightInfo,
    pub type_: HighlightInfo,
    pub operator: HighlightInfo,
    pub comment: HighlightInfo,
    pub number: HighlightInfo,
    pub string: HighlightInfo,
    pub char: HighlightInfo,
}

impl HighlightData {
    fn default() -> Self {
        HighlightData {
            keyword: HighlightInfo::default(),
            type_: HighlightInfo::default(),
            operator: HighlightInfo::default(),
            comment: HighlightInfo::default(),
            number: HighlightInfo::default(),
            string: HighlightInfo::default(),
            char: HighlightInfo::default(),
        }
    }

    pub fn new() -> Result<Self, LoadError> {
        let mut input = String::new();
        let filename = format!("{}/.config/ysd.conf", env::var("HOME").unwrap());
        File::open(&filename).and_then(|mut f| {
            f.read_to_string(&mut input)
        }).map_err(|_| LoadError::OpenFile)?;

        let mut parser = Parser::new(&input);
        match parser.parse() {
            None => {
                let data = parser.errors.iter().map(|err| {
                    let (low_line, low_col) = parser.to_linecol(err.lo);
                    let (hi_line, hi_col) = parser.to_linecol(err.hi);
                    (low_line, low_col, hi_line, hi_col)
                }).collect();
                Err(LoadError::Parsing(data))
            },
            Some(toml) => HighlightData::from_toml(Value::Table(toml))
        }
    }

    fn from_toml(toml: Value) -> Result<Self, LoadError> {
        let toml = toml
            .as_table().ok_or(LoadError::InvalidFile)?;

        let keyword = toml
            .get("keyword").ok_or(LoadError::Keyword)?
            .as_table().ok_or(LoadError::Keyword)?;
        let keyword_color = keyword
            .get("color").ok_or(LoadError::KeywordColor)?
            .as_str().ok_or(LoadError::KeywordColor)?
            .to_string();
        let keyword_color = Color::from_str(&keyword_color)
            .map_err(|_| LoadError::KeywordColor)?;
        let keyword_patterns = keyword
            .get("patterns").ok_or(LoadError::KeywordPatterns)?
            .as_slice().ok_or(LoadError::KeywordPatterns)?
            .into_iter()
                .map(|e| e.as_str().map(str::to_string).ok_or(LoadError::KeywordPatterns))
            .collect::<Result<Vec<_>, _>>()?;

        let type_ = toml
            .get("type").ok_or(LoadError::Type)?
            .as_table().ok_or(LoadError::Type)?;
        let type_color = type_
            .get("color").ok_or(LoadError::TypeColor)?
            .as_str().ok_or(LoadError::TypeColor)?
            .to_string();
        let type_color = Color::from_str(&type_color)
            .map_err(|_| LoadError::TypeColor)?;
        let type_patterns = type_
            .get("patterns").ok_or(LoadError::TypePatterns)?
            .as_slice().ok_or(LoadError::TypePatterns)?
            .into_iter()
                .map(|e| e.as_str().map(str::to_string).ok_or(LoadError::TypePatterns))
            .collect::<Result<Vec<_>, _>>()?;

        let operator = toml
            .get("operator").ok_or(LoadError::Operator)?
            .as_table().ok_or(LoadError::Operator)?;
        let operator_color = operator
            .get("color").ok_or(LoadError::OperatorColor)?
            .as_str().ok_or(LoadError::OperatorColor)?
            .to_string();
        let operator_color = Color::from_str(&operator_color)
            .map_err(|_| LoadError::OperatorColor)?;
        let operator_patterns = operator
            .get("patterns").ok_or(LoadError::OperatorPattern)?
            .as_slice().ok_or(LoadError::OperatorPattern)?
            .into_iter()
                .map(|e| e.as_str().map(str::to_string).ok_or(LoadError::OperatorPattern))
                .collect::<Result<Vec<_>, _>>()?;

        let comment = toml
            .get("comment").ok_or(LoadError::Comment)?
            .as_table().ok_or(LoadError::Comment)?;
        let comment_color = comment
            .get("color").ok_or(LoadError::CommentColor)?
            .as_str().ok_or(LoadError::CommentColor)?
            .to_string();
        let comment_color = Color::from_str(&comment_color)
            .map_err(|_| LoadError::CommentColor)?;
        let comment_pattern = comment
            .get("patterns").ok_or(LoadError::CommentPattern)?
            .as_slice().ok_or(LoadError::CommentPattern)?
            .into_iter()
                .map(|e| e.as_str().map(str::to_string).ok_or(LoadError::CommentPattern))
            .collect::<Result<Vec<_>, _>>()?;

        let number_color = toml
            .get("number").ok_or(LoadError::Number)?
            .as_table().ok_or(LoadError::Number)?
            .get("color").ok_or(LoadError::NumberColor)?
            .as_str().ok_or(LoadError::NumberColor)?
            .to_string();
        let number_color = Color::from_str(&number_color)
            .map_err(|_| LoadError::NumberColor)?;

        let string_color = toml
            .get("string").ok_or(LoadError::String)?
            .as_table().ok_or(LoadError::String)?
            .get("color").ok_or(LoadError::StringColor)?
            .as_str().ok_or(LoadError::StringColor)?
            .to_string();
        let string_color = Color::from_str(&string_color)
            .map_err(|_| LoadError::StringColor)?;

        let char_color = toml
            .get("char").ok_or(LoadError::Char)?
            .as_table().ok_or(LoadError::Char)?
            .get("color").ok_or(LoadError::CharColor)?
            .as_str().ok_or(LoadError::CharColor)?
            .to_string();
        let char_color = Color::from_str(&char_color)
            .map_err(|_| LoadError::CharColor)?;

        Ok(HighlightData {
            keyword: HighlightInfo {
                color: keyword_color,
                patterns: keyword_patterns,
            },
            type_: HighlightInfo {
                color: type_color,
                patterns: type_patterns,
            },
            operator: HighlightInfo {
                color: operator_color,
                patterns: operator_patterns,
            },
            comment: HighlightInfo {
                color: comment_color,
                patterns: comment_pattern,
            },
            number: HighlightInfo {
                color: number_color,
                patterns: vec![r"\d+".to_string()],
            },
            string: HighlightInfo {
                color: string_color,
                patterns: vec![r#""[^"]*""#.to_string()],
            },
            char: HighlightInfo {
                color: char_color,
                patterns: vec![r#"'[^']*'"#.to_string()],
            },
        })
    }
}

pub fn data() -> &'static HighlightData {
    &HIGHLIGHT_DATA
}

lazy_static! {
    static ref HIGHLIGHT_DATA: HighlightData =
        HighlightData::new().unwrap_or_else(|err| {
            use std::io::{stderr, Write};
            writeln!(&mut stderr(), "{:?}", err).unwrap();
            HighlightData::default()
        });
}

pub fn draw(x: usize, y: usize, lines: &[String]) {
    let text = &lines.iter()
        .map(|line| format!("{:1$}", line, terminal::width() - 1))
        .map(|line| {
            if line.len() < terminal::width() {
                line
            } else {
                line[0 .. terminal::width() - 1].to_string()
            }
        })
        .fold("".to_string(), |acc, line| format!("{}\n{}", acc, line));
    terminal::print(x, y, text);

    let data = data();
    for expr in data.type_.patterns.iter() {
        let regex = Regex::new(expr).unwrap();
        for (start, end) in regex.find_iter(text) {
            let y = start / terminal::width();
            let x = start % terminal::width() - 1;
            let color = terminal::ColorPair::SyntaxType;
            terminal::attribute(terminal::color_pair(color), || {
                terminal::print(x, y, &text[start .. end]);
            });
        }
    }

    for expr in data.keyword.patterns.iter() {
        let regex = Regex::new(expr).unwrap();
        for (start, end) in regex.find_iter(text) {
            let y = start / terminal::width();
            let x = start % terminal::width() - 1;
            let keyword_color = terminal::ColorPair::SyntaxKeyword;
            terminal::attribute(terminal::color_pair(keyword_color), || {
                terminal::print(x, y, &text[start .. end]);
            });
        }
    }

    for expr in data.operator.patterns.iter() {
        let regex = Regex::new(expr).unwrap();
        for (start, end) in regex.find_iter(text) {
            let y = start / terminal::width();
            let x = start % terminal::width() - 1;
            let color = terminal::ColorPair::SyntaxOperator;
            terminal::attribute(terminal::color_pair(color), || {
                terminal::print(x, y, &text[start .. end]);
            });
        }
    }

    for expr in data.number.patterns.iter() {
        let regex = Regex::new(expr).unwrap();
        for (start, end) in regex.find_iter(text) {
            let y = start / terminal::width();
            let x = start % terminal::width() - 1;
            let color = terminal::ColorPair::SyntaxNumber;
            terminal::attribute(terminal::color_pair(color), || {
                terminal::print(x, y, &text[start .. end]);
            });
        }
    }

    for expr in data.string.patterns.iter() {
        let regex = Regex::new(expr).unwrap();
        for (start, end) in regex.find_iter(text) {
            let y = start / terminal::width();
            let x = start % terminal::width() - 1;
            let color = terminal::ColorPair::SyntaxString;
            terminal::attribute(terminal::color_pair(color), || {
                terminal::print(x, y, &text[start .. end]);
            });
        }
    }

    for expr in data.char.patterns.iter() {
        let regex = Regex::new(expr).unwrap();
        for (start, end) in regex.find_iter(text) {
            let y = start / terminal::width();
            let x = start % terminal::width() - 1;
            let color = terminal::ColorPair::SyntaxChar;
            terminal::attribute(terminal::color_pair(color), || {
                terminal::print(x, y, &text[start .. end]);
            });
        }
    }

    for expr in data.comment.patterns.iter() {
        let regex = Regex::new(expr).unwrap();
        for (start, end) in regex.find_iter(text) {
            let y = start / terminal::width();
            let x = start % terminal::width() - 1;
            let color = terminal::ColorPair::SyntaxComment;
            terminal::attribute(terminal::color_pair(color), || {
                terminal::print(x, y, &text[start .. end]);
            });
        }
    }
}

