use buffer::BufferName;
use util::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Layout {
    Buffer(BufferName),

    //    line pos  width    line          body
    Lined(Direction, i32, Box<Layout>, Box<Layout>),
}
