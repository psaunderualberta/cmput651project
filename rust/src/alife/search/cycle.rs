use crate::heuristic::mutator::mutate_heuristic;
use crate::{heuristic::parser::HeuristicNode, map::util::Map};
use crate::constants::MUTATION_INTERVAL;
use super::problem::{Problem, ProblemResult};

#[derive(Clone)]
pub struct ProblemCycle {
    pub problems: Vec<Problem>,
}

impl ProblemCycle {
    pub fn new(map: &Map, num_problems: usize) -> ProblemCycle {
        let mut problems = Vec::new();
        let original_start = map.random_free_position();
        let mut start = original_start;

        for _ in 0..num_problems - 1 {
            // Ensure the problem is not trivial
            let mut goal = map.random_free_position();
            while goal == start {
                goal = map.random_free_position();
            }

            problems.push(Problem { start, goal });
            start = goal;
        }

        // Push the final problem, to create an actual 'cycle'
        problems.push(Problem {
            start,
            goal: original_start,
        });

        ProblemCycle {
            problems,
        }
    }

    pub fn len(&self) -> usize {
        self.problems.len()
    }

    pub fn get(&self, idx: usize) -> &Problem {
        &self.problems[idx]
    }
}

#[derive(Clone)]
pub struct CycleSolver<'a> {
    map: &'a Map,
    h: HeuristicNode,
    results: Vec<Option<ProblemResult>>,
    problems: ProblemCycle,
    problem_index: usize,
    can_mutate: bool
}

impl CycleSolver<'_> {
    pub fn new<'a>(map: &'a Map, h: HeuristicNode, num_problems: usize) -> CycleSolver<'a> {
        let pcycle = ProblemCycle::new(map, num_problems);
        Self::from_cycle(pcycle, map, h)
    }

    pub fn from_cycle<'a>(problems: ProblemCycle, map: &'a Map, h: HeuristicNode) -> CycleSolver<'a> {
        CycleSolver {
            h,
            map, 
            results: vec![None; problems.len()],
            problem_index: 0,
            problems,
            can_mutate: false,
        }
    }

    pub fn solve_cycle(&mut self) -> () {
        let mut num_solved = 0;
        while num_solved != self.problems.len() {
            self.solve_current();
            self.next_problem();
            num_solved += 1;
        }
    }

    // Return value is whether the 'step' resulted in a problem being solved
    pub fn solve_current(&mut self) -> ProblemResult {
        if self.results[self.problem_index].is_none() {
            let problem = self.problems.get(self.problem_index);
            self.results[self.problem_index] = Some(problem.solve(self.map, &self.h));
        };

        self.results[self.problem_index].clone().unwrap()
    }

    pub fn next_problem(&mut self) -> () {
        self.problem_index = (self.problem_index + 1) % self.problems.len();
        if self.problem_index % MUTATION_INTERVAL == 0 {
            self.can_mutate = true;
        }
    }

    pub fn get_expansions_in_single_cycle(&self) -> usize {
        if self.results.clone().into_iter().any(|r| r.is_none()) {
            return 0;
        }

        self.results.clone().into_iter()
            .map(|r| r.unwrap().expansions.len())
            .sum()
    }

    pub fn get_mutated_heuristic(&mut self) -> HeuristicNode {
        if !self.can_mutate {
            panic!("Trying to mutate a heuristic {} when not possible!!!", &self.h);
        }

        self.can_mutate = false;
        mutate_heuristic(&self.h)
    }

    pub fn able_to_mutate(&self) -> bool {
        self.can_mutate
    }

    pub fn get_heuristic(&self) -> HeuristicNode {
        self.h.clone()
    }
}
