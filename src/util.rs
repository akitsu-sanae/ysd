use serde::de::{Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl<'de> Deserialize<'de> for Rgb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = toml::value::Value::deserialize(deserializer)?;
        match value {
            toml::Value::String(str) => {
                // TODO: more strict
                let values: Vec<&str> = str.split_terminator(',').collect();
                match values.as_slice() {
                    [r, g, b] => Ok(Rgb(
                        r.parse().unwrap(),
                        g.parse().unwrap(),
                        b.parse().unwrap(),
                    )),
                    _ => panic!("invalid RGB"),
                }
            }
            _ => panic!("invalid color must be string"),
        }
    }
}

// TODO: impl Serialize for Rgb

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
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
