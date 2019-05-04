pub mod node;
pub mod point;
pub mod search;

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use node::Node;
use point::Point;
use search::{bfs, dfs, dijk, Search, Status};

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

        while let Status::Searching = search.next() {}

        Ok(search.result())
    }

    pub fn step_depth_first_search(&self, start: &str, end: &str) -> dfs::DepthFirstSearch {
        dfs::DepthFirstSearch::new(self, start, end)
    }

    pub fn breadth_first_search(
        &self,
        start: &str,
        end: &str,
    ) -> Result<Option<Vec<(String, f64)>>, DoesNotContainError> {
        if !self.nodes.contains_key(start) || !self.nodes.contains_key(end) {
            return Err(DoesNotContainError);
        }

        let mut search = bfs::BreadthFirstSearch::new(&self, start, end);

        while let Status::Searching = search.next() {}

        Ok(search.result())
    }

    pub fn step_breadth_first_search(&self, start: &str, end: &str) -> bfs::BreadthFirstSearch {
        bfs::BreadthFirstSearch::new(self, start, end)
    }

    pub fn shortest_path(
        &self,
        start: &str,
        end: &str,
    ) -> Result<Option<Vec<(String, f64)>>, DoesNotContainError> {
        if !self.nodes.contains_key(start) || !self.nodes.contains_key(end) {
            return Err(DoesNotContainError);
        }

        let mut search = dijk::ShortestPath::new(&self, start, end);

        while let Status::Searching = search.next() {}

        Ok(search.result())
    }

    pub fn step_shortest_path(&self, start: &str, end: &str) -> dijk::ShortestPath {
        dijk::ShortestPath::new(self, start, end)
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
