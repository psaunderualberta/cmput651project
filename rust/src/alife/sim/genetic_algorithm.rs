use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::time::{Duration, Instant};

use crate::alife::search::cycle::{CycleSolver, ProblemCycle};

use crate::heuristic::mutator::mutate_heuristic;
use crate::heuristic::util::random_heuristic;
use crate::heuristic::Heuristic;
use crate::map::util::Map;

pub const MAX_POPULATION_SIZE: usize = 10000;
pub const MAX_BEST_INDIVIDUALS: usize = 10;

#[derive(Debug, Clone)]
pub struct Individual {
    pub heuristic: Heuristic,
    pub expansions: usize,
    pub path_len: usize,
}

impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic.root() == other.heuristic.root()
    }
}

impl Eq for Individual {}

impl Hash for Individual {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.heuristic.root().to_string().hash(state);
    }
}

impl Individual {
    fn fitness(&self, baseline_expansions: usize, baseline_path_len: usize) -> f64 {
        let expansion_ratio = self.expansions as f64 / baseline_expansions as f64;
        let path_len_ratio = self.path_len as f64 / baseline_path_len as f64;
        let size_weight = (20.0 + self.heuristic.size() as f64).sqrt();
        // let size_weight = 1.0;

        1.0 / (expansion_ratio * path_len_ratio * path_len_ratio * size_weight)
        // 1.0 / (1.0 * expansion_ratio + 2.0 * path_len_ratio + size_weight)
    }
}

pub struct GeneticAlgorithm<'a> {
    // The map used to perform the search
    pub map: &'a Map,
    // The problem cycle on which all heuristics will be evaluated
    pub cycle: ProblemCycle,
    // The results of solving the problem cycle with manhattan distance
    pub baseline: &'a CycleSolver<'a>,
    pub baseline_expansions: usize,
    pub baseline_path_len: usize,
    // The maximum number of expansions allowed per heuristic
    pub expansion_bound: usize,
    // The maximum amount of time allowed for the simulation
    pub time_limit: Duration,
    // max population size
    pub max_population_size: usize,
    pub population: HashSet<Individual>,
    pub popvec: Vec<Individual>,
    pub best_individuals: Vec<Individual>,
}

