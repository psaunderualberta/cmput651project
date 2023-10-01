pub mod search;
pub mod sim;

use std::time::Duration;

use crate::alife::search::cycle::ProblemCycle;
use crate::alife::sim::simulator::Simulation;
use crate::constants::PROBLEM_CYCLE_LENGTH;
use crate::heuristic::parser::parse_heuristic;
use crate::map::util::Map;

use self::search::cycle::CycleSolver;

pub fn alife(map: &Map, time_limit: Duration) {
    let seed = Some(42);

    let cycle = ProblemCycle::new(map, PROBLEM_CYCLE_LENGTH);
    let manhattan = parse_heuristic("(+ deltaX deltaY)");
    let mut baseline = CycleSolver::from_cycle(cycle.clone(), map, manhattan);
    baseline.solve_cycle();
    let expansion_limit = baseline.get_total_expansions_in_cycle() * 5;

    let mut sim = Simulation::new(
        map,
        cycle,
        &baseline,
        expansion_limit,
        time_limit,
        seed,
        true
    );

    let result = sim.run();
    println!("Best: {}\n-> {:.3}", result.best, result.score);
}
