#[derive(Debug, PartialEq)]
pub struct Point {
    row: f64,
    col: f64,
}

// Associate functions
impl Point {
    pub fn new(row: f64, col: f64) -> Point {
        Point { row, col }
    }
}

// Public methods
impl Point {
    pub fn row(&self) -> f64 {
        self.row
    }

    pub fn col(&self) -> f64 {
        self.col
    }

    pub fn dist(&self, other: &Point) -> f64 {
        ((self.row - other.row).powf(2.0) + (self.col - other.col).powf(2.0)).sqrt()
    }
}
