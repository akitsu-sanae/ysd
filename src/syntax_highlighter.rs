/*============================================================================
  Copyright (C) 2017 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;
use toml::{Parser, Table};
use regex::Regex;
use terminal::{self, Color, ColorPair, Text, Frame};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Keyword,
    Type,
    Operator,
    Comment,
    Number,
    String,
    Char,
}

impl TokenType {
    fn literal_pat(&self) -> Option<Vec<String>> {
        use self::TokenType::*;
        match *self {
            Number => Some(vec![r"\d+".to_string()]),
            String => Some(vec![r#""[^"]*""#.to_string()]),
            Char => Some(vec![r#"'[^']*'"#.to_string()]),
            _ => None,
        }
    }
}

use std::slice::Iter;
impl TokenType {
    fn iteration() -> Iter<'static, TokenType> {
        use self::TokenType::*;
        static DATA : [TokenType; 7] = [
            Keyword, Type, Operator, Comment,
            Number, String, Char,
        ];
        DATA.into_iter()
    }

    fn to_string(&self) -> String {
        use self::TokenType::*;
        match *self {
            Keyword => "keyword".to_string(),
            Type => "type".to_string(),
            Operator => "operator".to_string(),
            Comment => "comment".to_string(),
            Number => "number".to_string(),
            String => "string".to_string(),
            Char => "char".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct HighlightInfo {
    pub color: Color,
    pub pat: Vec<String>,
}

impl HighlightInfo {
    fn new() -> Self {
        HighlightInfo {
            color: Color::White,
            pat: vec![],
        }
    }
}

#[derive(Debug)]
pub struct HighlightData {
    pub data: HashMap<TokenType, HighlightInfo>,
}

impl HighlightData {
    fn default() -> Self {
        let mut data = HashMap::new();
        for t in TokenType::iteration() {
            data.insert(*t, HighlightInfo::new());
        }
        HighlightData {
            data: data,
        }
    }

    pub fn new() -> Result<Self, String> {
        let mut input = String::new();
        let filename = format!(
            "{}/.config/ysd.conf",
            env::var("HOME").unwrap());
        File::open(&filename).and_then(|mut f| {
            f.read_to_string(&mut input)
        }).map_err(|_| format!("can not open: {}", filename))?;

        let mut parser = Parser::new(&input);
        match parser.parse() {
            None => Err(format!("can not parse config file: {}", filename)),
            Some(toml) => HighlightData::from_toml(toml)
        }
    }

    fn read_info(token: TokenType, toml: &Table) -> Result<HighlightInfo, String> {
        let token_s = token.to_string();
        let msg = format!("can not read {}", token_s);
        let table = toml.get(&token_s).ok_or(msg.clone())?
            .as_table().ok_or(msg.clone())?;

        let color = table.get("color").ok_or(msg.clone())?.to_string();
        let color = color[1 .. color.len()-1].to_string(); // remove double quot
        let color = Color::from_str(&color).map_err(|_| msg.clone())?;

        if let Some(pat) = token.literal_pat() {
            return Ok(HighlightInfo {
                color: color,
                pat: pat,
            })
        }

        let pat = table.get("patterns").ok_or(msg.clone())?
            .as_slice().ok_or(msg.clone())?
            .into_iter()
            .map(|e| e.as_str().map(str::to_string).ok_or(msg.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(HighlightInfo {
            color: color,
            pat: pat,
        })
    }

    fn from_toml(toml: Table) -> Result<Self, String> {
        let mut data = HashMap::new();
        for token in TokenType::iteration() {
            data.insert(
                *token,
                HighlightData::read_info(*token, &toml)?);
        }
        Ok(HighlightData {
            data: data
        })
    }

    fn make_frame(exprs: &Vec<String>, content: &str, color: ColorPair) -> Frame {
        let mut frame = Frame::new(color);
        let width = content.find('\n').unwrap() + 1;
        for expr in exprs {
            let regex = Regex::new(&expr).unwrap();
            for mat in regex.find_iter(content) {
                let x = mat.start() % width;
                let y = mat.start() / width;
                let content = content[mat.start() .. mat.end()].to_string();
                frame.texts.push(Text {
                    x: x,
                    y: y,
                    content: content,
                });
            }
        }
        frame
    }

    fn make_frames(&self, content: &str) -> Vec<Frame> {
        let mut result = vec![];
        let exprs = &self.data[&TokenType::Type].pat;
        result.push(Self::make_frame(exprs, content, ColorPair::SyntaxType));

        let exprs = &self.data[&TokenType::Keyword].pat;
        result.push(Self::make_frame(exprs, content, ColorPair::SyntaxKeyword));

        let exprs = &self.data[&TokenType::Number].pat;
        result.push(Self::make_frame(exprs, content, ColorPair::SyntaxNumber));

        let exprs = &self.data[&TokenType::String].pat;
        result.push(Self::make_frame(exprs, content, ColorPair::SyntaxString));

        let exprs = &self.data[&TokenType::Char].pat;
        result.push(Self::make_frame(exprs, content, ColorPair::SyntaxChar));
        result
    }
}

pub fn data() -> &'static HighlightData {
    &HIGHLIGHT_DATA
}

lazy_static! {
    static ref HIGHLIGHT_DATA: HighlightData =
        HighlightData::new().unwrap_or_else(|err| {
            use std::io::{stderr, Write};
            writeln!(&mut stderr(), "{}", err).unwrap();
            HighlightData::default()
        });
}

pub fn make_frames(content: &str) -> Vec<Frame> {
    data().make_frames(content)
}


