#[derive(Debug, Clone)]
pub struct PuzzleData {
    pub shapes: Vec<Shape>,
    pub regions: Vec<Region>,
}

#[derive(Debug, Clone)]
pub struct Shape(pub Vec<Vec<bool>>);

impl Shape {
    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn area(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|&&cell| cell).count())
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct Region {
    pub dimensions: (usize, usize),
    pub requirements: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Placement {
    pub shape_index: usize,
    pub x: usize,
    pub y: usize,
    pub rotation: Rotation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    None,
    R90,
    R180,
    R270,
}

impl Rotation {
    pub fn next(&self) -> Rotation {
        match self {
            Rotation::None => Rotation::R90,
            Rotation::R90 => Rotation::R180,
            Rotation::R180 => Rotation::R270,
            Rotation::R270 => Rotation::None,
        }
    }
}
