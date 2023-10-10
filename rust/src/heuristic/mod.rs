pub mod executors;
pub mod mutator;
pub mod parser;
pub mod util;

use std::time::Instant;

use pyo3::prelude::*;

use parser::HeuristicNode;

#[derive(Debug, Clone, Eq, PartialEq)]
#[pyclass]
pub struct Heuristic {
    #[pyo3(get)]
    pub root: HeuristicNode,
    #[pyo3(get)]
    pub creation: Instant
}

impl Heuristic {
    pub fn new(root: HeuristicNode) -> Heuristic {
        Heuristic {
            root,
            creation: Instant::now()
        }
    }
}
