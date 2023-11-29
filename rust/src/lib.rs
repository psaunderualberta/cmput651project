pub mod alife;
pub mod constants;
pub mod heuristic;
pub mod map;

use std::collections::HashMap;
use std::time::Duration;

use alife::search::cycle::{CycleSolver, ProblemCycle};
use alife::sim::genetic_algorithm::{GeneticAlgorithm, GeneticAlgorithmResult};
use alife::sim::simulator::{Simulation, SimulationResult};
use constants::PROBLEM_CYCLE_LENGTH;
use heuristic::mutate_probs::{Term, TermProbabilities};
use pyo3::prelude::*;
use pyo3::{pymodule, types::PyModule, Python};

use alife::search::problem::{Problem, ProblemResult};
use heuristic::parser::parse_heuristic;
use heuristic::Heuristic;
use map::parser::parse_map_file;
use map::util::{Map, Maps};

use crate::heuristic::executors::interpreter::Interpreter;
use crate::heuristic::executors::HeuristicExecuter;

#[pymodule]
fn libcmput651py<'py>(py: Python<'py>, m: &'py PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(test_heuristic, m)?)?;
    m.add_function(wrap_pyfunction!(solve_cycle_on_map, m)?)?;
    m.add_function(wrap_pyfunction!(get_problems, m)?)?;

    let heuristic_module = PyModule::new(py, "heuristic")?;
    heuristic_module.add_function(wrap_pyfunction!(manhattan_distance, m)?)?;
    m.add_submodule(heuristic_module)?;

    // Alife module
    let alife_module = PyModule::new(py, "alife")?;
    alife_module.add_function(wrap_pyfunction!(simulation, m)?)?;
    m.add_submodule(alife_module)?;

    let ga_module = PyModule::new(py, "genetic_algorithm")?;
    ga_module.add_function(wrap_pyfunction!(genetic_algorithm, m)?)?;
    ga_module.add_function(wrap_pyfunction!(get_genetic_algorithm, m)?)?;
    ga_module.add_function(wrap_pyfunction!(random_term_probabilities, m)?)?;
    ga_module.add_function(wrap_pyfunction!(crossover_probabilities, m)?)?;
    ga_module.add_function(wrap_pyfunction!(mutate_probabilities, m)?)?;
    ga_module.add_function(wrap_pyfunction!(term_probabilities_from_dict, m)?)?;
    ga_module.add_function(wrap_pyfunction!(probabilities2dict, m)?)?;
    m.add_submodule(ga_module)?;

    Ok(())
}

// simple function for debugging
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    #[cfg(debug_assertions)]
    println!("Debugging enabled");

    #[cfg(not(debug_assertions))]
    println!("Debugging disabled");

    Ok((a + b).to_string())
}

#[pyfunction]
fn get_problems(map_name: String) -> PyResult<(Map, ProblemCycle)> {
    let map_path = Maps::name2path(map_name.as_str());
    let map = parse_map_file(map_path);

    let cycle = ProblemCycle::new(map.clone(), PROBLEM_CYCLE_LENGTH);
    let manhattan = parse_heuristic("(+ deltaX deltaY)");
    let mut baseline = CycleSolver::from_cycle(cycle.clone(), map.clone(), manhattan);
    baseline.solve_cycle();

    Ok((map, cycle))
}

#[pyfunction]
fn test_heuristic(h: &Heuristic) -> PyResult<()> {
    let map = parse_map_file(Maps::Den312d.path());

    // Generate random start and goal positions
    let start = map.random_free_position();
    let mut goal = map.random_free_position();
    while start == goal {
        goal = map.random_free_position();
    }

    println!("Start: {:?}", map.ind2sub(start));
    println!("Goal: {:?}", map.ind2sub(goal));

    let problem = Problem::new(start, goal);
    let executer = Interpreter::create(h);
    let result = problem.solve(&map, |x1, y1, x2, y2| executer.execute(x1, y1, x2, y2));

    assert!(result.solved);
    problem.print_path_on_map(&map, result.solution_path);

    Ok(())
}

