use std::collections::HashMap;

use crate::{
    buffer::{Buffer, BufferName},
    cursor::Cursor,
    frame::Frame,
};

pub struct State {
    pub buffers: HashMap<BufferName, Buffer>,
    pub frames: HashMap<BufferName, Frame>,
    pub current_buffer_name: BufferName,
    pub cursor: Cursor,
    pub message: String,
    pub is_quit: bool,
}

impl State {
    pub fn from_file(filename: &str) -> Self {
        let buffer = Buffer::from_file(filename);
        let initial_buffer_name = BufferName("<untitled>".to_string());

        let mut buffers = HashMap::new();
        buffers.insert(initial_buffer_name.clone(), buffer);

        let mut frames = HashMap::new();
        frames.insert(initial_buffer_name.clone(), Frame::screen());

        State {
            buffers: buffers,
            frames: frames,
            current_buffer_name: initial_buffer_name,
            cursor: Cursor::default(),
            message: String::new(),
            is_quit: false,
        }
    }

    pub fn current_buffer(&self) -> &Buffer {
        self.buffers.get(&self.current_buffer_name).unwrap()
    }

    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        self.buffers.get_mut(&self.current_buffer_name).unwrap()
    }
}
