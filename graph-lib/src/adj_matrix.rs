#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AdjacencyMatrix {
    matrix: Vec<Vec<bool>>,
}

#[allow(dead_code)]
impl AdjacencyMatrix {
    pub fn new(cols: usize, rows: usize) -> AdjacencyMatrix {
        AdjacencyMatrix {
            matrix: vec![vec![false; cols]; rows],
        }
    }
}

#[allow(dead_code)]
impl AdjacencyMatrix {
    pub fn set_adjacency(&mut self, row: usize, col: usize) {
        self.matrix[row][col] = true;
    }

    pub fn clear_adjacency(&mut self, row: usize, col: usize) {
        self.matrix[row][col] = false;
    }

    pub fn get_adjacency(&self, row: usize) -> &Vec<bool> {
        &self.matrix[row]
    }
}
