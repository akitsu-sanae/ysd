use util::Direction;

#[derive(Debug, Clone)]
pub struct Frame {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Frame {
    pub fn screen() -> Self {
        let (width, height) = ::termion::terminal_size().expect("can not get screen size");
        Frame {
            x: 0,
            y: 0,
            width: width as i32,
            height: height as i32,
        }
    }

    pub fn split(&self, dir: &Direction, line_width: i32) -> (Frame, Frame) {
        match dir {
            Direction::Up => (
                Frame {
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height: line_width,
                },
                Frame {
                    x: self.x,
                    y: self.y + line_width,
                    width: self.width,
                    height: self.height - line_width,
                },
            ),
            Direction::Down => (
                Frame {
                    x: self.x,
                    y: self.y + self.height - line_width,
                    width: self.width,
                    height: line_width,
                },
                Frame {
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height: self.height - line_width,
                },
            ),
            Direction::Left => (
                Frame {
                    x: self.x,
                    y: self.y,
                    width: line_width,
                    height: self.height,
                },
                Frame {
                    x: self.x + line_width,
                    y: self.y,
                    width: self.width - line_width,
                    height: self.height,
                },
            ),
            Direction::Right => (
                Frame {
                    x: self.x + self.width - line_width,
                    y: self.y,
                    width: line_width,
                    height: self.height,
                },
                Frame {
                    x: self.x,
                    y: self.y,
                    width: self.width - line_width,
                    height: self.height,
                },
            ),
        }
    }
}
