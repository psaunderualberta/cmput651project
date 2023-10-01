use crate::{
    alife::search::problem::ProblemResult, constants::MUTATION_INTERVAL,
    heuristic::parser::HeuristicNode,
};

pub struct ExpansionTracker {
    pub results: Vec<ProblemResult>,
    pub total_expansions: usize,
    pub bound: usize,
    pub current_tracked_expansions: usize,
    pub problem_index: usize,
    pub heuristic: HeuristicNode,
    pub can_mutate: bool,
}

impl ExpansionTracker {
    pub fn new(results: Vec<ProblemResult>, bound: usize, heuristic: HeuristicNode) -> ExpansionTracker {
        ExpansionTracker {
            results,
            total_expansions: 0,
            current_tracked_expansions: 0,
            bound,
            problem_index: 0,
            heuristic,
            can_mutate: false,
        }
    }

    pub fn expand(&mut self) {
        if self.results[self.problem_index].expansions.len() == self.current_tracked_expansions {
            self.problem_index = (self.problem_index + 1) % self.results.len();
            self.can_mutate = self.problem_index % MUTATION_INTERVAL == 0;
            self.current_tracked_expansions = 0;
        }

        self.current_tracked_expansions += 1;
    }

    pub fn get_expansion_average(&self) -> f64 {
        self.results
            .iter()
            .fold(0.0, |acc, result| acc + result.expansions.len() as f64)
            / self.results.len() as f64
    }

    pub fn get_heuristic(&self) -> HeuristicNode {
        self.heuristic.clone()
    }

    pub fn consume_mutation(&mut self) -> bool {
        if self.can_mutate {
            self.can_mutate = false;
            return true;
        }

        false
    }

    pub fn expired(&self) -> bool {
        self.total_expansions >= self.bound
    }
}
