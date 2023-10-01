use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::alife::search::cycle::{CycleSolver, ProblemCycle};
use crate::constants::INITIAL_H_POPULATION_SIZE;
use crate::heuristic::mutator::mutate_heuristic;
use crate::heuristic::parser::HeuristicNode;
use crate::heuristic::util::random_heuristic;
use crate::map::util::Map;

use super::expansion_tracker::ExpansionTracker;

pub struct Simulation<'a> {
    pub map: &'a Map,
    pub cycle: ProblemCycle,
    pub baseline: &'a CycleSolver<'a>,
    pub expansion_bound: usize,
    pub time_limit: Duration,
    pub trackers: HashMap<i32, ExpansionTracker>,
    pub results: HashMap<HeuristicNode, f64>,
    pub limits: HashMap<i32, i32>,
    verbose: bool,
}

pub struct SimulationResult {
    pub heuristics: HashMap<HeuristicNode, f64>,
    pub best: HeuristicNode,
    pub score: f64,
}

impl Simulation<'_> {
    pub fn new<'a>(
        map: &'a Map,
        cycle: ProblemCycle,
        baseline: &'a CycleSolver,
        expansion_bound: usize,
        time_limit: Duration,
        seed: Option<u64>,
        verbose: bool,
    ) -> Simulation<'a> {
        if seed.is_some() {
            fastrand::seed(seed.unwrap());
        }

        Simulation {
            map,
            cycle,
            baseline,
            expansion_bound,
            time_limit,
            results: HashMap::new(),
            trackers: HashMap::new(),
            limits: HashMap::new(),
            verbose,
        }
    }

    pub fn run(&mut self) -> SimulationResult {
        let mut heuristic_id = 0;

        // Create the initial population of trackers
        for i in 0..INITIAL_H_POPULATION_SIZE {
            println!("{}", i);
            let h = random_heuristic(-1);
            let mut cycle = CycleSolver::from_cycle(self.cycle.clone(), self.map, h.clone());
            let results = cycle.solve_cycle();
            let tracker = ExpansionTracker::new(results, self.expansion_bound, h.clone());
            self.results.insert(h.clone(), tracker.get_expansion_average());
            self.trackers.insert(heuristic_id, tracker);
            heuristic_id += 1;
        }

        let timer = Instant::now();

        // While there are still some problems to solve
        let mut num_expansion_steps: usize = 0;
        while timer.elapsed() < self.time_limit {
            if self.verbose && num_expansion_steps % 100000 == 0 {
                println!("Elapsed Time: {:?}", timer.elapsed());
            }

            num_expansion_steps += 1;

            let keys: Vec<i32> = self.trackers.keys().map(|x| *x).collect();

            // Iterate over the sets of trackers
            for key in keys {
                // Get the current solver
                let cur_tracker = self.trackers.get_mut(&key).unwrap();

                // Perform one mimicked expansion
                cur_tracker.expand();
                if cur_tracker.expired() {
                    // This cycle has no more expansions left
                    self.trackers.remove(&key);
                } else {
                    // If we are able to perform a mutation, do it and add
                    // the new cycle + heuristic to the set of trackers
                    if cur_tracker.consume_mutation() {
                        if self.verbose {
                            println!("Mutating heuristic with id {}", key);
                        }

                        let h = cur_tracker.get_heuristic();
                        let h_mutated = mutate_heuristic(&h);
                        let mut new_cycle = CycleSolver::from_cycle(self.cycle.clone(), self.map, h_mutated.clone());
                        let results = new_cycle.solve_cycle();
                        let new_tracker = ExpansionTracker::new(results, self.expansion_bound, h_mutated);
                        self.trackers.insert(heuristic_id, new_tracker);
                        heuristic_id += 1;
                    }
                }
            }
        }

        println!("Num expansion steps: {}", num_expansion_steps);

        // Get the best heuristic for quick reference when returned.
        let mut best_heuristic = random_heuristic(1);
        let mut best_exp_per_cycle = f64::MAX;
        for heuristic in self.results.keys() {
            let exp_per_cycle = *self.results.get(heuristic).unwrap();
            if exp_per_cycle.min(best_exp_per_cycle) == exp_per_cycle {
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
