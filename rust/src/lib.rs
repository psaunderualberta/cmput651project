pub mod alife;
pub mod constants;
pub mod heuristic;
pub mod map;

use pyo3::prelude::*;
use pyo3::{
    exceptions::PyIndexError,
    pymodule,
    types::{PyDict, PyModule},
    FromPyObject, PyAny, PyObject, Python,
};

use alife::search::problem::Problem;
use heuristic::parser::parse_heuristic;
use heuristic::Heuristic;
use map::parser::parse_map_file;
use map::util::Maps;

use crate::heuristic::executors::interpreter::Interpreter;
use crate::heuristic::executors::HeuristicExecuter;

#[pymodule]
fn libcmput651py<'py>(py: Python<'py>, m: &'py PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    m.add_function(wrap_pyfunction!(test_heuristic, m)?)?;

    let heuristic_module = PyModule::new(py, "heuristic")?;
    heuristic_module.add_function(wrap_pyfunction!(manhattan_distance, m)?)?;
    m.add_submodule(heuristic_module)?;

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
fn test_heuristic(h: &Heuristic) -> PyResult<()> {
    let map = parse_map_file(Maps::Den312d.value());

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
fn manhattan_distance() -> PyResult<Heuristic> {
    Ok(parse_heuristic("(+ deltaX deltaY)"))
}
