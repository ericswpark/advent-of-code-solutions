use crate::enums::Direction;

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
pub(crate) struct Coordinate {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

pub(crate) struct Node {
    pub(crate) value: u8,
    pub(crate) min_heat_loss: i64
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Iteration {
    pub(crate) coordinate: Coordinate,
    pub(crate) direction: Direction,
    pub(crate) moves_left: u8,
    pub(crate) heat_loss: i64,
    pub(crate) path_map: Vec<Direction>
}