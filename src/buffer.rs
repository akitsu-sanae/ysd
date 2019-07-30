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
pub struct Original {
    pub start: usize,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub enum Piece {
    Original(usize, usize), // (pos, length)
    Add(String),
}

impl Piece {
    fn is_original(&self) -> bool {
        match self {
            Piece::Original(_, _) => true,
            _ => false,
        }
    }
    fn is_add(&self) -> bool {
        match self {
            Piece::Add(_) => true,
            _ => false,
        }
    }

    fn length(&self) -> usize {
        match self {
            Piece::Original(_, length) => *length,
            Piece::Add(ref str) => str.len() as usize,
        }
    }

    fn split(self, pos: usize) -> (Piece, Piece) {
        match self {
            Piece::Original(start, length) => (
                Piece::Original(start, pos),
                Piece::Original(start + pos, length - pos),
            ),
            Piece::Add(mut left) => {
                let right = left.split_off(pos as usize);
                (Piece::Add(left), Piece::Add(right))
            }
        }
    }

    fn pop(self) -> Option<Piece> {
        match self {
            Piece::Original(_, 1) => None,
            Piece::Original(start, length) => Some(Piece::Original(start, length - 1)),
            Piece::Add(ref str) if str.len() == 1 => None,
            Piece::Add(mut str) => {
                str.pop();
                Some(Piece::Add(str))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    data: Vec<char>,
    piece_tables: Vec<Vec<Piece>>,
}

fn split_line_pieces(line: Vec<Piece>, split_pos: usize) -> (Vec<Piece>, Vec<Piece>) {
    let mut current_pos = 0;
    let mut left = vec![];
    let mut right = vec![];
    for piece in line {
        let piece_length = piece.length();
        if current_pos <= split_pos && split_pos < current_pos + piece_length {
            let (left_, right_) = piece.split(split_pos - current_pos);
            left.push(left_);
            right.push(right_);
        } else if current_pos < split_pos {
            left.push(piece)
        } else {
            right.push(piece)
        }
        current_pos += piece_length;
    }
    (left, right)
}

fn make_buffer_from_string(text: String) -> Buffer {
    let data: Vec<char> = text.chars().collect();
    let mut piece_tables: Vec<Vec<Piece>> = vec![];
    let mut piece = Piece::Original(0, 0);

    for (pos, c) in data.iter().enumerate() {
        if *c == '\n' {
            piece_tables.push(vec![piece]);
            piece = Piece::Original(pos + 1, 0);
        } else {
            piece = if let Piece::Original(pos, length) = piece {
                Piece::Original(pos, length + 1)
            } else {
                unreachable!()
            };
        }
    }

    Buffer {
        data: data,
        piece_tables: piece_tables,
    }
}

impl Buffer {
    pub fn from_file(filename: &str) -> Self {
        let mut text = String::new();
        File::open(filename)
            .and_then(|mut f| f.read_to_string(&mut text))
            .expect("can not open file");

        make_buffer_from_string(text)
    }

    pub fn save_as(&self, filename: &str) -> Result<(), String> {
        // TODO: remove unwrap
        let mut file = File::create(filename).unwrap();
        for piece_table in self.piece_tables.iter() {
            for piece in piece_table.iter() {
                match piece {
                    Piece::Original(start, length) => {
                        let word = &self.data[*start..(*start + *length)];
                        let word: String = word.iter().collect();
                        write!(file, "{}", word).unwrap()
                    }
                    Piece::Add(ref str) => write!(file, "{}", str).unwrap(),
                }
            }
        }
        file.flush().unwrap();
        Ok(())
    }

    pub fn empty() -> Self {
        Buffer {
            data: vec![],
            piece_tables: vec![vec![]],
        }
    }

    pub fn line_number(n_lines: usize) -> Self {
        let width = n_lines.to_string().len();
        Buffer {
            data: vec![],
            piece_tables: (0..n_lines)
                .into_iter()
                .map(|n| vec![Piece::Add(format!("{:width$}", n, width = width))])
                .collect(),
        }
    }

    pub fn status_buffer() -> ((Buffer, BufferId), (Buffer, BufferId)) {
        (
            (Buffer::empty(), BufferId::new()),
            (Buffer::empty(), BufferId::new()),
        )
    }

    pub fn height(&self) -> usize {
        self.piece_tables.len()
    }
    pub fn line_at(&self, line_i: usize) -> String {
        let mut result = String::new();
        for piece in self.piece_tables.get(line_i as usize).unwrap() {
            match piece {
                Piece::Original(start, length) => {
                    let word: String = self.data[*start..(*start + *length)].iter().collect();
                    result += word.as_str();
                }
                Piece::Add(ref str) => result += str.as_str(),
            }
        }
        result
    }
    pub fn line_width_at(&self, line_i: usize) -> usize {
        let mut result: usize = 0;
        for piece in self.piece_tables.get(line_i).unwrap() {
            result += piece.length();
        }
        result
    }

    pub fn clear(&mut self) -> &mut Self {
        self.piece_tables = vec![vec![Piece::Original(0, 0)]];
        self
    }
    pub fn push(&mut self, word: String) -> &mut Self {
        let last = self.piece_tables.last_mut().unwrap();
        last.push(Piece::Add(word));
        self
    }

    pub fn insert_line_at_cursor(&mut self, cursor: &Cursor) {
        if cursor.x >= self.line_width_at(cursor.y) {
            self.piece_tables
                .insert(cursor.y, vec![Piece::Original(0, 0)]);
            return;
        }

        let line = ::std::mem::replace(self.piece_tables.get_mut(cursor.y).unwrap(), vec![]);
        let (left, mut right) = split_line_pieces(line, cursor.x);
        self.piece_tables
            .get_mut(cursor.y)
            .unwrap()
            .append(&mut right);
        self.piece_tables.insert(cursor.y, left);
    }

    pub fn insert_at_cursor(&mut self, c: char, cursor: &Cursor) {
        let line_width = self.line_width_at(cursor.y);
        let insert_x = if cursor.x >= line_width {
            line_width
        } else {
            cursor.x
        };

        let line = ::std::mem::replace(self.piece_tables.get_mut(cursor.y).unwrap(), vec![]);
        let mut current_pos = 0;
        for piece in line {
            let ref mut line = self.piece_tables.get_mut(cursor.y).unwrap();
            let piece_length = piece.length();
            if current_pos == insert_x && piece.is_original() {
                if let Some(Piece::Add(ref mut str)) = line.last_mut() {
                    str.push(c);
                } else {
                    line.push(Piece::Add(format!("{}", c)));
                }
                line.push(piece);
            } else if current_pos <= insert_x && insert_x < current_pos + piece_length {
                match piece {
                    Piece::Original(_, _) => {
                        let (left, right) = piece.split(insert_x - current_pos);
                        let ref mut line = self.piece_tables.get_mut(cursor.y).unwrap();
                        if left.length() != 0 {
                            line.push(left);
                        }
                        line.push(Piece::Add(format!("{}", c)));
                        if right.length() != 0 {
                            line.push(right);
                        }
                    }
                    Piece::Add(mut str) => {
                        str.insert(insert_x - current_pos, c);
                        line.push(Piece::Add(str));
                    }
                }
            } else {
                line.push(piece);
            }
            current_pos += piece_length;
        }
    }

    pub fn erase_at_cursor(&mut self, cursor: &Cursor) {
        let line_width = self.line_width_at(cursor.y);
        let erase_x = if cursor.x >= line_width {
            line_width
        } else {
            cursor.x
        };

        let line = ::std::mem::replace(self.piece_tables.get_mut(cursor.y).unwrap(), vec![]);
        let mut current_pos = 0;
        for piece in line {
            let ref mut line = self.piece_tables.get_mut(cursor.y).unwrap();
            let piece_length = piece.length();
            if current_pos <= erase_x && erase_x < current_pos + piece_length {
                match piece {
                    Piece::Original(_, _) => {
                        let (mut left, right) = piece.split(erase_x - current_pos + 1);
                        let ref mut line = self.piece_tables.get_mut(cursor.y).unwrap();
                        if let Some(left) = left.pop() {
                            line.push(left);
                        }
                        if right.length() != 0 {
                            line.push(right);
                        }
                    }
                    Piece::Add(mut str) => {
                        str.remove(erase_x - current_pos);
                        line.push(Piece::Add(str));
                    }
                }
            } else {
                line.push(piece);
            }
            current_pos += piece_length;
        }
    }
}
