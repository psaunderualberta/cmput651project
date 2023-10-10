pub mod executors;
pub mod mutator;
pub mod parser;
pub mod util;

use std::time::{SystemTime, UNIX_EPOCH, Duration};

use pyo3::prelude::*;

use parser::HeuristicNode;

#[derive(Debug, Clone, Eq, PartialEq)]
#[pyclass]
pub struct Heuristic {
    pub root: HeuristicNode,
    pub creation: Duration
}

impl Heuristic {
    pub fn new(root: HeuristicNode) -> Heuristic {
        Heuristic {
            root,
            creation: SystemTime::now().duration_since(UNIX_EPOCH).expect("Backwards time??? :O")
        }
    }
}
