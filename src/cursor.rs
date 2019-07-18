use crate::util::{clamp, Direction};

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
        let (width, height) = ::termion::terminal_size().expect("can not get screen size");
        self.x = clamp(self.x, 0, width as i32 - 1);
        self.y = clamp(self.y, 0, height as i32 - 1);
    }
}
