use std::collections::HashMap;
use std::convert::TryInto;

use buffer::{Buffer, BufferId};
use config::Config;
use cursor::Cursor;
use frame::Frame;
use layout::{Layout, Panel, PanelName};
use util::Direction;

#[derive(Clone, Debug)]
pub struct State {
    pub buffers: HashMap<BufferId, Buffer>,
    pub layout: Layout,
    pub current_panel_name: PanelName,
    pub config: Config,
    pub is_quit: bool,
}

impl State {
    pub fn from_file(filename: &str) -> Self {
        let mut buffers = HashMap::new();

        let body_buffer = Buffer::from_file(filename);
        let body_buffer_id = BufferId::new();
        buffers.insert(body_buffer_id, body_buffer);

        let ((mode_buffer, mode_buffer_id), (msg_buffer, msg_buffer_id)) = Buffer::config_buffer();
        buffers.insert(mode_buffer_id, mode_buffer);
        buffers.insert(msg_buffer_id, msg_buffer);

        let config = Config {
            mode_buffer_id: mode_buffer_id,
            msg_buffer_id: msg_buffer_id,
        };

        let body_panel = Panel {
            cursor: Cursor::default(),
            path: Some(filename.to_string()),
            buffer_id: body_buffer_id,
        };

        let config_mode_panel = Panel {
            cursor: Cursor::default(),
            path: None,
            buffer_id: mode_buffer_id,
        };
        let config_msg_panel = Panel {
            cursor: Cursor::default(),
            path: None,
            buffer_id: msg_buffer_id,
        };

        let layout = Layout::Lined(
            Direction::Down,
            1,
            Box::new(Layout::Lined(
                Direction::Left,
                6,
                Box::new(Layout::Panel(
                    config_mode_panel,
                    PanelName::new("__config_mode__"),
                )),
                Box::new(Layout::Panel(
                    config_msg_panel,
                    PanelName::new("__config_msg__"),
                )),
            )),
            Box::new(Layout::Panel(body_panel, PanelName::new(filename))),
        );

        State {
            buffers: buffers,
            layout: layout,
            config: config,
            current_panel_name: PanelName::new(filename),
            is_quit: false,
        }
    }

    pub fn current_panel_with_frame(&self) -> (&Panel, Frame) {
        self.layout
            .traverse(&|panel, panel_name, frame: &Frame| {
                if &self.current_panel_name == panel_name {
                    Ok((panel, frame.clone()))
                } else {
                    Err(())
                }
            })
            .expect("internal error: missing current panel")
    }

    pub fn current_panel(&self) -> &Panel {
        let (panel, _) = self.current_panel_with_frame();
        panel
    }

    pub fn current_panel_with_frame_mut(&mut self) -> (&mut Panel, Frame) {
        let current_panel_name = self.current_panel_name.clone();
        self.layout
            .traverse_mut(&|panel, panel_name, frame| {
                if &current_panel_name == panel_name {
                    Ok((panel, frame.clone()))
                } else {
                    Err(())
                }
            })
            .expect("internal error: missing current panel")
    }
    pub fn current_panel_mut(&mut self) -> &mut Panel {
        let (panel, _) = self.current_panel_with_frame_mut();
        panel
    }

    pub fn update_mode(&mut self, mode: String) {
        self.buffers
            .get_mut(&self.config.mode_buffer_id)
            .expect("internal error: missing config mode buffer")
            .data = vec![mode];
    }

    pub fn update_message(&mut self, msg: &str) {
        self.buffers
            .get_mut(&self.config.msg_buffer_id)
            .expect("internal error: missing config message buffer")
            .data = vec![msg.to_string()];
    }

    pub fn clamp_cursor(&mut self) {
        let buffer_heights: HashMap<BufferId, usize> = self
            .buffers
            .iter()
            .map(|(buffer_id, buffer)| (buffer_id.clone(), buffer.data.len()))
            .collect();
        self.layout
            .traverse_mut::<(), ()>(&|panel, _panel_name, frame| {
                let buffer_id = panel.buffer_id;
                let width = frame.width;
                let height: i32 = buffer_heights
                    .get(&buffer_id)
                    .expect("internal error: missing buffer")
                    .clone()
                    .try_into()
                    .unwrap();
                panel.fix_cursor_pos(width, height);
                Err(())
            });
    }

    pub fn current_buffer(&self) -> &Buffer {
        let buffer_id = self.current_panel().buffer_id;
        self.buffers
            .get(&buffer_id)
            .expect("internal error: missing buffer")
    }
    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        let buffer_id = self.current_panel().buffer_id;
        self.buffers
            .get_mut(&buffer_id)
            .expect("internal error: missing buffer")
    }
}
