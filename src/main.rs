mod heuristic;

use crate::heuristic::parse_heuristic;

fn main() {
    println!("{:?}", parse_heuristic("( + deltaX deltaY )"));

    println!("{:?}", parse_heuristic("( min ( * ( * deltaY ( abs y2 ) ) ( abs ( max y2 deltaY ) ) ) ( min x1 ( neg ( abs ( abs ( neg ( sqrt ( sqr x2 ) ) ) ) ) ) ) )"));
}