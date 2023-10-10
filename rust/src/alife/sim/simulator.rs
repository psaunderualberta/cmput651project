use std::time::{Duration, Instant};

use priority_queue::PriorityQueue;
use pyo3::prelude::*;

use crate::alife::search::cycle::{CycleSolver, ProblemCycle};
use crate::alife::sim::heuristic_result::HeuristicResult;
use crate::constants::INITIAL_H_POPULATION_SIZE;
use crate::heuristic::mutator::mutate_heuristic;
use crate::heuristic::parser::parse_heuristic;
use crate::heuristic::util::random_heuristic;
use crate::heuristic::Heuristic;
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
    pub trackers: PriorityQueue<i32, ExpansionTracker>,
    // The results of each heuristic (avg. expansions in single cycle)
    pub results: Vec<HeuristicResult>,
    // Whether or not to print verbose output
    verbose: bool,
}

#[pyclass]
pub struct SimulationResult {
    // Mapping from each heuristic to its score for the cycle
    #[pyo3(get)]
    pub heuristics: Vec<HeuristicResult>,
    // The best heuristic found in terms of expansions per cycle
    #[pyo3(get)]
    pub best: HeuristicResult,
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

        let sim = Simulation {
            map,
            cycle,
            baseline,
            expansion_bound,
            time_limit,
            results: Vec::new(),
            trackers: PriorityQueue::new(),
            verbose,
        };

        if seed.is_some() {
            fastrand::seed(seed.unwrap());
        }

        sim
    }

    pub fn run(&mut self) -> SimulationResult {
        let mut heuristic_id = 0;

        // Create a default heuristic result, to be overwritten as soon as possible
        let mut best_heuristic = HeuristicResult {
            heuristic: parse_heuristic("(+ deltaX deltaY)").root.to_string(),
            creation: 0,
            score: f64::MAX,
        };

        // Create the initial population of trackers
        for i in 0..INITIAL_H_POPULATION_SIZE {
            if self.verbose {
                println!("Seeding heuristic #{}", i);
            }
            let h = Heuristic::new(random_heuristic(-1));
            let mut cycle = CycleSolver::from_cycle(self.cycle.clone(), self.map, h.clone());

            let results = cycle.solve_cycle();
            let tracker = ExpansionTracker::new(results, self.expansion_bound, h.clone());

            // Get heuristic result, update best heuristic if necessary
            let heuristic_result = tracker.get_heuristic_result();
            if best_heuristic.worse_than(&heuristic_result) {
                best_heuristic = heuristic_result.clone();
            }

            self.results.push(heuristic_result);
            self.trackers.push(heuristic_id, tracker);

            heuristic_id += 1;
        }

        let mut num_expansion_steps: u64 = 0;
        let timer = Instant::now();

        // While we have not reached timeout and there are still heuristics to evaluate
        while self.trackers.len() != 0 && timer.elapsed() < self.time_limit {
            if self.verbose && num_expansion_steps % 100000 == 0 {
                println!("Elapsed time: {:?}", timer.elapsed());
            }

            // Increment the number of expansion steps
            num_expansion_steps += 1;

            // Get the cycle with the shortest current problem length
            let (key, mut tracker) = self.trackers.pop().unwrap();
            let executed_expansions = tracker.get_current_num_expansions();
            tracker.next_problem();

            // Create a vector of heuristics to mutate
            let mut new_heuristics = Vec::new();
            if tracker.consume_mutation() {
                new_heuristics.push(tracker.get_heuristic());
            }

            // Iterate over the entire priority queue, subtracting the previous expansions
            for (_, other_tracker) in self.trackers.iter_mut() {
                other_tracker.reduce_num_expansions(executed_expansions);

                // Edge case where multiple trackers had minimal problem length
                if other_tracker.get_current_num_expansions() == 0 {
                    other_tracker.next_problem();
                }

                // Add a heuristic to the set of heuristics to be mutated
                if other_tracker.consume_mutation() {
                    let new_h = other_tracker.get_heuristic();
                    new_heuristics.push(new_h);
                }
            }

            // Mutate the heuristics to be mutated
            for h in new_heuristics.into_iter() {
                let new_h = mutate_heuristic(&h.root);
                let heuristic = Heuristic::new(new_h);
                let results =
                    CycleSolver::from_cycle(self.cycle.clone(), self.map, heuristic.clone())
                        .solve_cycle();
                let new_tracker =
                    ExpansionTracker::new(results, self.expansion_bound, heuristic.clone());

                // Get heuristic result, update best if necessary
                let heuristic_result = tracker.get_heuristic_result();
                if best_heuristic.worse_than(&heuristic_result) {
                    best_heuristic = heuristic_result.clone();

                    if self.verbose {
                        println!("New best heuristic: {}\n-> {:.4}", best_heuristic.heuristic, best_heuristic.score);
                    }
                }

                // Insert performance of this heuristic in the results hashmap
                self.results.push(heuristic_result);

                // Insert the new tracker into the trackers hashmap
                self.trackers.push(heuristic_id, new_tracker);

                // Increment the heuristic identifier
                heuristic_id += 1;
            }

            if !tracker.expired() {
                self.trackers.push(key, tracker);
            }
        }

        println!("Num expansion steps: {}", num_expansion_steps);

        // Return the results of the simulation
        SimulationResult {
            heuristics: self.results.clone(),
            best: best_heuristic.clone(),
        }
    }
}

// // Iterate over the sets of trackers
// let keys: Vec<i32> = self.trackers.keys().map(|x| *x).collect();
// for key in keys {
//     // Get the current solver
//     let cur_tracker = self.trackers.get_mut(&key).unwrap();

//     // Ensure not expired
//     assert!(!cur_tracker.expired());

//     // Perform one mimicked expansion of a state
//     cur_tracker.expand();

//     // If the tracker is expired, remove it from the set of trackers
//     if cur_tracker.expired() {
//         if self.verbose {
//             println!("{key}: K");
//         }

//         // This cycle has no more expansions left. Kill it >:)
//         self.trackers.remove(&key);
//     } else if cur_tracker.consume_mutation() {
//         // If we are able to perform a mutation, do it and add
//         // the new cycle + heuristic to the set of trackers

//         let h = cur_tracker.get_heuristic();
//         let h_mutated = mutate_heuristic(&h);
//         let results = CycleSolver::from_cycle(
//             self.cycle.clone(),
//             self.map,
//             Heuristic { root: h.clone() },
//         )
//         .solve_cycle();
//         let new_tracker =
//             ExpansionTracker::new(results, self.expansion_bound, h_mutated.clone());

//         // Insert performance of this heuristic in the results hashmap
//         self.results
//             .insert(h_mutated, new_tracker.get_expansion_average());

//         // Insert the new tracker into the trackers hashmap
//         self.trackers.insert(heuristic_id, new_tracker);

//         // Increment the heuristic identifier
//         heuristic_id += 1;

//         if self.verbose {
//             println!("{key}: M -> {heuristic_id}");
//         }
//     }
// }
