#[derive(Debug, Clone)]
pub struct PuzzleData {
    pub shapes: Vec<Shape>,
    pub regions: Vec<Region>,
}

#[derive(Debug, Clone)]
pub struct Shape(pub Vec<Vec<bool>>);

impl Shape {
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
