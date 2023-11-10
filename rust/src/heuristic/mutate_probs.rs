use crate::heuristic::util::normalize_vector;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum MutationType {
    Binary,
    Unary,
    Terminal,
    Number,
}

pub struct MutationProbabilities {
    pub binaries: Vec<f64>,
    pub unaries: Vec<f64>,
    pub terminals: Vec<f64>,
    pub numbers: Vec<f64>,
}

impl MutationProbabilities {
    pub fn new(uniform: bool) -> MutationProbabilities {
        let mut num_terms = HashMap::new();
        num_terms.insert(MutationType::Binary, 6);
        num_terms.insert(MutationType::Unary, 4);
        num_terms.insert(MutationType::Terminal, 6);
        num_terms.insert(MutationType::Number, 18); // -9 to 9, except 0

        match uniform {
            true => MutationProbabilities {
                binaries: MutationProbabilities::uniform_vector(num_terms[&MutationType::Binary]),
                unaries: MutationProbabilities::uniform_vector(num_terms[&MutationType::Unary]),
                terminals: MutationProbabilities::uniform_vector(
                    num_terms[&MutationType::Terminal],
                ),
                numbers: MutationProbabilities::uniform_vector(num_terms[&MutationType::Number]),
            },
            false => MutationProbabilities {
                binaries: MutationProbabilities::random_vector(num_terms[&MutationType::Binary]),
                unaries: MutationProbabilities::random_vector(num_terms[&MutationType::Unary]),
                terminals: MutationProbabilities::random_vector(num_terms[&MutationType::Terminal]),
                numbers: MutationProbabilities::random_vector(num_terms[&MutationType::Number]),
            },
        }
    }

    fn uniform_vector(num_elements: i32) -> Vec<f64> {
        vec![1.0 / num_elements as f64; num_elements as usize]
    }

    fn random_vector(num_elements: i32) -> Vec<f64> {
        let mut vec = Vec::new();

        // Create a vector of random numbers between 0 and 1
        for _ in 0..num_elements {
            vec.push(fastrand::f64());
        }

        // Normalize so that sum(vec) = 1
        let sum: f64 = vec.iter().sum();
        for i in 0..num_elements {
            vec[i as usize] /= sum;
        }

        vec
    }

    fn crossover(&self, other: &Self) -> Self {
        let mut result = MutationProbabilities {
            binaries: Vec::new(),
            unaries: Vec::new(),
            terminals: Vec::new(),
            numbers: Vec::new(),
        };

        // Create lambda function to sum two vectors then normalize
        let sum_normalize = |vec1: &Vec<f64>, vec2: &Vec<f64>| -> Vec<f64> {
            let mut result = Vec::new();
            for i in 0..vec1.len() {
                result.push(vec1[i] + vec2[i]);
            }

            normalize_vector(&mut result);

            result
        };

        // Sum the probabilities of each mutation type, then re-normalize so
        // that sum(vec) = 1
        result.binaries = sum_normalize(&self.binaries, &other.binaries);
        result.unaries = sum_normalize(&self.unaries, &other.unaries);
        result.terminals = sum_normalize(&self.terminals, &other.terminals);
        result.numbers = sum_normalize(&self.numbers, &other.numbers);

        result
    }

    fn mutate(&mut self, mut_prob: f64) -> () {
        // With probability 'mut_prob', change the value of each element in
        // every vector to a random number between 0 and 1
        for i in 0..self.binaries.len() {
            if fastrand::f64() < mut_prob {
                self.binaries[i] = fastrand::f64();
            }
        }
        for i in 0..self.unaries.len() {
            if fastrand::f64() < mut_prob {
                self.unaries[i] = fastrand::f64();
            }
        }
        for i in 0..self.terminals.len() {
            if fastrand::f64() < mut_prob {
                self.terminals[i] = fastrand::f64();
            }
        }
        for i in 0..self.numbers.len() {
            if fastrand::f64() < mut_prob {
                self.numbers[i] = fastrand::f64();
            }
        }

        // Re-normalize so that sum(vec) = 1
        normalize_vector(&mut self.binaries);
        normalize_vector(&mut self.unaries);
        normalize_vector(&mut self.terminals);
        normalize_vector(&mut self.numbers);
    }
}
