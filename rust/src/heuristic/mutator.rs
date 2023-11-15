use super::{
    parser::HeuristicNode,
    util::{heuristic_size, random_heuristic}, mutate_probs::TermProbabilities,
};
use crate::constants::*;

pub fn mutate_heuristic(heuristic: &HeuristicNode, term_probs: &Option<TermProbabilities>) -> HeuristicNode {
    // Since there is no guarantee that mutation will occur on the first call,
    // we loop until the heuristic is actually mutated
    let mut_prob = 1.0 / (heuristic_size(&heuristic) as f32);

    loop {
        let (new_heuristic, mutated) = mutate_heuristic_helper(heuristic, mut_prob, term_probs, MAX_TREE_SIZE);
        if mutated {
            break new_heuristic;
        }
    }
}

fn mutate_heuristic_helper(
    heuristic: &HeuristicNode,
    mut_prob: f32,
    term_probs: &Option<TermProbabilities>,
    max_possible_tree_size: i32,
) -> (HeuristicNode, bool) {
    // Sample the new tree size to result in a maximum tree size of MAX_TREE_SIZE
    let new_tree_size = fastrand::i32(1..=max_possible_tree_size);

    // Mutation probability of 1 / hsize
    // => Mutate iff X ~ Unif[0, 1] <= 1 / hsize

    match mut_prob >= fastrand::f32() {
        true => (random_heuristic(new_tree_size, term_probs), true),
        false => match heuristic {
            HeuristicNode::Number(_) => (random_heuristic(new_tree_size, term_probs), false),
            HeuristicNode::Terminal(_) => (random_heuristic(new_tree_size, term_probs), false),
            HeuristicNode::Unary(rule, h) => {
                let (new_h, mutated) =
                    mutate_heuristic_helper(h, mut_prob, term_probs, max_possible_tree_size - 1);
                (HeuristicNode::Unary(*rule, Box::new(new_h)), mutated)
            }
            HeuristicNode::Binary(rule, h1, h2) => {
                let right_size = heuristic_size(h2);
                let (new_h, mutated) =
                    mutate_heuristic_helper(h1, mut_prob, term_probs, max_possible_tree_size - right_size - 1);

                if mutated {
                    return (
                        HeuristicNode::Binary(*rule, Box::new(new_h), h2.clone()),
                        mutated,
                    );
                }

                let left_size = heuristic_size(h1);
                let (new_h, mutated) =
                    mutate_heuristic_helper(h2, mut_prob, term_probs, max_possible_tree_size - left_size - 1);
                (
                    HeuristicNode::Binary(*rule, h1.clone(), Box::new(new_h)),
                    mutated,
                )
            }
        },
    }
}
