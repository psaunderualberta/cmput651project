use pyo3::prelude::*;

use crate::heuristic::Heuristic;

#[derive(Debug, Clone)]
#[pyclass]
pub struct HeuristicResult {
    // The heuristic represented within this result
    #[pyo3(get)]
    pub heuristic: Heuristic,
    // The score of this heuristic
    #[pyo3(get)]
    pub score: f64,
}

impl HeuristicResult {
    pub fn less_than(&self, other: &Self) -> bool {
        self.score.min(other.score) == self.score
    }   
}
