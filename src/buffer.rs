/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/ysd
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::io::Read;
use std::io::Write;
use std::fs::File;
use config::Config;
use syntax_highlighter;
use terminal::{self, ColorPair, Frame, Text};

pub struct Buffer {
    pub lines: Vec<String>
}

impl Buffer {

    pub fn from_file(filename: &str) -> Self {
        let mut text = String::new();
        File::open(filename).and_then(|mut f| {
            f.read_to_string(&mut text)
        }).expect("can not open file");

        Buffer {
            lines: text.lines().map(str::to_string).collect(),
        }
    }

    pub fn erase(&mut self, (x, y): (usize, usize)) {
        if y >= self.lines.len() {
            return;
        }
        if self.lines[y].len() == 0 {
            return;
        }
        if x < self.lines[y].len() {
            self.lines[y].remove(x);
        } else {
            self.lines[y].pop().unwrap();
        }
    }
    pub fn insert(&mut self, (x, y): (usize, usize), ch: char) {
        if y >= self.lines.len() {
            return;
        }
        if x > self.lines[y].len() {
            self.lines[y].push(ch);
        } else {
            self.lines[y].insert(x, ch);
        }
    }

    pub fn make_frames(&self, top: usize, config: &Config) -> Vec<Frame> {
        let mut result = vec![];

        // main frame
        let linenum_width = if config.line_number_visible {
            (top + terminal::height() - 1).to_string().len() + 2
        } else {
            0
        };
        let content_width = terminal::width() - linenum_width;
        let content = (top .. top + terminal::height()-1)
            .map(|i| format!("{:1$}", self.lines[i], content_width))
            .map(|line| {
                if line.len() + linenum_width < terminal::width() {
                    line
                } else {
                    line[0 .. content_width].to_string()
                }
            })
            .fold(String::new(), |acc, line| format!("{}\n{}", acc, line));
        let content = content[1..].to_string();

        let mut main_frame = Frame::new(ColorPair::Normal);
        for (i, line) in content.lines().enumerate() {
            main_frame.texts.push(Text {
                x: linenum_width, y: i,
                content: line.to_string(),
            });
        }
        result.push(main_frame);
        let mut highlight_frames = syntax_highlighter::make_frames(&content);
        for frame in highlight_frames.iter_mut() {
            (*frame).x = linenum_width;
        }
        result.append(&mut highlight_frames);

        // line number frame
        if config.line_number_visible {
            let mut frame = Frame::new(ColorPair::LineNumber);
            for (y, i) in (top .. top + terminal::height()-1).enumerate() {
                frame.texts.push(Text {
                    x: 0, y: y,
                    content: format!("{}: ", i),
                });
            }
            result.push(frame);
        }
        result
    }

    pub fn is_valid_pos(&self, (x, y): (usize, usize)) -> bool {
        if y >= self.lines.len() {
            return false;
        }
        if x >= self.lines[y].len() {
            return false;
        }
        return true;
    }
    pub fn line(&self, i: usize) -> &String {
        &self.lines[i]
    }

    pub fn save(&self, filename: &str) {
        let current_pos = terminal::cursor_pos();

        File::create(filename.clone()).and_then(|mut f|{
            f.write(self.lines.join("\n").as_bytes())
        }).expect("can not create file");
        terminal::move_to(current_pos.0, current_pos.1);
    }
}

