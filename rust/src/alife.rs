pub mod search;
pub mod sim;

use crate::alife::search::cycle::ProblemCycle;
use crate::alife::sim::simulator::Simulation;
use crate::constants::PROBLEM_CYCLE_LENGTH;
use crate::heuristic::parser::parse_heuristic;

use crate::map::util::Map;
use std::time::Duration;

use self::search::cycle::CycleSolver;

pub fn alife(map: Map, time_limit: Duration) {
    let seed = Some(69);

    let cycle = ProblemCycle::new(map.clone(), PROBLEM_CYCLE_LENGTH);
    let manhattan = parse_heuristic("(+ deltaX deltaY)");
    let mut baseline = CycleSolver::from_cycle(cycle.clone(), map.clone(), manhattan);
    baseline.solve_cycle();
    let expansion_limit = baseline.get_total_expansions_in_cycle() * 5;

    let mut sim = Simulation::new(
        map,
        cycle,
        baseline,
        expansion_limit,
        time_limit,
        seed,
        true,
    );

    let result = sim.run();
    println!("Best: {:?}", result.best);
}
