use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            return Err("direction must be one char".to_string());
        }
        if let Some(c) = s.chars().next() {
            match c {
                'L' => Ok(Self::Left),
                'R' => Ok(Self::Right),
                'U' => Ok(Self::Up),
                'D' => Ok(Self::Down),
                _ => Err("unknown direction".to_string()),
            }
        } else {
            Err("not enough characters".to_string())
        }
    }
}
