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
            x: 1,
            y: 1,
            width: width,
            height: height,
        }
    }
}
