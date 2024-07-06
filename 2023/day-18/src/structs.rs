use std::str::FromStr;

use crate::enums::Direction;

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct DigStep {
    pub direction: Direction,
    pub repeat: u8,
    pub color: String,
}


impl FromStr for DigStep {
    type Err = ();

    fn from_str<'a>(s: &'a str) -> Result<Self, Self::Err> {
        let mut s_split = s.split(" ");

        let direction = s_split.next().ok_or(())?.parse()?;
        let repeat = s_split.next().ok_or(())?.parse::<u8>().unwrap();
        let color_enclosed = s_split.next().ok_or(())?;

        let color = color_enclosed
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .ok_or(())?.to_string();

        Ok(DigStep { direction, repeat, color })
    }
}