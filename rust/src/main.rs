mod constants;
mod heuristic;
mod map;
mod alife;

use heuristic::mutator::mutate_heuristic;
use heuristic::parser::parse_heuristic;
use heuristic::util::{heuristic_size, random_heuristic};
use map::parser::parse_map_file;
use map::util::Maps;
use alife::search::problem::Problem;

fn main() {
    let choice = 2;

    match choice {
        0 => heuristic_demo(),
        1 => map_demo(),
        2 => search_demo(),
        _ => { unreachable!(); }
    }
}

fn heuristic_demo() {
    println!("{:?}", parse_heuristic("(+ deltaX deltaY)"));
    println!("{:?}", parse_heuristic("(min (* (* deltaY (abs y2)) (abs (max y2 deltaY))) \
                                        (min x1 (neg (abs (abs (neg (sqrt (sqr x2))))))))"));
    println!("{:?}", random_heuristic(2));

    let mut h = parse_heuristic("(+ deltaX deltaY)");
    for _ in 0..100 {
        h = mutate_heuristic(&h);
        println!("{}", h);
        println!("{:?}", heuristic_size(&h));
    }
}

fn map_demo() {
    let map  = parse_map_file(Maps::Den009d.value());
    println!("{}", map);
}

fn search_demo() {
    let map = parse_map_file(Maps::Den009d.value());
    let h = parse_heuristic("(+ deltaX deltaY)");

    // Generate random start and goal positions
    let start_pos = map.random_free_position();
    let mut goal_pos = map.random_free_position();

    while start_pos == goal_pos {
        goal_pos = map.random_free_position();
    }

    println!("Start: {:?}", map.ind2sub(start_pos));
    println!("Goal: {:?}", map.ind2sub(goal_pos));

    let mut problem = Problem::new(&map, &h, start_pos, goal_pos);
    let (solved, complete) = problem.solve();

    assert_eq!(solved, true);
    assert_eq!(complete, true);
    problem.print_path_on_map();
}