impl GeneticAlgorithm<'_> {
    pub fn new<'a>(
        map: &'a Map,
        cycle: ProblemCycle,
        baseline: &'a CycleSolver,
        expansion_bound: usize,
        time_limit: Duration,
        seed: Option<u64>,
        _verbose: bool,
    ) -> GeneticAlgorithm<'a> {
        // Seed the random number generator if a seed was provided
        if seed.is_some() {
            fastrand::seed(seed.unwrap());
        }

        GeneticAlgorithm {
            map,
            cycle,
            baseline,
            baseline_expansions: baseline.get_total_expansions_in_cycle(),
            baseline_path_len: baseline.get_total_path_length_in_cycle(),
            expansion_bound,
            time_limit,
            max_population_size: MAX_POPULATION_SIZE,
            population: HashSet::with_capacity(MAX_POPULATION_SIZE + 100),
            popvec: Vec::with_capacity(MAX_POPULATION_SIZE + 100),
            best_individuals: Vec::with_capacity(MAX_BEST_INDIVIDUALS + 1),
        }
    }

    pub fn run(&mut self) {
        // for i in 0..self.max_population_size {
        //     let h = random_heuristic(fastrand::i32(1..=7));
        //     let individual = self.add_individual(Heuristic { root: h });
        //     println!(
        //         "Generating initial heuristic #{}/{} with {:2.2}% expansions of baseline",
        //         i,
        //         self.max_population_size,
        //         100.0 * individual.result as f64 / self.baseline_expansions as f64
        //     );
        //     println!("Heuristic: {}", individual.heuristic.root);
        // }

        let timer = Instant::now();
        let mut iter_count = 0;
        let mut next_log = timer.elapsed() + Duration::from_secs(10);
        // while timer.elapsed() < self.time_limit {
        //     let h = random_heuristic(fastrand::i32(1..=7));
        //     let individual = self.add_individual(Heuristic { root: h });
        //     iter_count += 1;
        //     if timer.elapsed() > next_log {
        //         println!("\n### Best Heuristics ###\n");
        //         for individual in self.best_individuals.iter() {
        //             println!(
        //                 "Heuristic {:2.2}% expansions of baseline: {}",
        //                 100.0 * individual.result as f64 / self.baseline_expansions as f64,
        //                 individual.heuristic.root
        //             );
        //         }
        //         println!("\n Iterations per second: {}", iter_count as f64 / 100.0);
        //         iter_count = 0;
        //         next_log = timer.elapsed() + Duration::from_secs(10);
        //     }
        // }

        // println!("\n### Best Heuristics ###\n");
        // for individual in self.best_individuals.iter() {
        //     println!(
        //         "Heuristic {:2.2}% expansions of baseline: {}",
        //         100.0 * individual.result as f64 / self.baseline_expansions as f64,
        //         individual.heuristic.root
        //     );
        // }

        while timer.elapsed() < self.time_limit {
            let mut n = 0;
            if self.popvec.len() >= 2000 {
                n = 100;
            }
            let selected = self.prune_and_select_population(n);
            // for individual in selected.iter() {
            //     let mut heuristic = individual.heuristic.clone();
            //     heuristic.root = mutate_heuristic(&heuristic.root);
            //     self.add_heuristic(heuristic);
            //     // let h = random_heuristic(fastrand::i32(1..=7));
            //     // self.add_heuristic(Heuristic { root: h });
            // }
            let mutated: Vec<Heuristic> = selected
                .par_iter()
                .map(|individual| Heuristic::new(mutate_heuristic(individual.heuristic.root())))
                .collect();
            for heuristic in mutated {
                self.add_heuristic(heuristic);
            }
            for _ in 0..100 {
                let h = random_heuristic(fastrand::i32(1..=7));
                self.add_heuristic(Heuristic::new(h));
            }
            iter_count += 1;
            if timer.elapsed() > next_log {
                println!("\n### Best Heuristics ###\n");
                for individual in self.popvec.iter().take(10) {
                    println!(
                        "Heuristic {:2.2}% expansions of baseline, {:2.2}% path len of baseline: {}",
                        100.0 * individual.expansions as f64 / self.baseline_expansions as f64,
                        100.0 * individual.path_len as f64 / self.baseline_path_len as f64,
                        individual.heuristic.root()
                    );
                }
                println!("\n Iterations per second: {}", iter_count as f64 / 100.0);
                iter_count = 0;
                next_log = timer.elapsed() + Duration::from_secs(10);
            }
        }
    }

    fn add_heuristic(&mut self, heuristic: Heuristic) -> bool {
        let mut individual = Individual {
            heuristic,
            expansions: 0,
            path_len: 0,
        };
        if self.population.contains(&individual) {
            false
        } else {
            let mut cycle =
                CycleSolver::from_cycle(self.cycle.clone(), self.map, individual.heuristic.clone());
            cycle.solve_cycle();
            individual.expansions = cycle.get_total_expansions_in_cycle();
            individual.path_len = cycle.get_total_path_length_in_cycle();
            self.population.insert(individual.clone());
            self.popvec.push(individual);
            true
        }
    }

    fn prune_and_select_population(&mut self, n: usize) -> Vec<Individual> {
        // let mut popvec = self.population.iter().collect::<Vec<_>>();
        // sort largest to smallest
        self.popvec.sort_by(|a, b| {
            b.fitness(self.baseline_expansions, self.baseline_path_len)
                .partial_cmp(&a.fitness(self.baseline_expansions, self.baseline_path_len))
                .unwrap_or(Ordering::Equal)
        });
        // remove such that only self.max_population_size remain
        for individual in self.popvec.iter().skip(self.max_population_size) {
            self.population.remove(individual);
        }
        self.popvec.truncate(self.max_population_size);
        // select n random individuals weighted by result
        // let mut rng = fastrand::Rng::new();
        let mut selected: HashSet<usize> = HashSet::with_capacity(n);
        let mut sums = Vec::with_capacity(self.popvec.len());
        let mut sum = 0.0;
        for individual in self.popvec.iter() {
            sum += 1.0 / individual.fitness(self.baseline_expansions, self.baseline_path_len);
            sums.push(sum);
        }
        println!("sums len: {}, n: {}", sums.len(), n);
        while selected.len() < n {
            let r = fastrand::f64() * sum;
            for (i, sum) in sums.iter().enumerate() {
                if r < *sum {
                    selected.insert(i);
                    break;
                }
            }
        }
        selected
            .iter()
            .map(|i| self.popvec[*i].clone())
            .collect::<Vec<_>>()
    }

    // fn add_individual(&mut self, heuristic: Heuristic) -> Individual {
    //     // insert only if population does not already contain individual
    //     let mut individual = Individual {
    //         heuristic,
    //         expansions: 0,
    //         path_len: 0,
    //     };
    //     if self.population.contains(&individual) {
    //         self.population.get(&individual).unwrap().clone()
    //     } else {
    //         let mut cycle =
    //             CycleSolver::from_cycle(self.cycle.clone(), self.map, individual.heuristic.clone());
    //         cycle.solve_cycle();
    //         let result = cycle.get_total_expansions_in_cycle();
    //         individual.expansions = result;
    //         self.population.insert(individual.clone());
    //         for i in 0..(self.best_individuals.len() + 1) {
    //             if result
    //                 < match self.best_individuals.get(i) {
    //                     Some(ind) => ind.expansions,
    //                     None => usize::MAX,
    //                 }
    //             {
    //                 self.best_individuals.insert(i, individual.clone());
    //                 break;
    //             }
    //         }
    //         if self.best_individuals.len() > MAX_BEST_INDIVIDUALS {
    //             self.best_individuals.pop();
    //         }
    //         if self.population.len() > self.max_population_size {
    //             if let Some(worst) = self.population.iter().next() {
    //                 let mut worst = worst;
    //                 for individual in self.population.iter() {
    //                     if individual.expansions > worst.expansions {
    //                         worst = individual;
    //                     }
    //                 }
    //                 self.population.remove(&worst.clone());
    //             }
    //         }
    //         individual
    //     }
    // }
}
