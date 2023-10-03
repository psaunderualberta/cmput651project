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
    // The map used to perform the search
    pub map: &'a Map,
    // The problem cycle on which all heuristics will be evaluated
    pub cycle: ProblemCycle,
    // The results of solving the problem cycle with manhattan distance
    pub baseline: &'a CycleSolver<'a>,
    // The maximum number of expansions allowed per heuristic
    pub expansion_bound: usize,
    // The maximum amount of time allowed for the simulation
    pub time_limit: Duration,
    // The set of trackers for each heuristic
    pub trackers: HashMap<i32, ExpansionTracker>,
    // The results of each heuristic (avg. expansions in single cycle)
    pub results: HashMap<HeuristicNode, f64>,
    // Whether or not to print verbose output
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

        // Seed the random number generator if a seed was provided
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

        // While we have not reached timeout and there are still heuristics to evaluate
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
                
                // If the tracker is expired, remove it from the set of trackers
                if cur_tracker.expired() {
                    if self.verbose {
                        println!("{key}: K");
                    }

                    // This cycle has no more expansions left. Kill it >:)
                    self.trackers.remove(&key);
                } else if cur_tracker.consume_mutation() {
                    // If we are able to perform a mutation, do it and add
                    // the new cycle + heuristic to the set of trackers
                
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

                    // Increment the heuristic identifier
                    heuristic_id += 1;

                    if self.verbose {
                        println!("{key}: M -> {heuristic_id}");
                    }
                }
            }
        }

        println!("Num expansion steps: {}", num_expansion_steps);

        // Get the best heuristic for quick reference when returned.
        let mut best_heuristic = random_heuristic(1);
        let mut best_exp_per_cycle = f64::MAX;
        for (heuristic, exp_per_cycle) in self.results.iter() {
            if exp_per_cycle.min(best_exp_per_cycle) == *exp_per_cycle {
                best_exp_per_cycle = *exp_per_cycle;
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
