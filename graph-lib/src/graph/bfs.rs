use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;

use crate::graph::Graph;

#[derive(Debug, Copy, Clone)]
pub enum Status {
    Found,
    NotFound,
    Searching,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Status::Found => write!(f, "Found"),
            Status::NotFound => write!(f, "Not Found"),
            Status::Searching => write!(f, "Searching"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum State {
    Pop,
    Push,
    Done(Status),
}

#[derive(Debug)]
pub struct BreadthFirstSearch<'a> {
    graph: &'a Graph,
    current: &'a str,
    dest: &'a str,
    discovered: HashSet<&'a str>,
    queue: VecDeque<(&'a str, (&'a str, f64))>,
    visited: HashMap<&'a str, (&'a str, f64)>,
    state: State,
}

// Associate functions
impl<'a> BreadthFirstSearch<'a> {
    pub fn new(graph: &'a Graph, start: &str, dest: &str) -> BreadthFirstSearch<'a> {
        let start = graph.nodes()[start].id();
        let dest = graph.nodes()[dest].id();
        let mut discovered = HashSet::new();
        discovered.insert(start);
        let mut queue = VecDeque::new();
        queue.push_back((start, ("", 0.0)));

        BreadthFirstSearch {
            graph,
            current: "",
            dest: dest,
            discovered,
            queue,
            visited: HashMap::new(),
            state: State::Pop,
        }
    }
}

// Public methods
impl<'a> BreadthFirstSearch<'a> {
    pub fn current(&self) -> &str {
        self.current
    }

    pub fn queue(&self) -> &VecDeque<(&str, (&str, f64))> {
        &self.queue
    }

    pub fn visited(&self) -> &HashMap<&str, (&str, f64)> {
        &self.visited
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn next(&mut self) -> Status {
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

                self.current = id;
                self.visited.insert(id, (from, dist));
                self.state = State::Push;
                return Status::Searching;
            }
            State::Push => {
                for (id, dist) in self.graph.nodes()[self.current].edges().iter() {
                    if self.discovered.contains(&id.as_ref()) {
                        continue;
                    }
                    self.discovered.insert(id.as_ref());
                    self.queue.push_back((id, (self.current, *dist)));
                }

                self.state = State::Pop;
                return Status::Searching;
            }
            State::Done(status) => {
                return status;
            }
        }
    }

    pub fn result(&self) -> Option<Vec<(String, f64)>> {
        match &self.state {
            State::Done(Status::Found) => Some(self.make_path()),
            _ => None,
        }
    }
}

// Private methods
impl<'a> BreadthFirstSearch<'a> {
    fn make_path(&self) -> Vec<(String, f64)> {
        let mut path: Vec<(String, f64)> = Vec::new();

        let mut id: &str = &self.dest;
        loop {
            let (prev, dist) = self.visited[id];

            path.push((id.to_owned(), dist));
            id = prev;

            if prev == "" {
                break;
            }
        }

        path.into_iter().rev().collect()
    }
}

#[derive(Debug)]
pub struct RcBreadthFirstSearch {
    graph: Rc<Graph>,
    current: Rc<String>,
    dest: Rc<String>,
    discovered: HashSet<Rc<String>>,
    queue: VecDeque<(Rc<String>, (Rc<String>, f64))>,
    visited: HashMap<Rc<String>, (Rc<String>, f64)>,
    state: State,
}

// Associate functions
impl RcBreadthFirstSearch {
    pub fn new(graph: Rc<Graph>, start: &str, dest: &str) -> RcBreadthFirstSearch {
        let start = Rc::new(graph.nodes()[start].id().to_owned());
        let dest = Rc::new(graph.nodes()[dest].id().to_owned());
        let empty = Rc::new("".to_owned());
        let mut discovered = HashSet::new();
        discovered.insert(Rc::clone(&start));
        let mut queue = VecDeque::new();
        queue.push_back((start, (Rc::clone(&empty), 0.0)));

        RcBreadthFirstSearch {
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
impl RcBreadthFirstSearch {
    pub fn current(&self) -> &str {
        &self.current
    }

    pub fn queue(&self) -> &VecDeque<(Rc<String>, (Rc<String>, f64))> {
        &self.queue
    }

    pub fn visited(&self) -> &HashMap<Rc<String>, (Rc<String>, f64)> {
        &self.visited
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn next(&mut self) -> Status {
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
                    self.queue
                        .push_back((Rc::clone(&id), (Rc::clone(&self.current), *dist)));
                }

                self.state = State::Pop;
                return Status::Searching;
            }
            State::Done(status) => {
                return status;
            }
        }
    }

    pub fn result(&self) -> Option<Vec<(String, f64)>> {
        match &self.state {
            State::Done(Status::Found) => Some(self.make_path()),
            _ => None,
        }
    }
}

// Private methods
impl RcBreadthFirstSearch {
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
