use crate::{map::util::{Map, Tile}, heuristic::evaluator::evaluate_heuristic, constants::EDGE_COST};
use super::state::State;
use crate::heuristic::parser::Heuristic;
use std::{collections::BinaryHeap, vec};
pub struct Problem<'a> {
    start: State,
    goal: State,
    open: BinaryHeap<State>,
    in_open: Vec<bool>,
    distance: Vec<i32>,
    parents: Vec<Option<usize>>,
    expanded: Vec<usize>,
    traversed: Vec<usize>,
    path: Vec<usize>,
    map: &'a Map,
    h: &'a Heuristic,
    solved: bool,
    complete: bool,
}

impl Problem<'_> {
    pub fn new<'a>(map: &'a Map, h: &'a Heuristic, start_pos: usize, goal_pos: usize) -> Problem<'a> {

        let (sx, sy) = map.ind2sub(start_pos);
        let (gx, gy) = map.ind2sub(goal_pos);
        let start = State::new(start_pos, 0, evaluate_heuristic(h, sx, sy, gx, gy));
        let goal = State::new(goal_pos, 0, evaluate_heuristic(h, gx, gy, gx, gy));

        // Create binary heap
        let mut open = BinaryHeap::new();
        open.push(start.clone());

        Problem {
            start: start.clone(),
            goal: goal.clone(),
            expanded: Vec::new(),
            traversed: Vec::new(),
            open,
            in_open:  vec![false; map.map.len()],
            distance: vec![i32::MAX; map.map.len()],
            parents: vec![None; map.map.len()],
            path: Vec::new(),
            map,
            h,
            solved: false,
            complete: false,
        }
    }

    pub fn solve(&mut self) -> (bool, bool) {
        while !self.solved && !self.complete {
            self.step();
        }

        (self.solved, self.complete)
    }

    // TODO: Record information on the expansions & traversals during search
    pub fn step(&mut self) -> () {
        // Don't do anything if the problem is solved
        if self.solved || self.complete {
            return;
        }

        if self.open.len() == 0 {
            self.complete = true;
            self.solved = false;
            return;
        }

        // Extract the state with the lowest f value
        let cur = self.open.pop().unwrap();
        self.in_open[cur.position] = false;

        if cur == self.goal {
            self.solved = true;
            self.complete = true;
            return;
        }

        self.expanded.push(cur.position);

        // Iterate over all neighbours
        for &neighbour in self.map.neighbours[cur.position].iter() {
            let new_g = cur.g + EDGE_COST;
            
            if new_g < self.distance[neighbour] {
                self.traversed.push(neighbour);
                let (gx, gy) = self.map.ind2sub(self.goal.position);
                let (nx, ny) = self.map.ind2sub(neighbour);
                let new_state = State::new(
                    neighbour,
                    new_g,
                    evaluate_heuristic(self.h, nx, ny, gx, gy),
                );

                // Improve estimate of distance
                self.distance[neighbour] = new_g;

                // Update parent
                self.parents[neighbour] = Some(cur.position);

                // Add new_state to the heap
                if !self.in_open[neighbour] {
                    self.open.push(new_state);
                    self.in_open[neighbour] = true;
                }
            }
        }
    }

    // Gets the completed path
    fn get_path(&mut self) -> Vec<usize> {
        if self.path.len() == 0 {
            let mut cur = self.goal.position;
            self.path.push(cur);

            while cur != self.start.position {
                cur = self.parents[cur].unwrap();

                self.path.push(cur);
            }
        }

        self.path.clone()
    }

    // Prints the completed search path on the map
    pub fn print_path_on_map(&mut self) -> () {
        let path = self.get_path();

        for i in 0..self.map.map.len() {
            if i == self.start.position {
                print!("S");
            } else if i == self.goal.position {
                print!("G");
            } else if path.contains(&i) {
                print!("+");
            } else {
                match self.map.map[i] {
                    Tile::Passable => print!("_"),
                    Tile::Unpassable => print!("."),
                }
            }

            // Add space for readability
            print!(" ");

            if (i + 1) % self.map.m == 0 {
                println!();
            }
        }
    }
}