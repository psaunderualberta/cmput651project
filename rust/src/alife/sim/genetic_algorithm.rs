use pyo3::{pyclass, pymethods};
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::time::{Duration, Instant, SystemTime};

use crate::alife::search::cycle::{CycleSolver, ProblemCycle};
use crate::constants::MAX_TREE_SIZE;
use crate::heuristic::mutate_probs::TermProbabilities;
use crate::heuristic::mutator::mutate_heuristic;
use crate::heuristic::util::random_heuristic;
use crate::heuristic::util::{normalize_vector, random_weighted_sample};
use crate::heuristic::Heuristic;
use crate::map::util::Map;

// 10,000 is WAYYYYY too many. Decreased MAX_POPULATION_SIZE to 40
pub const MAX_POPULATION_SIZE: usize = 40;
pub const MAX_BEST_INDIVIDUALS: usize = 10;

#[derive(Debug, Clone)]
#[pyclass]
pub struct GeneticAlgorithmResult {
    #[pyo3(get)]
    pub best_heuristics: Vec<String>,
    #[pyo3(get)]
    pub best_fitnesses: Vec<f64>,
    #[pyo3(get)]
    pub history: Vec<Vec<(String, f64, u64)>>,
}

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
        let path_len_ratio = self.path_len as f64 / baseline_path_len as f64;
        let expansion_ratio = self.expansions as f64 / baseline_expansions as f64;
        let size_weight = 200.0 + self.heuristic.size() as f64;

        path_len_ratio.powi(2) * expansion_ratio * size_weight
    }
}

#[pyclass]
pub struct GeneticAlgorithm {
    // The map used to perform the search
    pub map: Map,
    // The problem cycle on which all heuristics will be evaluated
    pub cycle: ProblemCycle,
    // The results of solving the problem cycle with manhattan distance
    pub baseline: CycleSolver,
    pub baseline_expansions: usize,
    pub baseline_path_len: usize,
    // The maximum number of expansions allowed per heuristic
    pub expansion_bound: usize,
    // The maximum amount of time allowed for the simulation
    pub time_limit: Duration,
    // max population size
    pub max_population_size: usize,
    pub h_population: Vec<Heuristic>,
    pub i_population: Vec<Individual>,
    pub best_individuals: Vec<Individual>,
    pub term_probs: Option<TermProbabilities>,
}

impl GeneticAlgorithm {
    pub fn new(
        map: Map,
        cycle: ProblemCycle,
        baseline: CycleSolver,
        expansion_bound: usize,
        time_limit: Duration,
        term_probs: Option<TermProbabilities>,
        seed: Option<u64>,
        _verbose: bool,
    ) -> GeneticAlgorithm {
        // Seed the random number generator if a seed was provided
        if seed.is_some() {
            fastrand::seed(seed.unwrap());
        }

        GeneticAlgorithm {
            map,
            cycle,
            baseline: baseline.clone(),
            baseline_expansions: baseline.get_total_expansions_in_cycle(),
            baseline_path_len: baseline.get_total_path_length_in_cycle(),
            expansion_bound,
            time_limit,
            max_population_size: MAX_POPULATION_SIZE,
            h_population: Vec::with_capacity(MAX_POPULATION_SIZE),
            i_population: Vec::with_capacity(MAX_POPULATION_SIZE),
            best_individuals: Vec::with_capacity(MAX_BEST_INDIVIDUALS + 1),
            term_probs,
        }
    }

