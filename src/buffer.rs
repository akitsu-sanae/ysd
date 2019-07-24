use std::fs::File;
use std::io::Read;
use std::io::Write;

use cursor::Cursor;
use frame::Frame;
use util::{clamp, Direction};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BufferName(pub String);

use std::fmt;
impl fmt::Display for BufferName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    pub data: Vec<String>,
    pub cursor: Cursor,
}

impl Buffer {
    pub fn from_file(filename: &str) -> Self {
        let mut text = String::new();
        File::open(filename)
            .and_then(|mut f| f.read_to_string(&mut text))
            .expect("can not open file");

        Buffer {
            data: text.lines().map(str::to_string).collect(),
            cursor: Cursor::default(),
        }
    }

    pub fn save_as(&self, filename: &str) -> Result<(), String> {
        // TODO: remove unwrap
        let mut file = File::create(filename).unwrap();
        writeln!(file, "{}", self.data.join("\n")).unwrap();
        file.flush().unwrap();
        Ok(())
    }

    pub fn empty() -> Self {
        Buffer {
            data: vec![String::new()],
            cursor: Cursor::default(),
        }
    }

    pub fn config_buffer() -> (Buffer, Buffer, BufferName, BufferName) {
        (
            Buffer::empty(),
            Buffer::empty(),
            BufferName("__config_mode_buffer_name__".to_string()),
            BufferName("__config_msg_buffer_name__".to_string()),
        )
    }

    pub fn insert_line_at_cursor(&mut self) {
        let (left, right) = if self.cursor.x < self.data[self.cursor.y as usize].len() as i32 {
            self.data[self.cursor.y as usize].split_at(self.cursor.x as usize)
        } else {
            (self.data[self.cursor.y as usize].as_str(), "")
        };
        let (left, right) = (left.to_string(), right.to_string());
        self.data[self.cursor.y as usize] = right;
        self.data.insert(self.cursor.y as usize, left);
        self.cursor.x = 0;
        self.cursor.go(Direction::Down, 1);
    }

    pub fn insert_at_cursor(&mut self, c: char) {
        if self.cursor.x < self.data[self.cursor.y as usize].len() as i32 {
            self.data[self.cursor.y as usize].insert(self.cursor.x as usize, c);
        } else {
            self.data[self.cursor.y as usize].push(c);
        }
        self.cursor.go(Direction::Right, 1);
    }

    pub fn fix_cursor_pos(&mut self, frame: &Frame) {
        self.cursor.x = clamp(self.cursor.x, 0, frame.width - 1);
        self.cursor.y = clamp(self.cursor.y, 0, self.data.len() as i32 - 1);
    }
}
