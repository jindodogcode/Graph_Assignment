use std::collections::BTreeMap;

use crate::graph::point::Point;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    id: String,
    point: Point,
    edges: BTreeMap<String, f64>,
}

// Associate functions
impl Node {
    pub fn new(id: &str, point: Point) -> Node {
        Node {
            id: id.to_owned(),
            point,
            edges: BTreeMap::new(),
        }
    }
}

// Public methods
impl Node {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn edges(&self) -> &BTreeMap<String, f64> {
        &self.edges
    }

    pub fn add_edge(&mut self, other: &str, dist: f64) {
        self.edges.insert(other.to_owned(), dist);
    }

    pub fn remove_edge(&mut self, id: &str) -> Option<f64> {
        self.edges.remove(id)
    }
}
