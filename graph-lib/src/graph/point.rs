#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    row: i64,
    col: i64,
}

// Associate functions
impl Point {
    pub fn new(row: i64, col: i64) -> Point {
        Point { row, col }
    }
}

// Public methods
impl Point {
    pub fn row(&self) -> i64 {
        self.row
    }

    pub fn col(&self) -> i64 {
        self.col
    }

    pub fn dist(&self, other: &Point) -> f64 {
        (((self.row - other.row).pow(2) + (self.col - other.col).pow(2)) as f64).sqrt()
    }
}
