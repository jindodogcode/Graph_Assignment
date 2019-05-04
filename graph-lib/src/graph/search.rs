pub mod bfs;
pub mod dfs;
pub mod dijk;

use std::rc::Rc;

use std::fmt::{self, Display, Formatter};

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

pub trait Search {
    fn current(&self) -> &str;
    fn visible(&self) -> Vec<(Rc<String>, (Rc<String>, f64))>;
    fn visited(&self) -> Vec<(Rc<String>, (Rc<String>, f64))>;
    fn state(&self) -> State;
    fn next(&mut self) -> Status;
    fn result(&self) -> Option<Vec<(String, f64)>>;
}
