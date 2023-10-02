pub mod executors;
pub mod mutator;
pub mod parser;
pub mod util;

use pyo3::prelude::*;

use parser::HeuristicNode;

#[pyclass]
pub struct Heuristic {
    pub root: HeuristicNode,
}
