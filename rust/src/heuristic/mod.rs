pub mod executors;
pub mod mutator;
pub mod parser;
pub mod util;

use pyo3::prelude::*;

use parser::HeuristicNode;

#[derive(Debug, Clone, Eq, PartialEq)]
#[pyclass]
pub struct Heuristic {
    root: HeuristicNode,
    size: usize,
}

impl Heuristic {
    pub fn new(root: HeuristicNode) -> Heuristic {
        let size = heuristic_node_size(root.clone());
        Heuristic { root, size }
    }

    pub fn root(&self) -> &HeuristicNode {
        &self.root
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

fn heuristic_node_size(node: HeuristicNode) -> usize {
    match node {
        HeuristicNode::Number(_) => 1,
        HeuristicNode::Terminal(_) => 1,
        HeuristicNode::Unary(_, h) => 1 + heuristic_node_size(*h),
        HeuristicNode::Binary(_, h1, h2) => 1 + heuristic_node_size(*h1) + heuristic_node_size(*h2),
    }
}
