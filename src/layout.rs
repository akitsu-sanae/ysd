use buffer::BufferName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Layout {
    Buffer(BufferName),
}

impl Layout {
    pub fn single_buffer(name: &BufferName) -> Self {
        Layout::Buffer(name.clone())
    }
}
