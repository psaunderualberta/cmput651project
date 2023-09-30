use std::collections::HashMap;

use crate::alife::search::cycle::{CycleSolver, ProblemCycle};
use crate::constants::INITIAL_H_POPULATION_SIZE;
use crate::heuristic::parser::HeuristicNode;
use crate::heuristic::util::random_heuristic;
use crate::map::util::Map;

pub struct SimulationResult {
    pub heuristics: HashMap<HeuristicNode, usize>,
    pub best: HeuristicNode,
    pub score: usize,
}

pub struct Simulation<'a> {
    pub map: &'a Map,
    pub cycle: ProblemCycle,
    pub baseline: &'a CycleSolver<'a>,
    pub expansion_bound: f64,
    pub degredation_rate: f64,
    pub solvers: HashMap<i32, CycleSolver<'a>>,
    pub results: HashMap<HeuristicNode, usize>,
    pub limits: HashMap<i32, i32>,
}

impl Simulation<'_> {
    pub fn new<'a>(
        map: &'a Map,
        cycle: ProblemCycle,
        baseline: &'a CycleSolver,
        expansion_bound_multiple: f64,
        degredation_rate: f64,
        seed: Option<u64>,
    ) -> Simulation<'a> {
        if seed.is_some() {
            fastrand::seed(seed.unwrap());
        }

        // Get the initial starting bound
        let expansion_bound = expansion_bound_multiple * (baseline.get_expansions_in_single_cycle() as f64);


        Simulation {
            map,
            cycle,
            baseline,
            expansion_bound,
            degredation_rate,
            results: HashMap::new(),
            solvers: HashMap::new(),
            limits: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> SimulationResult {
        let mut heuristic_id = 0;

        // Create the initial population of solvers
        for _ in 0..INITIAL_H_POPULATION_SIZE {
            let h = random_heuristic(-1);
            let cycle = CycleSolver::from_cycle(self.cycle.clone(), self.map, h);
            self.solvers.insert(heuristic_id, cycle);
            heuristic_id += 1;
        }

        // While there are still some problems to solve
        while !self.solvers.is_empty() {
            // Exponential decay rate
            self.expansion_bound = self.expansion_bound * self.degredation_rate;

            let keys: Vec<i32> = self.solvers.keys().map(|x| *x).collect();

            // Iterate over the sets of solvers
            for key in keys {
                // Get the current solver
                let cur_solver = self.solvers.get_mut(&key).unwrap();

                // Solve one problem on this specific cycle
                let solve_result = cur_solver.solve_current();
                if solve_result.bound_exceeded {
                    // Record the # of expansions made by the heuristic within a single cycle
                    let exp_per_cycle = cur_solver.get_expansions_in_single_cycle();
                    let h = cur_solver.get_heuristic();
                    self.results.insert(h, exp_per_cycle);

                    // This cycle has no more mutations left
                    self.solvers.remove(&key);
                } else {
                    // Increment the solver to the next problem
                    cur_solver.next_problem();

                    // If we are able to perform a mutation, do it and add
                    // the new cycle + heuristic to the set of solvers
                    if cur_solver.able_to_mutate() {
                        let h = cur_solver.get_mutated_heuristic();
                        let new_cycle = CycleSolver::from_cycle(self.cycle.clone(), self.map, h);
                        self.solvers.insert(heuristic_id, new_cycle);
                        heuristic_id += 1;
                    }
                }
            }
        }

        // Get the best heuristic for quick reference when returned.
        let mut best_heuristic = random_heuristic(1);
        let mut best_exp_per_cycle = usize::MAX;
        for heuristic in self.results.keys() {
            let exp_per_cycle = *self.results.get(heuristic).unwrap();
            if exp_per_cycle < best_exp_per_cycle {
                best_exp_per_cycle = exp_per_cycle;
                best_heuristic = heuristic.clone();
            }
        }

        // Return the results of the simulation
        SimulationResult {
            heuristics: self.results.clone(),
            best: best_heuristic,
            score: best_exp_per_cycle,
        }
    }
}