    pub fn run(&mut self) -> GeneticAlgorithmResult {
        let mut history = Vec::new();

        for _ in 0..self.max_population_size {
            let h = random_heuristic(fastrand::i32(1..=MAX_TREE_SIZE), &self.term_probs);
            self.h_population.push(Heuristic::new(h));
        }

        let timer = Instant::now();
        let mut iter_count = 0;
        let mut next_log = timer.elapsed() + Duration::from_secs(10);

        let mut generation_number = 0;
        while timer.elapsed() < self.time_limit {
            // Update the generation number
            generation_number += 1;

            // Solve the problem cycle with each heuristic in the population
            self.i_population = self
                .h_population
                .iter()
                .map(|heuristic| self.compute_individual(heuristic.clone()))
                .collect();

            // Get the current unix time
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            // Add the current population to the history vector
            history.push(
                self.i_population
                    .iter()
                    .map(|individual| {
                        (
                            individual.heuristic.root().to_string(),
                            individual.fitness(self.baseline_expansions, self.baseline_path_len),
                            now,
                        )
                    })
                    .collect::<Vec<_>>(),
            );

            // Update the best individuals
            self.best_individuals
                .extend(self.i_population.clone().into_iter());
            self.best_individuals.sort_by(|a, b| {
                let a_fitness = a.fitness(self.baseline_expansions, self.baseline_path_len);
                let b_fitness = b.fitness(self.baseline_expansions, self.baseline_path_len);
                a_fitness.partial_cmp(&b_fitness).unwrap_or(Ordering::Equal)
            });
            self.best_individuals.truncate(MAX_BEST_INDIVIDUALS);

            // Get the individuals in the next population
            let next_population = self.get_next_population();
            self.h_population = next_population
                .par_iter()
                .map(|heuristic| {
                    Heuristic::new(mutate_heuristic(heuristic.root(), &self.term_probs))
                })
                .collect();
        }

        println!("{}", generation_number);

        GeneticAlgorithmResult {
            best_heuristics: self
                .best_individuals
                .iter()
                .map(|i| i.heuristic.root().to_string())
                .collect(),
            best_fitnesses: self
                .best_individuals
                .iter()
                .map(|i| i.fitness(self.baseline_expansions, self.baseline_path_len))
                .collect(),
            history,
        }
    }

    fn compute_individual(&self, heuristic: Heuristic) -> Individual {
        let mut cycle =
            CycleSolver::from_cycle(self.cycle.clone(), self.map.clone(), heuristic.clone());
        cycle.solve_cycle();
        Individual {
            heuristic,
            expansions: cycle.get_total_expansions_in_cycle(),
            path_len: cycle.get_total_path_length_in_cycle(),
        }
    }

    fn get_next_population(&self) -> Vec<Heuristic> {
        let mut selected = Vec::with_capacity(MAX_POPULATION_SIZE);

        // Get the fitnesses in the current population
        let mut weights = self
            .i_population
            .iter()
            .map(|i| i.fitness(self.baseline_expansions, self.baseline_path_len))
            .collect::<Vec<_>>();

        // Normalize the weights and select n random individuals according to the weights
        normalize_vector(&mut weights);
        while selected.len() < MAX_POPULATION_SIZE {
            selected
                .push(random_weighted_sample::<Heuristic>(&weights, &self.h_population).clone());
        }

        // return the selected individuals
        selected
    }

    fn select_n_individuals(&self, n: usize, weights: &Vec<f64>) -> Vec<Individual> {
        let mut selected = Vec::with_capacity(n);

        while selected.len() < n {
            selected.push(
                random_weighted_sample::<Individual>(weights, &self.best_individuals).clone(),
            );
        }

        // return the selected individuals
        selected
    }
}

#[pymethods]
impl GeneticAlgorithm {
    pub fn initialize_ga(&mut self) {
        let mut h_population: Vec<Heuristic> = Vec::with_capacity(1000);
        for _ in 0..900 {
            let h = random_heuristic(fastrand::i32(1..=7), &self.term_probs);
            h_population.push(Heuristic::new(h));
        }
        self.best_individuals = h_population
            .par_iter()
            .map(|heuristic| self.compute_individual(heuristic.clone()))
            .collect();
    }

