use ordered_float::NotNan;

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::graph::search::{Search, State, Status};
use crate::graph::Graph;

#[derive(Debug, Eq)]
struct Edge {
    id: Rc<String>,
    from: Rc<String>,
    dist: Reverse<NotNan<f64>>,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.dist == other.dist
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.dist.cmp(&other.dist))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Edge {
    fn new(id: Rc<String>, from: Rc<String>, dist: f64) -> Edge {
        let dist = Reverse(NotNan::new(dist).unwrap());
        Edge { id, from, dist }
    }
}

#[derive(Debug)]
pub struct ShortestPath {
    graph: Rc<Graph>,
    current: Rc<String>,
    dest: Rc<String>,
    queue: BinaryHeap<Edge>,
    visited: HashMap<Rc<String>, (Rc<String>, f64)>,
    state: State,
}

// Associate functions
impl ShortestPath {
    pub fn new(graph: &Graph, start: &str, dest: &str) -> ShortestPath {
        let graph = Rc::new(graph.clone());
        let start = Rc::new(graph.nodes()[start].id().to_owned());
        let dest = Rc::new(graph.nodes()[dest].id().to_owned());
        let empty = Rc::new("".to_owned());
        let mut queue = BinaryHeap::new();
        queue.push(Edge::new(start, Rc::clone(&empty), 0.0));

        ShortestPath {
            graph,
            current: Rc::clone(&empty),
            dest,
            queue,
            visited: HashMap::new(),
            state: State::Pop,
        }
    }
}

// Public methods
impl Search for ShortestPath {
    fn current(&self) -> &str {
        &self.current
    }

    fn visible(&self) -> Vec<(Rc<String>, (Rc<String>, f64))> {
        self.queue
            .iter()
            .map(|Edge { id, from, dist }| (Rc::clone(id), (Rc::clone(from), *dist.0)))
            .collect()
    }

    fn visited(&self) -> Vec<(Rc<String>, (Rc<String>, f64))> {
        self.visited
            .iter()
            .map(|(s1, (s2, f))| (Rc::clone(s1), (Rc::clone(s2), *f)))
            .collect()
    }

    fn state(&self) -> State {
        self.state
    }

    fn next(&mut self) -> Status {
        match self.state {
            State::Pop => {
                let Edge { id, from, dist } = if let Some(path) = self.queue.pop() {
                    path
                } else {
                    let status = Status::NotFound;
                    self.state = State::Done(status);

                    return status;
                };
                if *id == *self.dest {
                    self.visited.insert(id, (from, *dist.0));
                    let status = Status::Found;
                    self.state = State::Done(status);

                    return status;
                }

                self.current = Rc::clone(&id);

                if self.visited.contains_key(&id) && self.visited[&id].1 < *dist.0 {
                    return Status::Searching;
                }

                self.visited.insert(id, (from, *dist.0));
                self.state = State::Push;
                return Status::Searching;
            }
            State::Push => {
                for (id, dist) in self.graph.nodes()[&*self.current].edges().iter() {
                    let id = Rc::new(id.clone());

                    if self.visited.contains_key(id.as_ref()) && self.visited[id.as_ref()].1 < *dist
                    {
                        continue;
                    }

                    self.queue.push(Edge::new(
                        id,
                        Rc::clone(&self.current),
                        *dist + self.visited[self.current.as_ref()].1,
                    ));
                }

                self.state = State::Pop;
                return Status::Searching;
            }
            State::Done(status) => {
                return status;
            }
        }
    }

    fn result(&self) -> Option<Vec<(String, f64)>> {
        match &self.state {
            State::Done(Status::Found) => Some(self.make_path()),
            _ => None,
        }
    }
}

// Private methods
impl ShortestPath {
    fn make_path(&self) -> Vec<(String, f64)> {
        let mut path: Vec<(String, f64)> = Vec::new();

        let mut id = Rc::clone(&self.dest);
        loop {
            let (prev, dist) = &self.visited[&id];

            path.push(((*id).clone(), *dist));
            id = Rc::clone(prev);

            if prev.is_empty() {
                break;
            }
        }

        path.into_iter().rev().collect()
    }
}
