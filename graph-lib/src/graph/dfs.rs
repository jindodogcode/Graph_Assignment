use std::collections::HashMap;
use std::collections::HashSet;

use crate::graph::Graph;

#[derive(Debug, Copy, Clone)]
pub enum Status {
    Found,
    NotFound,
    Searching,
}

#[derive(Debug, Copy, Clone)]
pub enum State {
    Pop,
    Push,
    Done(Status),
}

impl State {}

#[derive(Debug)]
pub struct DepthFirstSearch<'a> {
    graph: &'a Graph,
    current: &'a str,
    dest: &'a str,
    discovered: HashSet<String>,
    stack: Vec<(&'a str, (&'a str, f64))>,
    visited: HashMap<&'a str, (&'a str, f64)>,
    state: State,
}

// Associate functions
impl<'a> DepthFirstSearch<'a> {
    pub fn new(graph: &'a Graph, start: &'a str, dest: &'a str) -> DepthFirstSearch<'a> {
        let mut discovered = HashSet::new();
        discovered.insert(start.to_owned());
        let mut stack = Vec::new();
        stack.push((start, ("", 0.0)));

        DepthFirstSearch {
            graph,
            current: "",
            dest,
            discovered,
            stack,
            visited: HashMap::new(),
            state: State::Pop,
        }
    }
}

// Public methods
impl<'a> DepthFirstSearch<'a> {
    pub fn current(&self) -> &str {
        self.current
    }

    pub fn stack(&self) -> &Vec<(&str, (&str, f64))> {
        &self.stack
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
                let (id, (from, dist)) = if let Some(path) = self.stack.pop() {
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
                    if self.discovered.contains(id) {
                        continue;
                    }
                    self.discovered.insert(id.to_owned());
                    self.stack.push((id, (self.current, *dist)));
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
impl<'a> DepthFirstSearch<'a> {
    fn make_path(&self) -> Vec<(String, f64)> {
        let mut path: Vec<(String, f64)> = Vec::new();

        let mut id = self.dest;
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
