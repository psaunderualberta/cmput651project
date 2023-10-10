use pyo3::prelude::*;

#[derive(Debug, Clone, Default)]
#[pyclass]
pub struct HeuristicResult {
    // The heuristic represented within this result
    #[pyo3(get)]
    pub heuristic: String,
    // The time of creation for this heuristic
    #[pyo3(get)]
    pub creation: u128,
    // The score of this heuristic
    #[pyo3(get)]
    pub score: f64,
}

impl HeuristicResult {
    pub fn worse_than(&self, other: &Self) -> bool {
        self.score.gt(&other.score)
    }   
}
