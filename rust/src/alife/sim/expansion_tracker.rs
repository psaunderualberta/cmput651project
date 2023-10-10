use std::cmp::Ordering;

use crate::{
    alife::search::problem::ProblemResult, constants::MUTATION_INTERVAL,
    heuristic::parser::HeuristicNode,
};

#[derive(Clone, Eq, PartialEq)]
pub struct ExpansionTracker {
    pub total_expansions: usize,
    pub bound: usize,
    pub current_problem_expansions: usize,
    pub expansions: Vec<usize>,
    pub problem_index: usize,
    pub heuristic: HeuristicNode,
    pub can_mutate: bool,
}

impl ExpansionTracker {
    pub fn new(results: Vec<ProblemResult>, bound: usize, heuristic: HeuristicNode) -> ExpansionTracker {
        let expansions: Vec<usize> = results.iter().map(|r| r.expansions.len()).collect();
        ExpansionTracker {
            total_expansions: 0,
            current_problem_expansions: expansions[0],
            expansions,
            bound,
            problem_index: 0,
            heuristic,
            can_mutate: false,
        }
    }

    pub fn get_heuristic_score(&self) -> f64 {
        self.expansions.iter().sum::<usize>() as f64 / self.expansions.len() as f64
    }

    pub fn get_current_num_expansions(&self) -> usize {
        self.current_problem_expansions
    }

    pub fn reduce_num_expansions(&mut self, expansions: usize) {
        self.current_problem_expansions -= expansions;
        self.total_expansions += expansions;
    }

    pub fn next_problem(&mut self) {
        self.problem_index = (self.problem_index + 1) % self.expansions.len();

        if self.problem_index % MUTATION_INTERVAL == 0 {
            self.can_mutate = true;
        }

        self.current_problem_expansions = self.expansions[self.problem_index];
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

// Impl ord for ExpansionTracker
// Lower current expansion number => higher priority
impl Ord for ExpansionTracker {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.current_problem_expansions < other.current_problem_expansions {
            true => Ordering::Greater,
            false => Ordering::Less,
        }
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for ExpansionTracker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
