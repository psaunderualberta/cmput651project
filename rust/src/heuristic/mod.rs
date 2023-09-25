pub mod executors;
pub mod mutator;
pub mod parser;
pub mod util;

use parser::HeuristicNode;

pub struct Heuristic {
    pub root: HeuristicNode,
}
