use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::graph::search::{Search, State, Status};
use crate::graph::Graph;

#[derive(Debug)]
pub struct BreadthFirstSearch {
    graph: Rc<Graph>,
    current: Rc<String>,
    dest: Rc<String>,
    discovered: HashSet<Rc<String>>,
    queue: VecDeque<(Rc<String>, (Rc<String>, f64))>,
    visited: HashMap<Rc<String>, (Rc<String>, f64)>,
    state: State,
}

// Associate functions
impl BreadthFirstSearch {
    pub fn new(graph: &Graph, start: &str, dest: &str) -> BreadthFirstSearch {
        let graph = Rc::new(graph.clone());
        let start = Rc::new(graph.nodes()[start].id().to_owned());
        let dest = Rc::new(graph.nodes()[dest].id().to_owned());
        let empty = Rc::new("".to_owned());
        let mut discovered = HashSet::new();
        discovered.insert(Rc::clone(&start));
        let mut queue = VecDeque::new();
        queue.push_back((start, (Rc::clone(&empty), 0.0)));

        BreadthFirstSearch {
            graph,
            current: Rc::clone(&empty),
            dest,
            discovered,
            queue,
            visited: HashMap::new(),
            state: State::Pop,
        }
    }
}

// Public methods
impl Search for BreadthFirstSearch {
    fn current(&self) -> &str {
        &self.current
    }

    fn visible(&self) -> Vec<(Rc<String>, (Rc<String>, f64))> {
        self.queue
            .iter()
            .map(|(s1, (s2, f))| (Rc::clone(s1), (Rc::clone(s2), *f)))
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
                let (id, (from, dist)) = if let Some(path) = self.queue.pop_front() {
                    path
                } else {
                    let status = Status::NotFound;
                    self.state = State::Done(status);

                    return status;
                };
                if *id == *self.dest {
                    self.visited.insert(id, (from, dist));
                    let status = Status::Found;
                    self.state = State::Done(status);

                    return status;
                }

                self.current = Rc::clone(&id);
                self.visited.insert(id, (from, dist));
                self.state = State::Push;
                return Status::Searching;
            }
            State::Push => {
                for (id, dist) in self.graph.nodes()[&*self.current].edges().iter() {
                    if self.discovered.contains(id) {
                        continue;
                    }
                    let id = Rc::new(id.clone());
                    self.discovered.insert(Rc::clone(&id));
                    self.queue.push_back((
                        Rc::clone(&id),
                        (
                            Rc::clone(&self.current),
                            *dist + self.visited[self.current.as_ref()].1,
                        ),
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
impl BreadthFirstSearch {
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
