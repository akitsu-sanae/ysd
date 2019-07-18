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
}
