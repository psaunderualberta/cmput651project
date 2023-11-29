use pyo3::prelude::*;

use crate::heuristic::util::normalize_vector;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Term {
    Binary,
    Unary,
    Terminal,
    Number,
}

impl Term {
    pub fn from_str(s: &str) -> Term {
        match s {
            "binaries" => Term::Binary,
            "unaries" => Term::Unary,
            "terminals" => Term::Terminal,
            "numbers" => Term::Number,
            _ => {
                unreachable!("Invalid term type '{}'", s);
            }
        }
    }

    pub fn to_str(self: &Self) -> &str {
        match self {
            Term::Binary => "binaries",
            Term::Unary => "unaries",
            Term::Terminal => "terminals",
            Term::Number => "numbers",
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct TermProbabilities {
    #[pyo3(get)]
    pub binaries: Vec<f64>,
    #[pyo3(get)]
    pub unaries: Vec<f64>,
    #[pyo3(get)]
    pub terminals: Vec<f64>,
    #[pyo3(get)]
    pub numbers: Vec<f64>,
}

impl TermProbabilities {
    pub fn new(uniform: bool) -> TermProbabilities {
        let mut num_terms = HashMap::new();
        num_terms.insert(Term::Binary, 6);
        num_terms.insert(Term::Unary, 4);
        num_terms.insert(Term::Terminal, 6);
        num_terms.insert(Term::Number, 9); // 1 to 9

        match uniform {
            true => TermProbabilities {
                binaries: TermProbabilities::uniform_vector(num_terms[&Term::Binary]),
                unaries: TermProbabilities::uniform_vector(num_terms[&Term::Unary]),
                terminals: TermProbabilities::uniform_vector(num_terms[&Term::Terminal]),
                numbers: TermProbabilities::uniform_vector(num_terms[&Term::Number]),
            },
            false => TermProbabilities {
                binaries: TermProbabilities::random_vector(num_terms[&Term::Binary]),
                unaries: TermProbabilities::random_vector(num_terms[&Term::Unary]),
                terminals: TermProbabilities::random_vector(num_terms[&Term::Terminal]),
                numbers: TermProbabilities::random_vector(num_terms[&Term::Number]),
            },
        }
    }

    pub fn from_hashmap(hashmap: HashMap<String, Vec<f64>>) -> TermProbabilities {
        let mut result = TermProbabilities {
            binaries: Vec::new(),
            unaries: Vec::new(),
            terminals: Vec::new(),
            numbers: Vec::new(),
        };

        assert!(hashmap.len() == 4, "Invalid hashmap length");

        for (key, value) in hashmap {
            match key.as_str() {
                "binaries" => assert!(value.len() == 6, "Invalid binaries vector length"),
                "unaries" => assert!(value.len() == 4, "Invalid unaries vector length"),
                "terminals" => assert!(value.len() == 6, "Invalid terminals vector length"),
                "numbers" => assert!(value.len() == 9, "Invalid numbers vector length"),
                _ => {
                    unreachable!("Invalid key '{}' in hashmap", key);
                }
            }

            match key.as_str() {
                "binaries" => result.binaries = value,
                "unaries" => result.unaries = value,
                "terminals" => result.terminals = value,
                "numbers" => result.numbers = value,
                _ => {
                    unreachable!("Invalid key '{}' in hashmap", key);
                }
            }
        }

        result
    }

    pub fn get(self: &Self, t: Term) -> &Vec<f64> {
        match t {
            Term::Binary => &self.binaries,
            Term::Unary => &self.unaries,
            Term::Terminal => &self.terminals,
            Term::Number => &self.numbers,
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
        normalize_vector(&mut vec);
        vec
    }

    pub fn crossover(&self, other: &Self) -> Self {
        let mut result = TermProbabilities {
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

    pub fn mutate(self, mut_prob: f64) -> Self {
        let mut copy = TermProbabilities { ..self };

        // With probability 'mut_prob', change the value of each element in
        // every vector to a random number between 0 and 1
        for i in 0..copy.binaries.len() {
            if fastrand::f64() < mut_prob {
                copy.binaries[i] = fastrand::f64();
            }
        }
        for i in 0..copy.unaries.len() {
            if fastrand::f64() < mut_prob {
                copy.unaries[i] = fastrand::f64();
            }
        }
        for i in 0..copy.terminals.len() {
            if fastrand::f64() < mut_prob {
                copy.terminals[i] = fastrand::f64();
            }
        }
        for i in 0..copy.numbers.len() {
            if fastrand::f64() < mut_prob {
                copy.numbers[i] = fastrand::f64();
            }
        }

        // Re-normalize so that sum(vec) = 1
        normalize_vector(&mut copy.binaries);
        normalize_vector(&mut copy.unaries);
        normalize_vector(&mut copy.terminals);
        normalize_vector(&mut copy.numbers);

        copy
    }

    pub fn get_operator_order(&self, operators: &str) -> Vec<String> {
        let result = match operators {
            "binaries" => vec!["plus", "div", "mul", "minus", "max", "min"],
            "unaries" => vec!["neg", "abs", "sqrt", "sqr"],
            "terminals" => vec!["x1", "x2", "y1", "y2", "deltaX", "deltaY"],
            "numbers" => vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"],
            _ => unreachable!(
                "Invalid operator type '{}' in get_operator_order",
                operators
            ),
        };

        result.iter().map(|s| s.to_string()).collect()
    }
}
