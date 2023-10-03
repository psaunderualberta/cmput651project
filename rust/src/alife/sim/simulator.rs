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
    // Mapping from each heuristic to the average # of expansions per cycle
    pub heuristics: HashMap<HeuristicNode, f64>,

    // The best heuristic found in terms of expansions per cycle
    pub best: HeuristicNode,

    // The score of the best heuristic
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
            self.results
                .insert(h.clone(), tracker.get_expansion_average());
            self.trackers.insert(heuristic_id, tracker);
            heuristic_id += 1;
        }

        let mut num_expansion_steps: usize = 0;
        let timer = Instant::now();

        // While we have not reached timeout
        while self.trackers.keys().len() != 0 && timer.elapsed() < self.time_limit {
            if self.verbose && num_expansion_steps % 100000 == 0 {
                println!("-1: {:?}", timer.elapsed());
            }

            // Increment the number of expansion steps
            num_expansion_steps += 1;

            // Iterate over the sets of trackers
            let keys: Vec<i32> = self.trackers.keys().map(|x| *x).collect();
            for key in keys {
                // Get the current solver
                let cur_tracker = self.trackers.get_mut(&key).unwrap();

                // Ensure not expired
                assert!(!cur_tracker.expired());

                // Perform one mimicked expansion of a state
                cur_tracker.expand();
                if cur_tracker.expired() {
                    if self.verbose {
                        println!("{key}: K");
                    }

                    // This cycle has no more expansions left. Kill it >:)
                    self.trackers.remove(&key);
                } else {
                    // If we are able to perform a mutation, do it and add
                    // the new cycle + heuristic to the set of trackers
                    if cur_tracker.consume_mutation() {
                        let h = cur_tracker.get_heuristic();
                        let h_mutated = mutate_heuristic(&h);
                        let results = CycleSolver::from_cycle(
                            self.cycle.clone(),
                            self.map,
                            h_mutated.clone(),
                        )
                        .solve_cycle();
                        let new_tracker =
                            ExpansionTracker::new(results, self.expansion_bound, h_mutated.clone());

                        // Insert performance of this heuristic in the results hashmap
                        self.results.insert(h_mutated, new_tracker.get_expansion_average());

                        // Insert the new tracker into the trackers hashmap
                        self.trackers.insert(heuristic_id, new_tracker);

                        if self.verbose {
                            println!("{key}: M -> {heuristic_id}");
                        }

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
