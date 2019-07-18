
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up, Down, Left, Right,
}

pub fn clamp<T: Ord>(v: T, min: T, max: T) -> T {
    if v <= min {
        min
    } else if v >= max {
        max
    } else {
        v
    }
}


