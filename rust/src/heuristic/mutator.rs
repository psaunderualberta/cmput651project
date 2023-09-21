use super::{
    parser::Heuristic,
    util::{heuristic_size, random_heuristic},
};
use crate::constants::*;

pub fn mutate_heuristic(heuristic: &Heuristic) -> Heuristic {
    // Since there is no guarantee that mutation will occur on the first call,
    // we loop until the heuristic is actually mutated
    let mut_prob = 1.0 / (heuristic_size(&heuristic) as f32);

    loop {
        let (new_heuristic, mutated) = mutate_heuristic_helper(heuristic, mut_prob, MAX_TREE_SIZE);
        if mutated {
            break new_heuristic;
        }
    }
}

fn mutate_heuristic_helper(
    heuristic: &Heuristic,
    mut_prob: f32,
    max_possible_tree_size: i32,
) -> (Heuristic, bool) {
    // Sample the new tree size to result in a maximum tree size of MAX_TREE_SIZE
    let new_tree_size = fastrand::i32(1..=max_possible_tree_size);

    // Mutation probability of 1 / hsize
    // => Mutate iff X ~ Unif[0, 1] <= 1 / hsize

    // TODO: THIS DOESN"T WORK LOL
    match mut_prob >= fastrand::f32() {
        true => (random_heuristic(new_tree_size), true),
        false => match heuristic {
            Heuristic::Terminal(_) => (random_heuristic(new_tree_size), false),
            Heuristic::Unary(rule, h) => {
                let (new_h, mutated) =
                    mutate_heuristic_helper(h, mut_prob, max_possible_tree_size - 1);
                (Heuristic::Unary(*rule, Box::new(new_h)), mutated)
            }
            Heuristic::Binary(rule, h1, h2) => {
                let right_size = heuristic_size(h2);
                let (new_h, mutated) =
                    mutate_heuristic_helper(h1, mut_prob, max_possible_tree_size - right_size - 1);

                if mutated {
                    return (
                        Heuristic::Binary(*rule, Box::new(new_h), h2.clone()),
                        mutated,
                    );
                }

                let left_size = heuristic_size(h1);
                let (new_h, mutated) =
                    mutate_heuristic_helper(h2, mut_prob, max_possible_tree_size - left_size - 1);
                (
                    Heuristic::Binary(*rule, h1.clone(), Box::new(new_h)),
                    mutated,
                )
            }
        },
    }
}
