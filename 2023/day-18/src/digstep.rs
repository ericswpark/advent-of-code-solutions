use std::str::FromStr;

use crate::direction::Direction;

#[derive(Debug, Clone)]
pub(crate) struct DigStep {
    pub(crate) direction: Direction,
    pub(crate) length: usize,
    pub(crate) color: String,
}

impl FromStr for DigStep {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 3 {
            return Err("wrong number of parts in dig step".to_string());
        }

        let direction: Direction = parts[0].parse()?;
        let length = parts[1]
            .parse::<usize>()
            .map_err(|_| "length is not usize".to_string())?;
        let color: String = parts[2].chars().skip(2).take(parts[2].len() - 3).collect();
        if color.len() != 6 {
            return Err("incomplete hex code for color in dig step".to_string());
        }

        Ok(DigStep {
            direction,
            length,
            color,
        })
    }
}
