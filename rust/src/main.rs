mod constants;
mod heuristic;
mod map;
mod alife;

use crate::heuristic::mutator::mutate_heuristic;
use crate::heuristic::parser::parse_heuristic;
use crate::heuristic::util::{heuristic_size, random_heuristic};
use crate::map::parser::parse_map_file;
use crate::map::util::Maps;

fn main() {
    match false {
        true => heuristic_demo(),
        false => map_demo(),
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