#[pyfunction]
fn solve_cycle_on_map(map_name: String, h: &Heuristic) -> PyResult<Vec<ProblemResult>> {
    let map_path = Maps::name2path(map_name.as_str());
    let map = parse_map_file(map_path);

    Ok(CycleSolver::new(map, h.clone(), PROBLEM_CYCLE_LENGTH).solve_cycle())
}

#[pyfunction]
fn simulation(map_name: String, seed: u64, secs: u64) -> PyResult<SimulationResult> {
    let map_path = Maps::name2path(map_name.as_str());
    let map = parse_map_file(map_path);

    let cycle = ProblemCycle::new(map.clone(), PROBLEM_CYCLE_LENGTH);
    let manhattan = parse_heuristic("(+ deltaX deltaY)");
    let mut baseline = CycleSolver::from_cycle(cycle.clone(), map.clone(), manhattan);
    baseline.solve_cycle();
    let expansion_limit = baseline.get_total_expansions_in_cycle() * 5;
    let time_limit = Duration::from_secs(secs);

    let mut sim = Simulation::new(
        map,
        cycle,
        baseline,
        expansion_limit,
        time_limit,
        Some(seed),
        true,
    );

    Ok(sim.run())
}

#[pyfunction]
fn manhattan_distance() -> PyResult<Heuristic> {
    Ok(parse_heuristic("(+ deltaX deltaY)"))
}

#[pyfunction]
fn genetic_algorithm(
    m: Map,
    c: ProblemCycle,
    probs: TermProbabilities,
    seed: u64,
    secs: u64,
) -> PyResult<GeneticAlgorithmResult> {
    let manhattan = parse_heuristic("(+ deltaX deltaY)");
    let mut baseline = CycleSolver::from_cycle(c.clone(), m.clone(), manhattan);
    baseline.solve_cycle();

    let time_limit = Duration::from_secs(secs);
    let expansion_limit: usize = baseline.get_total_expansions_in_cycle() * 5;

    let mut sim = GeneticAlgorithm::new(
        m,
        c,
        baseline,
        expansion_limit,
        time_limit,
        Some(probs),
        Some(seed),
        true,
    );

    Ok(sim.run())
}

#[pyfunction]
fn get_genetic_algorithm() -> GeneticAlgorithm {
    let map = parse_map_file(Maps::Den312d.path());
    let seed = Some(42);

    let cycle = ProblemCycle::new(map.clone(), PROBLEM_CYCLE_LENGTH);
    let manhattan = parse_heuristic("(+ deltaX deltaY)");
    let mut baseline = CycleSolver::from_cycle(cycle.clone(), map.clone(), manhattan);
    baseline.solve_cycle();
    let expansion_limit: usize = baseline.get_total_expansions_in_cycle() * 5;

    GeneticAlgorithm::new(
        map,
        cycle,
        baseline,
        expansion_limit,
        Duration::from_secs(10),
        None,
        seed,
        true,
    )
}

#[pyfunction]
fn crossover_probabilities(
    probs1: TermProbabilities,
    probs2: TermProbabilities,
) -> PyResult<TermProbabilities> {
    Ok(probs1.crossover(&probs2))
}

#[pyfunction]
fn mutate_probabilities(probs: TermProbabilities, mut_prob: f64) -> PyResult<TermProbabilities> {
    Ok(probs.mutate(mut_prob))
}

#[pyfunction]
fn random_term_probabilities(uniform: bool) -> PyResult<TermProbabilities> {
    Ok(TermProbabilities::new(uniform))
}

#[pyfunction]
fn term_probabilities_from_dict(dict: HashMap<String, Vec<f64>>) -> PyResult<TermProbabilities> {
    Ok(TermProbabilities::from_hashmap(dict))
}

#[pyfunction]
fn probabilities2dict(probs: TermProbabilities) -> PyResult<HashMap<String, HashMap<String, f64>>> {
    let mut dict = HashMap::new();

    for term in vec!["binaries", "unaries", "terminals", "numbers"] {
        let mut term_dict = HashMap::new();
        let term_probs = probs.get(Term::from_str(term));
        let operators = probs.get_operator_order(term);

        for (operator, prob) in operators.iter().zip(term_probs.iter()) {
            term_dict.insert(operator.to_string(), *prob);
        }

        dict.insert(term.to_string(), term_dict);
    }
    Ok(dict)
}
