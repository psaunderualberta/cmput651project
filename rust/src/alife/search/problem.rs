use super::state::State;
use crate::heuristic::parser::HeuristicNode;
use crate::{
    constants::EDGE_COST,
    heuristic::executors::interpreter::Interpreter,
    heuristic::executors::jit::Jit,
    heuristic::executors::HeuristicExecuter,
    heuristic::Heuristic,
    map::util::{Map, Tile},
};
use colored::*;
use std::{collections::BinaryHeap, vec};
pub struct Problem<'a> {
    // pub executer: Interpreter,
    pub executer: Jit<'a>,
    start: State,
    goal: State,
    open: BinaryHeap<State>,
    in_open: Vec<bool>,
    distance: Vec<f32>,
    parents: Vec<Option<usize>>,
    expanded: Vec<usize>,
    traversed: Vec<usize>,
    path: Vec<usize>,
    map: &'a Map,
    h: &'a HeuristicNode,
    solved: bool,
    complete: bool,
}

impl Problem<'_> {
    pub fn new<'a>(
        map: &'a Map,
        h: &'a HeuristicNode,
        start_pos: usize,
        goal_pos: usize,
        context: &'a inkwell::context::Context,
    ) -> Problem<'a> {
        let (sx, sy) = map.ind2sub(start_pos);
        let (gx, gy) = map.ind2sub(goal_pos);
        let (sx, sy, gx, gy) = (sx as f32, sy as f32, gx as f32, gy as f32);
        // let executor = Interpreter::create(&Heuristic { root: h.clone() });
        let executor = Jit::create(&Heuristic { root: h.clone() }, context);
        let start = State::new(start_pos, 0.0, executor.execute(sx, sy, gx, gy));
        let goal = State::new(goal_pos, 0.0, executor.execute(sx, sy, gx, gy));

        // Create binary heap
        let mut open = BinaryHeap::new();
        open.push(start.clone());

        Problem {
            executer: executor,
            start: start.clone(),
            goal: goal.clone(),
            expanded: Vec::new(),
            traversed: Vec::new(),
            open,
            in_open: vec![false; map.map.len()],
            distance: vec![f32::MAX; map.map.len()],
            parents: vec![None; map.map.len()],
            path: Vec::new(),
            map,
            h,
            solved: false,
            complete: false,
        }
    }

    pub fn reset(&mut self) {
        self.open = BinaryHeap::new();
        self.open.push(self.start.clone());
        self.in_open = vec![false; self.map.map.len()];
        self.distance = vec![f32::MAX; self.map.map.len()];
        self.parents = vec![None; self.map.map.len()];
        self.expanded = Vec::new();
        self.traversed = Vec::new();
        self.path = Vec::new();
        self.solved = false;
        self.complete = false;
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
                let (gx, gy, nx, ny) = (gx as f32, gy as f32, nx as f32, ny as f32);
                let new_state = State::new(neighbour, new_g, self.executer.execute(nx, ny, gx, gy));

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
