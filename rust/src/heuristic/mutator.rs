use super::{parser::Heuristic, util::{heuristic_size, random_heuristic}};
use crate::constants::*;

pub fn mutate_heuristic(heuristic: &Heuristic) -> Heuristic {
    // Since there is no guarantee that mutation will occur on the first call,
    // we loop until the heuristic is actually mutated
    loop {
        let (new_heuristic, mutated) = mutate_heuristic_helper(heuristic);
        if mutated && heuristic_size(&new_heuristic) <= MAX_TREE_SIZE {
            break new_heuristic;
        }
    }
}

fn mutate_heuristic_helper(heuristic: &Heuristic) -> (Heuristic, bool) {
    let hsize = heuristic_size(&heuristic);
    
    // Sample the new tree size to result in a maximum tree size of MAX_TREE_SIZE
    let new_tree_size = fastrand::i32(1..=MAX_TREE_SIZE - hsize);

    // Mutation probability of 1 / hsize
    // => Mutate iff X ~ Unif[0, 1] <= 1 / hsize
    match 1.0 >= fastrand::f32() * (hsize as f32) {
        true => (random_heuristic(new_tree_size), true),
        false => mutate_heuristic_helper(heuristic)
    }
}