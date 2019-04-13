use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge {
    node: Rc<RefCell<Node>>,
    weight: u64,
}

#[allow(dead_code)]
impl Edge {
    pub fn new(node: Rc<RefCell<Node>>, weight: u64) -> Edge {
        Edge { node, weight }
    }
}

#[allow(dead_code)]
impl Edge {
    pub fn node(&self) -> Rc<RefCell<Node>> {
        Rc::clone(&self.node)
    }

    pub fn weight(&self) -> u64 {
        self.weight
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    id: String,
    edges: Vec<Edge>,
}

#[allow(dead_code)]
impl Node {
    pub fn new(id: &str) -> Node {
        Node {
            id: id.to_owned(),
            edges: Vec::new(),
        }
    }
}

#[allow(dead_code)]
impl Node {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn add_edge_raw(&mut self, node: Rc<RefCell<Node>>, weight: u64) {
        self.edges.push(Edge::new(node, weight));
    }

    pub fn has_edge(&self, id: &str) -> bool {
        for edge in self.edges.iter() {
            if edge.node().borrow().id() == id {
                return true;
            }
        }
        false
    }
}
