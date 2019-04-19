pub mod dfs;
pub mod node;
pub mod point;

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

use node::Node;
use point::Point;

#[derive(Debug, Clone)]
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
            n.remove_edge(remove_id)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn depth_first_search(
        &self,
        start: &str,
        end: &str,
    ) -> Result<Option<Vec<(String, f64)>>, DoesNotContainError> {
        if !self.nodes.contains_key(start) || !self.nodes.contains_key(end) {
            return Err(DoesNotContainError);
        }

        let mut search = dfs::DepthFirstSearch::new(&self, start, end);

        while let dfs::Status::Searching = search.next() {}

        Ok(search.result())
    }

    pub fn step_depth_first_search<'b>(
        &'b self,
        start: &'b str,
        end: &'b str,
    ) -> dfs::DepthFirstSearch<'b> {
        dfs::DepthFirstSearch::new(&self, start, end)
    }

    pub fn rc_step_depth_first_search(self, start: &str, end: &str) -> dfs::RcDepthFirstSearch {
        dfs::RcDepthFirstSearch::new(Rc::new(self), start, end)
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

#[derive(Debug)]
pub struct DoesNotContainError;

impl fmt::Display for DoesNotContainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Graph does not contain that node.")
    }
}

impl Error for DoesNotContainError {}
