use std::collections::HashMap;

use buffer::{Buffer, BufferName};
use frame::Frame;
use layout::Layout;
use util::Direction;

#[derive(Clone, Debug)]
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

        let (mode_buffer, msg_buffer, mode_buffer_name, msg_buffer_name) = Buffer::config_buffer();
        buffers.insert(mode_buffer_name.clone(), mode_buffer);
        buffers.insert(msg_buffer_name.clone(), msg_buffer);

        let layout = Layout::Lined(
            Direction::Down,
            1,
            Box::new(Layout::Lined(
                Direction::Left,
                6,
                Box::new(Layout::Buffer(mode_buffer_name)),
                Box::new(Layout::Buffer(msg_buffer_name)),
            )),
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

    pub fn update_mode(&mut self, mode: String) {
        self.buffers
            .get_mut(&BufferName("__config_mode_buffer_name__".to_string()))
            .expect(
                format!("internal error: unknown buffer name __config_mode_buffer_name__").as_str(),
            )
            .data = vec![mode];
    }

    pub fn update_message(&mut self, msg: &str) {
        self.buffers
            .get_mut(&BufferName("__config_msg_buffer_name__".to_string()))
            .expect(
                format!("internal error: unknown buffer name __config_msg_buffer_name__").as_str(),
            )
            .data = vec![msg.to_string()];
    }

    fn clamp_cursor_impl(&mut self, layout: &Layout, frame: &Frame) {
        use self::Layout::*;
        match layout {
            Buffer(name) => {
                let ref mut buf = self
                    .buffers
                    .get_mut(name)
                    .expect(format!("internal error: unknown buffer name {}", name).as_str());

                buf.fix_cursor_pos(frame);
            }
            Lined(dir, line_width, line, body) => {
                let (line_frame, body_frame) = frame.split(dir, *line_width);
                self.clamp_cursor_impl(line, &line_frame);
                self.clamp_cursor_impl(body, &body_frame);
            }
        }
    }

    pub fn clamp_cursors(&mut self) {
        self.clamp_cursor_impl(&self.layout.clone(), &Frame::screen());
    }
}
