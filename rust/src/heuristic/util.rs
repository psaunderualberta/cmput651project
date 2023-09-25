use super::parser::HeuristicNode;
use crate::heuristic::parser::Rule;
use std::cmp::*;

pub fn heuristic_size(heuristic: &HeuristicNode) -> i32 {
    match heuristic {
        HeuristicNode::Terminal(_) => 1,
        HeuristicNode::Unary(_, heuristic) => 1 + heuristic_size(heuristic),
        HeuristicNode::Binary(_, left, right) => 1 + heuristic_size(left) + heuristic_size(right),
    }
}

pub fn heuristic_depth(heuristic: &HeuristicNode) -> i32 {
    match heuristic {
        HeuristicNode::Terminal(_) => 1,
        HeuristicNode::Unary(_, heuristic) => 1 + heuristic_depth(heuristic),
        HeuristicNode::Binary(_, left, right) => {
            1 + max(heuristic_depth(left), heuristic_depth(right))
        }
    }
}

pub fn random_heuristic(hsize: i32) -> HeuristicNode {
    let hsize = match hsize >= 1 {
        true => hsize,
        _ => fastrand::i32(1..=40),
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
        _ => {
            unreachable!()
        }
    }
}

fn random_terminal() -> HeuristicNode {
    match fastrand::u32(0..=5) {
        0 => HeuristicNode::Terminal(Rule::x1),
        1 => HeuristicNode::Terminal(Rule::x2),
        2 => HeuristicNode::Terminal(Rule::y1),
        3 => HeuristicNode::Terminal(Rule::y2),
        4 => HeuristicNode::Terminal(Rule::deltaX),
        5 => HeuristicNode::Terminal(Rule::deltaY),
        _ => {
            unreachable!()
        }
    }
}

fn random_unary(hsize: i32) -> HeuristicNode {
    match fastrand::u32(0..=3) {
        0 => HeuristicNode::Unary(Rule::neg, Box::new(random_heuristic(hsize - 1))),
        1 => HeuristicNode::Unary(Rule::abs, Box::new(random_heuristic(hsize - 1))),
        2 => HeuristicNode::Unary(Rule::sqrt, Box::new(random_heuristic(hsize - 1))),
        3 => HeuristicNode::Unary(Rule::sqr, Box::new(random_heuristic(hsize - 1))),
        _ => {
            unreachable!()
        }
    }
}

fn random_binary(hsize: i32) -> HeuristicNode {
    let left_subtree_size = fastrand::i32(1..=hsize - 2);
    let right_subtree_size = hsize - left_subtree_size - 1;

    match fastrand::u32(0..=5) {
        0 => HeuristicNode::Binary(
            Rule::plus,
            Box::new(random_heuristic(left_subtree_size)),
            Box::new(random_heuristic(right_subtree_size)),
        ),
        1 => HeuristicNode::Binary(
            Rule::div,
            Box::new(random_heuristic(left_subtree_size)),
            Box::new(random_heuristic(right_subtree_size)),
        ),
        2 => HeuristicNode::Binary(
            Rule::mul,
            Box::new(random_heuristic(left_subtree_size)),
            Box::new(random_heuristic(right_subtree_size)),
        ),
        3 => HeuristicNode::Binary(
            Rule::minus,
            Box::new(random_heuristic(left_subtree_size)),
            Box::new(random_heuristic(right_subtree_size)),
        ),
        4 => HeuristicNode::Binary(
            Rule::max,
            Box::new(random_heuristic(left_subtree_size)),
            Box::new(random_heuristic(right_subtree_size)),
        ),
        5 => HeuristicNode::Binary(
            Rule::min,
            Box::new(random_heuristic(left_subtree_size)),
            Box::new(random_heuristic(right_subtree_size)),
        ),
        _ => {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for heuristic size
    #[test]
    fn test_heuristic_size_1() {
        let h1 = HeuristicNode::Terminal(Rule::x1);
        assert_eq!(heuristic_size(&h1), 1);
    }

    #[test]
    fn test_heuristic_size_2() {
        let h2 = HeuristicNode::Unary(Rule::neg, Box::new(HeuristicNode::Terminal(Rule::x1)));
        assert_eq!(heuristic_size(&h2), 2);
    }

    #[test]
    fn test_heuristic_size_3() {
        let h3 = HeuristicNode::Binary(
            Rule::plus,
            Box::new(HeuristicNode::Unary(
                Rule::abs,
                Box::new(HeuristicNode::Terminal(Rule::deltaX)),
            )),
            Box::new(HeuristicNode::Terminal(Rule::deltaY)),
        );
        assert_eq!(heuristic_size(&h3), 4);
    }

    // Tests for heuristic depth
    #[test]
    fn test_heuristic_depth_1() {
        let h1 = HeuristicNode::Terminal(Rule::x1);
        assert_eq!(heuristic_depth(&h1), 1);
    }

    #[test]
    fn test_heuristic_depth_2() {
        let h2 = HeuristicNode::Unary(Rule::neg, Box::new(HeuristicNode::Terminal(Rule::x1)));
        assert_eq!(heuristic_depth(&h2), 2);
    }

    #[test]
    fn test_heuristic_depth_3() {
        let h3 = HeuristicNode::Binary(
            Rule::plus,
            Box::new(HeuristicNode::Unary(
                Rule::abs,
                Box::new(HeuristicNode::Terminal(Rule::deltaX)),
            )),
            Box::new(HeuristicNode::Terminal(Rule::deltaY)),
        );
        assert_eq!(heuristic_depth(&h3), 3);
    }
}
