mod node;

use std::cell::RefCell;
use std::rc::Rc;

use node::Node;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AdjacencyList {
    nodes: Vec<Rc<RefCell<Node>>>,
}

#[allow(dead_code)]
impl AdjacencyList {
    pub fn new() -> AdjacencyList {
        AdjacencyList { nodes: Vec::new() }
    }

    pub fn with_capacity(size: usize) -> AdjacencyList {
        AdjacencyList {
            nodes: Vec::with_capacity(size),
        }
    }
}

#[allow(dead_code)]
impl AdjacencyList {
    pub fn nodes(&self) -> &Vec<Rc<RefCell<Node>>> {
        &self.nodes
    }

    pub fn contains(&self, id: &str) -> bool {
        for node in self.nodes.iter() {
            if node.borrow().id() == id {
                return true;
            }
        }
        false
    }

    pub fn get_node(&self, id: &str) -> Option<Rc<RefCell<Node>>> {
        for node in self.nodes.iter() {
            if node.borrow().id() == id {
                return Some(Rc::clone(&node));
            }
        }
        None
    }
}
