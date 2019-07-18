use crate::util::{Direction, clamp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    pub x: u16,
    pub y: u16,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            x: 0,
            y: 0,
        }
    }
}

impl Cursor {
    pub fn go(&mut self, dir: Direction) {
        use self::Direction::*;
        match dir {
            Up => self.y -= 1,
            Down => self.y += 1,
            Left => self.x -= 1,
            Right => self.x += 1,
        }
        let (width, height) = ::termion::terminal_size().expect("can not get screen size");
        self. x = clamp(self.x, 0, width-1);
        self. y = clamp(self.y, 0, height-1);
    }
}

