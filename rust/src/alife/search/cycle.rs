use super::problem::{Problem, ProblemResult};
use crate::{
    heuristic::{executors::jit::Jit, Heuristic},
    map::util::Map,
};
use rayon::prelude::*;

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

        ProblemCycle { problems }
    }

    pub fn len(&self) -> usize {
        self.problems.len()
    }

    pub fn get(&self, idx: usize) -> &Problem {
        &self.problems[idx]
    }
}

// #[derive(Clone)]
pub struct CycleSolver<'a> {
    map: &'a Map,
    heuristic: Heuristic,
    results: Vec<Option<ProblemResult>>,
    problems: ProblemCycle,
}

impl CycleSolver<'_> {
    pub fn new<'a>(map: &'a Map, heuristic: Heuristic, num_problems: usize) -> CycleSolver<'a> {
        let pcycle = ProblemCycle::new(map, num_problems);
        Self::from_cycle(pcycle, map, heuristic)
    }

    pub fn from_cycle<'a>(
        problems: ProblemCycle,
        map: &'a Map,
        heuristic: Heuristic,
    ) -> CycleSolver<'a> {
        CycleSolver {
            map,
            heuristic,
            results: vec![None; problems.len()],
            problems,
        }
    }

    pub fn solve_cycle(&mut self) -> Vec<ProblemResult> {
        let context = inkwell::context::Context::create();
        let executor = Jit::create(&self.heuristic, &context);

        // Parallel problem solving :)
        let raw = executor.get_raw().clone();
        self.results
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, result)| {
                if result.is_none() {
                    let problem = self.problems.get(idx);
                    *result = Some(
                        problem.solve(self.map, |sx, sy, gx, gy| unsafe { raw(sx, sy, gx, gy) }),
                    );
                }
            });

        self.results
            .clone()
            .into_iter()
            .map(|r| r.unwrap())
            .collect()
    }

    // pub fn solve_current(&mut self) -> ProblemResult {
    //     if self.results[self.problem_index].is_none() {
    //         let problem = self.problems.get(self.problem_index);

    //         self.results[self.problem_index] = Some(problem.solve(self.map, &self.h));
    //     };

    //     self.results[self.problem_index].clone().unwrap()
    // }

    // pub fn next_problem(&mut self) -> () {
    //     self.problem_index = (self.problem_index + 1) % self.problems.len();
    // }

    pub fn get_total_expansions_in_cycle(&self) -> usize {
        if self.results.clone().into_iter().any(|r| r.is_none()) {
            return usize::MAX;
        }

        self.results
            .clone()
            .into_iter()
            .map(|r| r.unwrap().expansions.len())
            .sum()
    }

    pub fn get_total_path_length_in_cycle(&self) -> usize {
        if self.results.clone().into_iter().any(|r| r.is_none()) {
            return usize::MAX;
        }

        self.results
            .clone()
            .into_iter()
            .map(|r| r.unwrap().solution_path.len())
            .sum()
    }
}
