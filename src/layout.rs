use buffer::BufferId;
use cursor::Cursor;
use frame::Frame;
use util::{clamp, Direction};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PanelName(pub String);

impl PanelName {
    pub fn new(name: &str) -> Self {
        PanelName(name.to_string())
    }
}

use std::fmt;
impl fmt::Display for PanelName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Panel {
    pub cursor: Cursor,
    pub path: Option<String>,
    pub buffer_id: BufferId,
    pub is_visible_line_number: bool,
}

impl Panel {
    pub fn fix_cursor_pos(&mut self, width: usize, height: usize) {
        self.cursor.x = clamp(self.cursor.x, 0, width - 1);
        self.cursor.y = clamp(self.cursor.y, 0, height - 1);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Layout {
    Panel(Panel, PanelName),

    //    line pos  width    line          body
    Lined(Direction, usize, Box<Layout>, Box<Layout>),
}

impl Layout {
    fn traverse_impl<'a, T, E>(
        &'a self,
        f: &dyn Fn(&'a Panel, &'a PanelName, &Frame) -> Result<T, E>,
        frame: &Frame,
    ) -> Result<T, E> {
        match self {
            Layout::Panel(panel, panel_name) => f(panel, panel_name, frame),
            Layout::Lined(dir, line_width, line, body) => {
                let (line_frame, body_frame) = frame.split(dir, *line_width);
                body.traverse_impl(f, &body_frame)
                    .or(line.traverse_impl(f, &line_frame))
            }
        }
    }

    pub fn traverse<'a, T, E>(
        &'a self,
        f: &dyn Fn(&'a Panel, &'a PanelName, &Frame) -> Result<T, E>,
    ) -> Result<T, E> {
        self.traverse_impl(f, &Frame::screen())
    }

    fn traverse_mut_impl<'a, T, E>(
        &'a mut self,
        f: &dyn Fn(&'a mut Panel, &'a mut PanelName, &Frame) -> Result<T, E>,
        frame: &Frame,
    ) -> Result<T, E> {
        match self {
            Layout::Panel(ref mut panel, ref mut panel_name) => f(panel, panel_name, frame),
            Layout::Lined(dir, line_width, ref mut line, ref mut body) => {
                let (line_frame, body_frame) = frame.split(dir, *line_width);
                line.traverse_mut_impl(f, &line_frame)
                    .or(body.traverse_mut_impl(f, &body_frame))
            }
        }
    }

    pub fn traverse_mut<'a, T, E>(
        &'a mut self,
        f: &dyn Fn(&'a mut Panel, &'a mut PanelName, &Frame) -> Result<T, E>,
    ) -> Result<T, E> {
        self.traverse_mut_impl(f, &Frame::screen())
    }
}
