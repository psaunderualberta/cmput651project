use std::cmp::*;
use super::parser::Heuristic;
use crate::heuristic::parser::Rule;

pub fn heuristic_size(heuristic: &Heuristic) -> i32 {
    match heuristic {
        Heuristic::Terminal(_) => 1,
        Heuristic::Unary(_, heuristic) => 1 + heuristic_size(heuristic),
        Heuristic::Binary(_, left, right) => 1 + heuristic_size(left) + heuristic_size(right)
    }
}

pub fn heuristic_depth(heuristic: &Heuristic) -> i32 {
    match heuristic {
        Heuristic::Terminal(_) => 1,
        Heuristic::Unary(_, heuristic) => 1 + heuristic_depth(heuristic),
        Heuristic::Binary(_, left, right) => 1 + max(heuristic_depth(left), heuristic_depth(right))
    }
}

pub fn random_heuristic(hsize: i32) -> Heuristic {
    let hsize = match hsize >= 1 {
        true => hsize,
        _ => fastrand::i32(1..=40)
    };

    // Base cases
    if hsize == 1 {
        return random_terminal();
    } else if hsize == 2 {
        // with a heuristic size of 2, we can only have unary -> terminal
        // we can't have a binary, since that implies at least 3 terms
        return random_unary(2);
    }

    match fastrand::u32(0..=1) {
        0 => random_unary(hsize),
        1 => random_binary(hsize),
        _ => { unreachable!() }
    }
}

fn random_terminal() -> Heuristic {
    match fastrand::u32(0..=5) {
        0 => Heuristic::Terminal(Rule::x1),
        1 => Heuristic::Terminal(Rule::x2),
        2 => Heuristic::Terminal(Rule::y1),
        3 => Heuristic::Terminal(Rule::y2),
        4 => Heuristic::Terminal(Rule::deltaX),
        5 => Heuristic::Terminal(Rule::deltaY),
        _ => { unreachable!() }
    }
}

fn random_unary(hsize: i32) -> Heuristic {
    match fastrand::u32(0..=3) {
        0 => Heuristic::Unary(Rule::neg, Box::new(random_heuristic(hsize - 1))),
        1 => Heuristic::Unary(Rule::abs, Box::new(random_heuristic(hsize - 1))),
        2 => Heuristic::Unary(Rule::sqrt, Box::new(random_heuristic(hsize - 1))),
        3 => Heuristic::Unary(Rule::sqr, Box::new(random_heuristic(hsize - 1))),
        _ => { unreachable!() }
    }
}

fn random_binary(hsize: i32) -> Heuristic {

    let left_subtree_size = fastrand::i32(1..=hsize-2);
    let right_subtree_size = hsize - left_subtree_size - 1;

    match fastrand::u32(0..=5) {
        0 => Heuristic::Binary(Rule::x1, Box::new(random_heuristic(left_subtree_size)), Box::new(random_heuristic(right_subtree_size))),
        1 => Heuristic::Binary(Rule::x2, Box::new(random_heuristic(left_subtree_size)), Box::new(random_heuristic(right_subtree_size))),
        2 => Heuristic::Binary(Rule::y1, Box::new(random_heuristic(left_subtree_size)), Box::new(random_heuristic(right_subtree_size))),
        3 => Heuristic::Binary(Rule::y2, Box::new(random_heuristic(left_subtree_size)), Box::new(random_heuristic(right_subtree_size))),
        4 => Heuristic::Binary(Rule::deltaX, Box::new(random_heuristic(left_subtree_size)), Box::new(random_heuristic(right_subtree_size))),
        5 => Heuristic::Binary(Rule::deltaY, Box::new(random_heuristic(left_subtree_size)), Box::new(random_heuristic(right_subtree_size))),
        _ => { unreachable!() }
    }
}

// TODO: Write tests for heuristic_size

// TODO: Write tests for heuristic_depth