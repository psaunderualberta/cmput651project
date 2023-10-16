use std::collections::BinaryHeap;

use super::state::State;
use pyo3::prelude::*;

use crate::{
    constants::EDGE_COST,
    map::util::{Map, Tile},
};
use colored::*;

#[derive(Clone)]
pub struct Problem {
    pub start: usize,
    pub goal: usize,
}

#[derive(Clone)]
#[pyclass]
pub struct ProblemResult {
    #[pyo3(get)]
    pub expansions: Vec<usize>,
    #[pyo3(get)]
    pub num_traversals: usize,
    #[pyo3(get)]
    pub solution_path: Vec<usize>,
    #[pyo3(get)]
    pub solved: bool,
}

impl Problem {
    pub fn new(start: usize, goal: usize) -> Problem {
        Problem { start, goal }
    }

    pub fn solve(&self, map: &Map, executor: impl Fn(f32, f32, f32, f32) -> f32) -> ProblemResult {
        let (sx, sy) = map.ind2sub(self.start);
        let (gx, gy) = map.ind2sub(self.goal);
        let (sx, sy, gx, gy) = (sx as f32, sy as f32, gx as f32, gy as f32);
        let start = State::new(self.start, 0.0, executor(sx, sy, gx, gy));

        // Create priority queue
        let mut open = BinaryHeap::new();
        open.push(start.clone());

        // Create distance array 'g'
        let mut g = vec![None; map.map.len()];
        g[start.position] = Some(start.g);

        // create closed list
        let mut closed = vec![false; map.map.len()];

        // create parent graph
        let mut parents = vec![None; map.map.len()];

        let mut expansions = Vec::new();
        let mut num_traversals = 0;
        let mut solved = false;

        while !open.is_empty() {
            // Extract the state with the lowest f value
            let cur = open.pop().unwrap();
            if cur.position == self.goal {
                solved = true;
                break;
            }

            // Determine if there's a better path to this node
            let cur_g = g[cur.position].unwrap();
            if cur_g != cur.g {
                continue;
            }

            closed[cur.position] = true;
            expansions.push(cur.position);

            // Iterate over all neighbours
            for &neighbour in map.neighbours[cur.position].iter() {
                if closed[neighbour] {
                    continue;
                }

                let new_g = cur_g + EDGE_COST;
                num_traversals += 1;

                let (gx, gy) = map.ind2sub(self.goal);
                let (nx, ny) = map.ind2sub(neighbour);

                let (gx, gy, nx, ny) = (gx as f32, gy as f32, nx as f32, ny as f32);
                let new_h = executor(nx, ny, gx, gy);

                let new_state = State::new(neighbour, new_g, new_h);
                if g[neighbour].is_none() || new_g < g[neighbour].unwrap() {
                    // Update parent
                    parents[neighbour] = Some(cur.position);

                    g[neighbour] = Some(new_g);
                    open.push(new_state);
                }
            }
        }

        let solution_path = self.get_path(parents);
        ProblemResult {
            expansions,
            num_traversals,
            solution_path,
            solved,
        }
    }

    // Gets the completed path
    fn get_path(&self, parents: Vec<Option<usize>>) -> Vec<usize> {
        let mut path = Vec::new();
        if path.is_empty() {
            let mut cur = self.goal;
            path.push(cur);

            while cur != self.start {
                cur = parents[cur].unwrap();

                path.push(cur);
            }
        }

        path.clone()
    }

    // Prints the completed search path on the map
    pub fn print_path_on_map(&self, map: &Map, path: Vec<usize>) {
        for i in 0..map.map.len() {
            if i == self.start {
                print!("{}", "S".blue().bold());
            } else if i == self.goal {
                print!("{}", "G".green().bold());
            } else if path.contains(&i) {
                print!("{}", "+".yellow());
            } else {
                match map.map[i] {
                    // ■ ▣ ▢ • ·
                    Tile::Passable => print!("·"),
                    Tile::Unpassable => print!("■"),
                }
            }

            // Add space for readability
            print!(" ");

            if (i + 1) % map.m == 0 {
                println!();
            }
        }
    }

    pub fn print_expansions(&self, map: &Map, result: &ProblemResult) {
        for i in 0..map.map.len() {
            if i == self.start {
                print!("{}", "S".blue().bold());
            } else if i == self.goal {
                print!("{}", "G".green().bold());
            } else if result.solution_path.contains(&i) {
                print!("{}", "+".yellow());
            } else if result.expansions.contains(&i) {
                print!("{}", "x".red());
            } else {
                match map.map[i] {
                    // ■ ▣ ▢ • ·
                    Tile::Passable => print!("·"),
                    Tile::Unpassable => print!("■"),
                }
            }

            // Add space for readability
            print!(" ");

            if (i + 1) % map.m == 0 {
                println!();
            }
        }
    }
}
