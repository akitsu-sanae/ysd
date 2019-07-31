use std::collections::HashMap;

use buffer::{Buffer, BufferId};
use cursor::Cursor;
use frame::Frame;
use layout::{Layout, Panel, PanelName};
use status::Status;
use util::Direction;

#[derive(Clone, Debug)]
pub struct State {
    pub buffers: HashMap<BufferId, Buffer>,
    pub layout: Layout,
    pub current_panel_name: PanelName,
    pub status: Status,
    pub is_quit: bool,
}

impl State {
    pub fn from_file(filename: &str) -> Self {
        let mut buffers = HashMap::new();

        let body_buffer = Buffer::from_file(filename);
        let body_buffer_id = BufferId::new();
        buffers.insert(body_buffer_id, body_buffer);

        let ((mode_buffer, mode_buffer_id), (msg_buffer, msg_buffer_id)) = Buffer::status_buffer();
        buffers.insert(mode_buffer_id, mode_buffer);
        buffers.insert(msg_buffer_id, msg_buffer);

        let status = Status {
            mode_buffer_id: mode_buffer_id,
            msg_buffer_id: msg_buffer_id,
        };

        let body_panel = Panel {
            cursor: Cursor::default(),
            path: Some(filename.to_string()),
            buffer_id: body_buffer_id,
            is_visible_line_number: false,
            enable_syntax_highlight: true,
        };

        let status_mode_panel = Panel {
            cursor: Cursor::default(),
            path: None,
            buffer_id: mode_buffer_id,
            is_visible_line_number: false,
            enable_syntax_highlight: false,
        };
        let status_msg_panel = Panel {
            cursor: Cursor::default(),
            path: None,
            buffer_id: msg_buffer_id,
            is_visible_line_number: false,
            enable_syntax_highlight: false,
        };

        let layout = Layout::Lined(
            Direction::Down,
            1,
            Box::new(Layout::Lined(
                Direction::Left,
                6,
                Box::new(Layout::Panel(
                    status_mode_panel,
                    PanelName::new("__status_mode__"),
                )),
                Box::new(Layout::Panel(
                    status_msg_panel,
                    PanelName::new("__status_msg__"),
                )),
            )),
            Box::new(Layout::Panel(body_panel, PanelName::new(filename))),
        );

        State {
            buffers: buffers,
            layout: layout,
            status: status,
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
            .get_mut(&self.status.mode_buffer_id)
            .expect("internal error: missing status mode buffer")
            .clear()
            .push(mode);
    }

    pub fn update_message(&mut self, msg: &str) {
        self.buffers
            .get_mut(&self.status.msg_buffer_id)
            .expect("internal error: missing status message buffer")
            .clear()
            .push(msg.to_string());
    }

    pub fn clamp_cursor(&mut self) {
        let current_buffer_height = self.current_buffer().height();
        let ref current_panel_name = self.current_panel_name;
        self.layout
            .traverse_mut::<(), ()>(&|panel, panel_name, frame| {
                if &panel_name == &current_panel_name {
                    panel.fix_cursor_pos(frame.width, current_buffer_height);
                }
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
