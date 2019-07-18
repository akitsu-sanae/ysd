#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use std::str::FromStr;

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_string().to_lowercase().as_str() {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "left" => Ok(Direction::Left),
            "right" => Ok(Direction::Right),
            s => Err(format!("invalid direction: {}", s)),
        }
    }
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
