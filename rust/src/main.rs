mod heuristic;
mod constants;

use crate::heuristic::parser::parse_heuristic;
use crate::heuristic::util::random_heuristic;
use crate::heuristic::mutator::mutate_heuristic;

fn main() {
    println!("{:?}", parse_heuristic("(+ deltaX deltaY)"));
    println!("{:?}", parse_heuristic("(min (* (* deltaY (abs y2)) (abs (max y2 deltaY))) (min x1 (neg (abs (abs (neg (sqrt (sqr x2))))))))"));
    println!("{:?}", random_heuristic(2));

    let mut h = parse_heuristic("(+ deltaX deltaY)");
    for _ in 0..1000 {
        h = mutate_heuristic(&h);
        // println!("{}", h);
        // println!("{:?}", heuristic::util::heuristic_size(&h));
    }
}