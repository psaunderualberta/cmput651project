use std::collections::BinaryHeap;

use super::state::State;
use crate::heuristic::parser::HeuristicNode;
use crate::{
    constants::EDGE_COST,
    heuristic::executors::interpreter::Interpreter,
    heuristic::executors::HeuristicExecuter,
    heuristic::Heuristic,
    map::util::{Map, Tile},
};
use colored::*;

pub struct Problem<'a> {
    executer: Interpreter,
    start: State,
    goal: State,
    open: BinaryHeap<State>,
    closed: Vec<bool>,
    g: Vec<Option<f32>>,
    parents: Vec<Option<usize>>,
    expanded: Vec<usize>,
    traversed: Vec<usize>,
    path: Vec<usize>,
    map: &'a Map,
    solved: bool,
    complete: bool,
}

impl Problem<'_> {
    pub fn new<'a>(
        map: &'a Map,
        h: &'a HeuristicNode,
        start_pos: usize,
        goal_pos: usize,
    ) -> Problem<'a> {
        let (sx, sy) = map.ind2sub(start_pos);
        let (gx, gy) = map.ind2sub(goal_pos);
        let (sx, sy, gx, gy) = (sx as f32, sy as f32, gx as f32, gy as f32);
        let executor = Interpreter::create(&Heuristic { root: h.clone() });
        let start = State::new(start_pos, 0.0, executor.execute(sx, sy, gx, gy));
        let goal = State::new(goal_pos, 0.0, executor.execute(gx, gy, gx, gy));

        // TODO: Try creating entire distance array as positive? Then only place references into the open list?

        // Create priority queue
        let mut open = BinaryHeap::new();
        open.push(start.clone());

        // Create closed list
        let mut g = vec![None; map.map.len()];
        g[start.position] = Some(start.g);

        Problem {
            executer: executor,
            start: start.clone(),
            goal: goal.clone(),
            expanded: Vec::new(),
            traversed: Vec::new(),
            open,
            closed: vec![false; map.map.len()],
            g,
            parents: vec![None; map.map.len()],
            path: Vec::new(),
            map,
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

        if cur == self.goal {
            self.solved = true;
            self.complete = true;
            return;
        }

        // Determine if there's a better path to this node
        let cur_g = self.g[cur.position].unwrap();
        if cur_g != cur.g {
            return;
        }

        self.closed[cur.position] = true;
        self.expanded.push(cur.position);

        // Iterate over all neighbours
        for &neighbour in self.map.neighbours[cur.position].iter() {
            if self.closed[neighbour] {
                continue;
            }

            let new_g = cur_g + EDGE_COST;
            self.traversed.push(neighbour);

            let (gx, gy) = self.map.ind2sub(self.goal.position);
            let (nx, ny) = self.map.ind2sub(neighbour);

            let (gx, gy, nx, ny) = (gx as f32, gy as f32, nx as f32, ny as f32);
            let new_h = self.executer.execute(nx, ny, gx, gy);

            let new_state = State::new(neighbour, new_g, new_h);
            if self.g[neighbour].is_none() || new_g < self.g[neighbour].unwrap() {
                // Update parent
                self.parents[neighbour] = Some(cur.position);

                self.g[neighbour] = Some(new_g);
                self.open.push(new_state);
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
                print!("{}", "S".blue().bold());
            } else if i == self.goal.position {
                print!("{}", "G".green().bold());
            } else if path.contains(&i) {
                print!("{}", "+".yellow());
            } else {
                match self.map.map[i] {
                    // ■ ▣ ▢ • ·
                    Tile::Passable => print!("·"),
                    Tile::Unpassable => print!("■"),
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
