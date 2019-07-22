use std::collections::HashMap;

use crate::{
    buffer::{Buffer, BufferName},
    layout::Layout,
};

pub struct State {
    pub buffers: HashMap<BufferName, Buffer>,
    pub layout: Layout,
    pub current_buffer_name: BufferName,
    pub message: String,
    pub is_quit: bool,
}

impl State {
    pub fn from_file(filename: &str) -> Self {
        let buffer = Buffer::from_file(filename);
        let initial_buffer_name = BufferName("<untitled>".to_string());

        let mut buffers = HashMap::new();
        buffers.insert(initial_buffer_name.clone(), buffer);

        let layout = Layout::single_buffer(&initial_buffer_name);

        State {
            buffers: buffers,
            layout: layout,
            current_buffer_name: initial_buffer_name,
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
