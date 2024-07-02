use std::cmp::Ordering;
use std::collections::HashSet;

use crate::enums::Direction;

#[derive(Copy, Clone, PartialEq, Hash, Eq, PartialOrd)]
pub(crate) struct Coordinate {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

pub(crate) struct Node {
    pub(crate) value: u8,
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Iteration {
    pub(crate) coordinate: Coordinate,
    pub(crate) direction: Direction,
    pub(crate) moves_left: u8,
    pub(crate) heat_loss: i64,
    pub(crate) visited: HashSet<Coordinate>,
    pub(crate) path_map: Vec<Direction>,
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
    }
}

impl Ord for Iteration {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| self.coordinate.cmp(&other.coordinate))
    }
}

impl PartialOrd for Iteration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
