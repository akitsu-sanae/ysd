use util::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor { x: 0, y: 0 }
    }
}

impl Cursor {
    pub fn go(&mut self, dir: Direction, n: usize) {
        use self::Direction::*;
        match dir {
            Up => self.y = if self.y > n { self.y - n } else { 0 },
            Down => self.y += n,
            Left => self.x = if self.x > n { self.x - n } else { 0 },
            Right => self.x += n,
        }
    }
}
