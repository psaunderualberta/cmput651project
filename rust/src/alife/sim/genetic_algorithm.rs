

use std::collections::{HashSet};
use std::hash::Hash;
use std::time::{Duration, Instant};

use crate::alife::search::cycle::{CycleSolver, ProblemCycle};



use crate::heuristic::util::random_heuristic;
use crate::heuristic::Heuristic;
use crate::map::util::Map;



pub const MAX_BEST_INDIVIDUALS: usize = 10;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Individual {
    pub heuristic: Heuristic,
    pub result: usize,
}

impl Hash for Individual {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.heuristic.root.to_string().hash(state);
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
    // The maximum number of expansions allowed per heuristic
    pub expansion_bound: usize,
    // The maximum amount of time allowed for the simulation
    pub time_limit: Duration,
    // max population size
    pub max_population_size: usize,
    pub population: HashSet<Individual>,
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
            expansion_bound,
            time_limit,
            max_population_size: 100,
            population: HashSet::new(),
            best_individuals: Vec::with_capacity(MAX_BEST_INDIVIDUALS + 1),
        }
    }

    pub fn run(&mut self) {
        for i in 0..self.max_population_size {
            let h = random_heuristic(-1);
            let individual = self.add_individual(Heuristic { root: h });
            println!(
                "Generating initial heuristic #{}/{} with {:2.2}% expansions of baseline",
                i,
                self.max_population_size,
                100.0 * individual.result as f64 / self.baseline_expansions as f64
            );
            println!("Heuristic: {}", individual.heuristic.root);
        }

        let timer = Instant::now();
        let mut next_log = timer.elapsed() + Duration::from_secs(30);
        while timer.elapsed() < self.time_limit {
            let h = random_heuristic(-1);
            let _individual = self.add_individual(Heuristic { root: h });
            if timer.elapsed() > next_log {
                println!("\n### Best Heuristics ###\n");
                for individual in self.best_individuals.iter() {
                    println!(
                        "Heuristic {:2.2}% expansions of baseline: {}",
                        100.0 * individual.result as f64 / self.baseline_expansions as f64,
                        individual.heuristic.root
                    );
                }
                next_log = timer.elapsed() + Duration::from_secs(30);
            }
        }

        println!("\n### Best Heuristics ###\n");
        for individual in self.best_individuals.iter() {
            println!(
                "Heuristic {:2.2}% expansions of baseline: {}",
                100.0 * individual.result as f64 / self.baseline_expansions as f64,
                individual.heuristic.root
            );
        }
    }

    fn add_individual(&mut self, heuristic: Heuristic) -> Individual {
        // insert only if population does not already contain individual
        let mut individual = Individual {
            heuristic,
            result: 0,
        };
        if self.population.contains(&individual) {
            self.population.get(&individual).unwrap().to_owned()
        } else {
            let mut cycle =
                CycleSolver::from_cycle(self.cycle.clone(), self.map, individual.heuristic.clone());
            cycle.solve_cycle();
            let result = cycle.get_total_expansions_in_cycle();
            individual.result = result;
            self.population.insert(individual.clone());
            for i in 0..(self.best_individuals.len() + 1) {
                if result
                    < match self.best_individuals.get(i) {
                        Some(ind) => ind.result,
                        None => usize::MAX,
                    }
                {
                    self.best_individuals.insert(i, individual.clone());
                    break;
                }
            }
            if self.best_individuals.len() > MAX_BEST_INDIVIDUALS {
                self.best_individuals.pop();
            }
            if self.population.len() > self.max_population_size {
                if let Some(worst) = self.population.iter().next() {
                    let mut worst = worst;
                    for individual in self.population.iter() {
                        if individual.result > worst.result {
                            worst = individual;
                        }
                    }
                    self.population.remove(&worst.clone());
                }
            }
            individual
        }
    }
}
