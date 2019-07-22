use std::collections::HashMap;

use buffer::{Buffer, BufferName};
use layout::Layout;
use util::Direction;

pub struct State {
    pub buffers: HashMap<BufferName, Buffer>,
    pub layout: Layout,
    pub current_buffer_name: BufferName,
    pub is_quit: bool,
}

impl State {
    pub fn from_file(filename: &str) -> Self {
        let mut buffers = HashMap::new();

        let body_buffer = Buffer::from_file(filename);
        let initial_buffer_name = BufferName("<untitled>".to_string());
        buffers.insert(initial_buffer_name.clone(), body_buffer);

        let (config_buffer, config_buffer_name) = Buffer::config_buffer();
        buffers.insert(config_buffer_name.clone(), config_buffer);

        let layout = Layout::Lined(
            Direction::Down,
            Box::new(Layout::Buffer(config_buffer_name)),
            Box::new(Layout::Buffer(initial_buffer_name.clone())),
        );

        State {
            buffers: buffers,
            layout: layout,
            current_buffer_name: initial_buffer_name,
            is_quit: false,
        }
    }

    pub fn current_buffer(&self) -> &Buffer {
        self.buffers.get(&self.current_buffer_name).unwrap()
    }

    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        self.buffers.get_mut(&self.current_buffer_name).unwrap()
    }

    pub fn update_message(&mut self, msg: &str) {
        self.buffers
            .get_mut(&BufferName("__config_buffer_name__".to_string()))
            .expect(format!("internal error: unknown buffer name __config_buffer_name__").as_str())
            .data = vec![msg.to_string()];
    }
}
