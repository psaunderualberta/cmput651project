use super::{mutate_probs::TermProbabilities, parser::HeuristicNode};
use crate::heuristic::mutate_probs::Term;
use crate::heuristic::parser::Rule;
use std::cmp::*;

pub fn heuristic_size(heuristic: &HeuristicNode) -> i32 {
    match heuristic {
        HeuristicNode::Number(_) => 1,
        HeuristicNode::Terminal(_) => 1,
        HeuristicNode::Unary(_, heuristic) => 1 + heuristic_size(heuristic),
        HeuristicNode::Binary(_, left, right) => 1 + heuristic_size(left) + heuristic_size(right),
    }
}

pub fn heuristic_depth(heuristic: &HeuristicNode) -> i32 {
    match heuristic {
        HeuristicNode::Number(_) => 1,
        HeuristicNode::Terminal(_) => 1,
        HeuristicNode::Unary(_, heuristic) => 1 + heuristic_depth(heuristic),
        HeuristicNode::Binary(_, left, right) => {
            1 + max(heuristic_depth(left), heuristic_depth(right))
        }
    }
}

pub fn random_heuristic(hsize: i32, term_probs: &Option<TermProbabilities>) -> HeuristicNode {
    let hsize = match hsize >= 1 {
        true => hsize,
        _ => fastrand::i32(1..=40),
    };

    let binding = Some(TermProbabilities::new(true));
    let term_probs = match term_probs.is_none() {
        true => &binding,
        false => term_probs,
    };

    // Base cases
    if hsize == 1 {
        return match fastrand::i32(0..=1) {
            0 => random_terminal(term_probs),
            1 => random_number(term_probs),
            other => {
                unreachable!("{:?}", other)
            }
        };
    } else if hsize == 2 {
        // with a heuristic size of 2, we can only have unary -> terminal
        // we can't have a binary, since that implies at least 3 terms
        return random_unary(2, term_probs);
    }

    match fastrand::u32(0..=1) {
        0 => random_unary(hsize, term_probs),
        1 => random_binary(hsize, term_probs),
        _ => {
            unreachable!()
        }
    }
}

fn random_number(term_probs: &Option<TermProbabilities>) -> HeuristicNode {
    let items = (1..=9).collect::<Vec<i32>>();

    HeuristicNode::Number(random_weighted_sample::<i32>(
        term_probs.as_ref().unwrap().get(Term::Number),
        &items,
    ))
}

fn random_terminal(term_probs: &Option<TermProbabilities>) -> HeuristicNode {
    let items = vec![
        Rule::x1,
        Rule::x2,
        Rule::y1,
        Rule::y2,
        Rule::deltaX,
        Rule::deltaY,
    ];

    HeuristicNode::Terminal(random_weighted_sample::<Rule>(
        term_probs.as_ref().unwrap().get(Term::Terminal),
        &items,
    ))
}

fn random_unary(hsize: i32, term_probs: &Option<TermProbabilities>) -> HeuristicNode {
    let sub = Box::new(random_heuristic(hsize - 1, term_probs));
    let items = vec![Rule::neg, Rule::abs, Rule::sqrt, Rule::sqr];

    HeuristicNode::Unary(
        random_weighted_sample::<Rule>(term_probs.as_ref().unwrap().get(Term::Unary), &items),
        sub,
    )
}

fn random_binary(hsize: i32, term_probs: &Option<TermProbabilities>) -> HeuristicNode {
    let left_subtree_size = fastrand::i32(1..=hsize - 2);
    let right_subtree_size = hsize - left_subtree_size - 1;
    let left = Box::new(random_heuristic(left_subtree_size, term_probs));
    let right = Box::new(random_heuristic(right_subtree_size, term_probs));

    let items = vec![
        Rule::plus,
        Rule::div,
        Rule::mul,
        Rule::minus,
        Rule::max,
        Rule::min,
    ];

    HeuristicNode::Binary(
        random_weighted_sample::<Rule>(term_probs.as_ref().unwrap().get(Term::Binary), &items),
        left,
        right,
    )
}

pub fn random_weighted_sample<T: Clone>(probs: &Vec<f64>, items: &Vec<T>) -> T {
    // throw error if probs has length 0
    if probs.len() == 0 {
        panic!("probs has length 0");
    }

    // throw error if probs and items have different lengths
    if probs.len() != items.len() {
        panic!("probs and items have different lengths");
    }

    let mut cum_probs = Vec::new();
    let mut cum_prob = 0.0;
    for prob in probs {
        cum_prob += prob;
        cum_probs.push(cum_prob);
    }

    let rand = fastrand::f64();
    for i in 0..cum_probs.len() {
        if rand <= cum_probs[i] {
            return items[i].clone();
        }
    }

    unreachable!()
}

pub fn normalize_vector(vec: &mut Vec<f64>) -> () {
    let sum: f64 = vec.iter().sum();
    for i in 0..vec.len() {
        vec[i] /= sum;
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

    // Tests for normalize vector
    #[test]
    fn test_normalize_vector_1() {
        let mut vec = vec![1.0, 1.0, 1.0];
        normalize_vector(&mut vec);
        assert_eq!(vec, vec![1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0]);
    }

    #[test]
    fn test_normalize_vector_2() {
        let mut vec = vec![1.0, 0.5, 1.0];
        normalize_vector(&mut vec);
        assert_eq!(vec, vec![1.0 / 2.5, 0.5 / 2.5, 1.0 / 2.5]);
    }

    #[test]
    fn test_normalize_vector_3() {
        let mut vec = vec![0.7, 0.5, 0.312, 0.5];
        normalize_vector(&mut vec);
        assert_eq!(
            vec,
            vec![0.7 / 2.012, 0.5 / 2.012, 0.312 / 2.012, 0.5 / 2.012]
        );
    }
}
