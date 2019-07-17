use std::collections::HashMap;

use crate::{buffer::{Buffer, BufferName}, frame::Frame};

pub struct State {
    pub buffers: HashMap<BufferName, Buffer>,
    pub frames: HashMap<BufferName, Frame>,
    pub is_quit: bool,
}

impl State {
    pub fn from_file(filename: &str) -> Self {
        let buffer = Buffer::from_file(filename);
        let initial_buffer_name = BufferName("<untitled>".to_string());

        let mut buffers = HashMap::new();
        buffers.insert(initial_buffer_name.clone(), buffer);

        let mut frames = HashMap::new();
        frames.insert(initial_buffer_name, Frame::screen());

        State {
            buffers: buffers,
            frames: frames,
            is_quit: false,
        }
    }
}

