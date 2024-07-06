use std::str::FromStr;
use core::fmt::Display;

#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
#[repr(u8)]
pub enum Direction {
    Up = b'U',
    Down = b'D',
    Left = b'L',
    Right = b'R',
}

impl Into<char> for Direction {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(())
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction_str = match self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        };

        write!(f, "{}", direction_str)
    }
}