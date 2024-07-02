#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
pub(crate) enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    pub(crate) fn left(self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
            Direction::E => Direction::N,
        }
    }

    pub(crate) fn right(self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
            Direction::E => Direction::S,
        }
    }
}
