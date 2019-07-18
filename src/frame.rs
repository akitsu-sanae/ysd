pub struct Frame {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Frame {
    pub fn screen() -> Self {
        let (width, height) = ::termion::terminal_size().expect("can not get screen size");
        Frame {
            x: 0,
            y: 0,
            width: width,
            height: height,
        }
    }
}
