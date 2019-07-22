use buffer::BufferName;
use util::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Layout {
    Buffer(BufferName),
    Lined(Direction, Box<Layout>, Box<Layout>),
}
