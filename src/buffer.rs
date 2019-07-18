use std::fs::File;
use std::io::Read;

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
}

impl Buffer {
    pub fn from_file(filename: &str) -> Self {
        let mut text = String::new();
        File::open(filename)
            .and_then(|mut f| f.read_to_string(&mut text))
            .expect("can not open file");

        Buffer {
            data: text.lines().map(str::to_string).collect(),
        }
    }

    pub fn insert(&mut self, (x, y): (i32, i32), c: char) {
        self.data[y as usize].insert(x as usize, c);
    }
}
