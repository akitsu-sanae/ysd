use util::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    pub x: i32,
    pub y: i32,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor { x: 0, y: 0 }
    }
}

impl Cursor {
    pub fn go(&mut self, dir: Direction, n: i32) {
        use self::Direction::*;
        match dir {
            Up => self.y -= n,
            Down => self.y += n,
            Left => self.x -= n,
            Right => self.x += n,
        }
    }
}
