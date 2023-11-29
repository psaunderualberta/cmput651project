mod alife;
mod constants;
mod heuristic;
mod map;

use std::time::Duration;

use alife::search::problem::Problem;
use heuristic::executors::jit::Jit;
use heuristic::mutator::mutate_heuristic;
use heuristic::parser::parse_heuristic;
use heuristic::util::{heuristic_size, random_heuristic};
use map::parser::parse_map_file;
use map::util::Maps;

use crate::alife::search::cycle::{CycleSolver, ProblemCycle};
use crate::alife::sim::genetic_algorithm::GeneticAlgorithm;
use crate::constants::PROBLEM_CYCLE_LENGTH;
use crate::heuristic::executors::interpreter::Interpreter;
use crate::heuristic::executors::HeuristicExecuter;
use crate::heuristic::Heuristic;

fn main() {
    let choice = 7;

    match choice {
        0 => heuristic_demo(),
        1 => map_demo(),
        2 => search_demo(),
        3 => benchmark(),
        5 => benchmark_executers(),
        6 => alife_demo(),
        7 => ga_demo(),
        8 => eval_heursitic(),
        _ => {
            unreachable!("Invalid choice in function `main`. Please choose from 0-4");
        }
    }
}

fn heuristic_demo() {
    println!("{:?}", parse_heuristic("(+ deltaX deltaY)"));
    println!(
        "{:?}",
        parse_heuristic(
            "(min (* (* deltaY (abs y2)) (abs (max y2 deltaY))) \
                                        (min x1 (neg (abs (abs (neg (sqrt (sqr x2))))))))"
        )
    );
    println!("{:?}", random_heuristic(2, &None));

    let mut h = parse_heuristic("(+ deltaX deltaY)");
    for _ in 0..100 {
        h = Heuristic::new(mutate_heuristic(h.root(), &None));
        println!("{}", h.root());
        println!("{:?}", heuristic_size(h.root()));
    }
}

fn map_demo() {
    let map = parse_map_file(Maps::Den009d.path());
    println!("{}", map);
}

fn search_demo() {
    let map = parse_map_file(Maps::Den312d.path());
    let h = parse_heuristic("(sqr (max deltaY deltaX))");

    // Generate random start and goal positions
    let start = map.random_free_position();
    let mut goal = map.random_free_position();
    while start == goal {
        goal = map.random_free_position();
    }

    println!("Start: {:?}", map.ind2sub(start));
    println!("Goal: {:?}", map.ind2sub(goal));

    let problem = Problem::new(start, goal);
    let executer = Interpreter::create(&h);
    let result = problem.solve(&map, |x1, y1, x2, y2| executer.execute(x1, y1, x2, y2));

    assert!(result.solved);
    problem.print_path_on_map(&map, result.solution_path);
}

fn benchmark() {
    use std::time::Instant;
    let map = parse_map_file(Maps::Orz103d.path());
    let h =
        parse_heuristic("(* (+ (* (+ (+ (+ deltaX deltaY) deltaY) deltaX) deltaY) deltaX) deltaY)");

    // Create problems
    let num_problems = PROBLEM_CYCLE_LENGTH;
    let mut astarcycle = CycleSolver::new(map, h, num_problems);

    // Perform first solve
    let now = Instant::now();
    astarcycle.solve_cycle();
    println!("Time to solve problems on first go: {:.2?}", now.elapsed());

    // Perform second solve of cycle
    let now = Instant::now();
    astarcycle.solve_cycle();
    println!("Time to solve problems on second go: {:.2?}", now.elapsed());
}

fn benchmark_executers() {
    let h =
        parse_heuristic("(* (+ (* (+ (+ (+ deltaX deltaY) deltaY) deltaX) deltaY) deltaX) deltaY)");
    let heuristic = h;

    let mut x = 0.0;
    for _ in 0..10000 {
        let context = inkwell::context::Context::create();
        let jit = Jit::create(&heuristic, &context);
        x += jit.execute(x, x, x, x);
        drop(jit)
    }

    println!("{}", x);
}

fn alife_demo() {
    let map = parse_map_file(Maps::Den312d.path());

    alife::alife(map, Duration::from_secs(10));
}

fn ga_demo() {
    let map = parse_map_file(Maps::Den312d.path());
    let seed = Some(42);

    let cycle = ProblemCycle::new(map.clone(), PROBLEM_CYCLE_LENGTH);
    let manhattan = parse_heuristic("(+ deltaX deltaY)");
    let mut baseline = CycleSolver::from_cycle(cycle.clone(), map.clone(), manhattan);
    baseline.solve_cycle();
    let expansion_limit: usize = baseline.get_total_expansions_in_cycle() * 5;

    let mut sim = GeneticAlgorithm::new(
        map,
        cycle,
        baseline,
        expansion_limit,
        Duration::from_secs(10),
        None,
        seed,
        true,
    );

    let _result = sim.run();
}

fn eval_heursitic() {
    let map = parse_map_file(Maps::Den312d.path());
    let seed = Some(42);

    let cycle = ProblemCycle::new(map.clone(), PROBLEM_CYCLE_LENGTH);
    let manhattan = parse_heuristic("(+ deltaX deltaY)");
    let mut baseline = CycleSolver::from_cycle(cycle.clone(), map.clone(), manhattan);
    baseline.solve_cycle();

    let heuristic = parse_heuristic(
        // "(* (max deltaX deltaY) 9)",
        // "(* deltaX 9)",
        // "(* deltaY 9)",
        "(* (* (* (* (* (max deltaX deltaY) 9) 9) 9) 9) 9)",
        // "(max deltaX deltaY)",
        // "(* deltaX 1)",
    );
    let mut individual = CycleSolver::from_cycle(cycle.clone(), map.clone(), heuristic.clone());
    individual.solve_cycle();

    println!(
        "Heuristic {:2.2}% expansions of baseline: {}",
        100.0 * individual.get_total_expansions_in_cycle() as f64
            / baseline.get_total_expansions_in_cycle() as f64,
        heuristic.root()
    );
}

/* Code for manually creating problems, rather than a single cycle */
// for i in 0..10000 {
//     println!("{}", i);
//     // Generate new problems
//     let start_pos = map.random_free_position();
//     let mut goal_pos = map.random_free_position();
//     while start_pos == goal_pos {
//         goal_pos = map.random_free_position();
//     }

//     // solve the problem
//     let mut problem = AStar::new(&map, &h, start_pos, goal_pos);
//     let (solved, complete) = problem.solve();

//     // Ensure the problem was solved
//     assert!(solved);
//     assert!(complete);
// }
