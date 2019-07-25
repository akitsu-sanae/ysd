use std::fs::File;
use std::io::Read;
use std::io::Write;

use cursor::Cursor;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BufferId(usize);

use std::sync::RwLock;
lazy_static! {
    static ref BUFFER_ID_COUNT: RwLock<usize> = RwLock::new(0);
}

impl BufferId {
    pub fn new() -> Self {
        let mut fresh_count = BUFFER_ID_COUNT.write().unwrap();
        let result = BufferId(*fresh_count);
        *fresh_count += 1;
        result
    }
}

#[derive(Debug, Clone)]
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
        }
    }

    pub fn config_buffer() -> ((Buffer, BufferId), (Buffer, BufferId)) {
        (
            (Buffer::empty(), BufferId::new()),
            (Buffer::empty(), BufferId::new()),
        )
    }

    pub fn insert_line_at_cursor(&mut self, cursor: &Cursor) {
        let (left, right) = if cursor.x < self.data[cursor.y as usize].len() as i32 {
            self.data[cursor.y as usize].split_at(cursor.x as usize)
        } else {
            (self.data[cursor.y as usize].as_str(), "")
        };
        let (left, right) = (left.to_string(), right.to_string());
        self.data[cursor.y as usize] = right;
        self.data.insert(cursor.y as usize, left);
    }

    pub fn insert_at_cursor(&mut self, c: char, cursor: &Cursor) {
        if cursor.x < self.data[cursor.y as usize].len() as i32 {
            self.data[cursor.y as usize].insert(cursor.x as usize, c);
        } else {
            self.data[cursor.y as usize].push(c);
        }
    }
}