    pub fn step_with_probs(
        &mut self,
        probs: Vec<TermProbabilities>,
    ) -> (Vec<(String, f64)>, Vec<f64>) {
        let timer = Instant::now();

        let mut prob_performance: Vec<f64> = vec![0.0; probs.len()];

        let rounds = 1;
        let mutations_per_prob = 100;

        for _ in 0..rounds {
            // Get the fitnesses in the current population
            let mut weights = self
                .best_individuals
                .iter()
                .map(|i| 1.0 / i.fitness(self.baseline_expansions, self.baseline_path_len))
                .collect::<Vec<_>>();

            // add 100 random individuals
            self.best_individuals.extend(
                (0..100)
                    // .into_par_iter()
                    .map(|_| {
                        let h = random_heuristic(fastrand::i32(1..=10), &self.term_probs);
                        self.compute_individual(Heuristic::new(h))
                    })
                    .collect::<Vec<_>>(),
            );

            // get average weight
            let avg_weight = weights.iter().sum::<f64>() / weights.len() as f64;
            // add 100 average weights
            for _ in 0..100 {
                weights.push(avg_weight);
            }

            // Normalize the weights and select n random individuals according to the weights
            normalize_vector(&mut weights);

            // Get the individuals in the next population
            let next_population =
                self.select_n_individuals(probs.len() * mutations_per_prob, &weights);

            let before = (&next_population)
                .iter()
                // .map(|i| self.baseline_expansions as f64 / i.expansions as f64)
                .map(|i| i.fitness(self.baseline_expansions, self.baseline_path_len))
                .collect::<Vec<_>>();

            // let h_population: Vec<Heuristic> = Vec::with_capacity(probs.len() * 10);

            // for p in 0..probs.len() {
            //     for i in 0..10 {
            //         let mut h = next_population.get(p * 10 + i).unwrap().heuristic.clone();
            //         h.mutate(&probs[p]);
            //         h_population.push(h);
            //     }
            // }

            // map with index
            let h_population: Vec<Heuristic> = next_population
                .into_iter()
                .zip(0..)
                // .par_bridge()
                .map(|(individual, i)| {
                    Heuristic::new(mutate_heuristic(
                        individual.heuristic.root(),
                        &Some(probs[i / mutations_per_prob].clone()),
                    ))
                })
                .collect();

            // Solve the problem cycle with each heuristic in the population
            let i_population: Vec<Individual> = h_population
                .iter()
                // .par_iter()
                .map(|heuristic| self.compute_individual(heuristic.clone()))
                .collect();

            let after = (&i_population)
                .iter()
                // .map(|i| self.baseline_expansions as f64 / i.expansions as f64)
                .map(|i| i.fitness(self.baseline_expansions, self.baseline_path_len))
                .collect::<Vec<_>>();

            for p in 0..probs.len() {
                for i in 0..mutations_per_prob {
                    // prob_performance[p] += (after[p * mutations_per_prob + i]
                    //     - before[p * mutations_per_prob + i])
                    //     .max(0.0);
                    // * after[p * mutations_per_prob + i];

                    prob_performance[p] += (before[p * mutations_per_prob + i]
                        - after[p * mutations_per_prob + i])
                        .max(0.0)
                        / after[p * mutations_per_prob + i];

                    // prob_performance[p] += after[p * mutations_per_prob + i];
                }
            }

            // Update the best individuals
            self.best_individuals
                .extend(i_population.clone().into_iter());

            // remove duplicates
            let mut set = HashSet::new();
            self.best_individuals
                .retain(|individual| set.insert(individual.clone()));

            self.best_individuals.sort_by(|a, b| {
                let a_fitness = a.fitness(self.baseline_expansions, self.baseline_path_len);
                let b_fitness = b.fitness(self.baseline_expansions, self.baseline_path_len);
                a_fitness.partial_cmp(&b_fitness).unwrap_or(Ordering::Equal)
            });
            self.best_individuals.truncate(900);

            // Log the best individuals
            // iter_count += 1;
            // if timer.elapsed() > next_log {
            println!("\n### Best Heuristics ###\n");
            for individual in self.best_individuals.iter().take(5) {
                println!(
                    "Heuristic {:2.2}% expansions of baseline, {:2.2}% path len of baseline: {}",
                    100.0 * individual.expansions as f64 / self.baseline_expansions as f64,
                    100.0 * individual.path_len as f64 / self.baseline_path_len as f64,
                    individual.heuristic.root()
                );
            }
            // println!("\n Iterations per second: {}", iter_count as f64 / 100.0);
            // iter_count = 0;
            // next_log = timer.elapsed() + Duration::from_secs(mutations_per_prob);
            // }
        }

        // divide prob_performance by 5 * 10
        for p in 0..probs.len() {
            prob_performance[p] /= (rounds * mutations_per_prob) as f64;
        }

        (
            self.best_individuals
                .iter()
                .take(10)
                .map(|individual| {
                    (
                        individual.heuristic.root().to_string(),
                        individual.fitness(self.baseline_expansions, self.baseline_path_len),
                    )
                })
                .collect::<Vec<_>>(),
            prob_performance,
        )

        // prob_performance
    }
}

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
