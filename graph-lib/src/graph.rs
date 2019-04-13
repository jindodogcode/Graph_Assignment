pub mod edge;
pub mod node;
pub mod point;

use std::collections::HashMap;

use node::Node;
use point::Point;

#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<String, Node>,
}

// Associate functions
impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Graph {
        Graph {
            nodes: HashMap::with_capacity(capacity),
        }
    }
}

// Public methods
impl Graph {
    pub fn nodes(&self) -> &HashMap<String, Node> {
        &self.nodes
    }

    pub fn add_node(&mut self, id: &str, point: Point) {
        let node = Node::new(id, point);

        self.nodes.insert(id.to_owned(), node);
    }

    pub fn remove_node(&mut self, id: &str) -> Option<Node> {
        for (_, node) in self.nodes.iter_mut() {
            node.remove_edge(id);
        }

        self.nodes.remove(id)
    }

    pub fn add_edge(&mut self, id: &str, other_id: &str) {
        let dist = self.calc_dist(id, other_id);

        if let Some(n) = self.nodes.get_mut(id) {
            n.add_edge(other_id, dist);
        }

        if let Some(n) = self.nodes.get_mut(other_id) {
            n.add_edge(id, dist);
        }
    }

    pub fn add_directed_edge(&mut self, id: &str, other_id: &str) {
        let dist = self.calc_dist(id, other_id);

        if let Some(n) = self.nodes.get_mut(id) {
            n.add_edge(other_id, dist);
        }
    }

    pub fn remove_edge(&mut self, id: &str, remove_id: &str) -> Option<f64> {
        if let Some(n) = self.nodes.get_mut(id) {
            return n.remove_edge(remove_id);
        } else {
            return None;
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

// Private methods
impl Graph {
    fn calc_dist(&self, id_one: &str, id_two: &str) -> f64 {
        let one = &self.nodes[id_one];
        let two = &self.nodes[id_two];

        one.point().dist(&two.point())
    }
}
