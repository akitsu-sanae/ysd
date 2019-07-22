use std::fs::File;
use std::io::Read;

use cursor::Cursor;
use util::Direction;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BufferName(pub String);

use std::fmt;
impl fmt::Display for BufferName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

    pub fn config_buffer() -> (Buffer, BufferName) {
        (
            Buffer {
                data: vec![],
                cursor: Cursor::default(),
            },
            BufferName("__config_buffer_name__".to_string()),
        )
    }

    pub fn insert_line_at_cursor(&mut self) {
        let (left, right) = self.data[self.cursor.y as usize].split_at(self.cursor.x as usize);
        let (left, right) = (left.to_string(), right.to_string());
        self.data[self.cursor.y as usize] = right;
        self.data.insert(self.cursor.y as usize, left);
        self.cursor.x = 0;
        self.cursor.go(Direction::Down, 1);
    }

    pub fn insert_at_cursor(&mut self, c: char) {
        self.data[self.cursor.y as usize].insert(self.cursor.x as usize, c);
        self.cursor.go(Direction::Right, 1);
    }
}
