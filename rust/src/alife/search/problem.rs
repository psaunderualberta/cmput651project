use crate::{map::util::Map, heuristic::evaluator::evaluate_heuristic, constants::EDGE_COST};
use super::state::State;
use crate::heuristic::parser::Heuristic;
use std::{collections::BinaryHeap, vec};
pub struct Problem<'a> {
    start: State,
    goal: State,
    cur: State,
    open: BinaryHeap<State>,
    closed: Vec<bool>,
    distance: Vec<i32>,
    parents: Vec<Option<usize>>,
    expanded: Vec<usize>,
    traversed: Vec<usize>,
    path: Vec<usize>,
    map: &'a Map,
    neighbours: Vec<usize>,
    neighbour_index: usize,
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
            cur: start,
            expanded: Vec::new(),
            traversed: Vec::new(),
            open,
            closed:  vec![false; map.map.len()],
            distance: vec![i32::MAX; map.map.len()],
            parents: vec![None; map.map.len()],
            path: Vec::new(),
            map,
            neighbours: Vec::new(),
            neighbour_index: usize::MAX,
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

    pub fn step(&mut self) -> () {
        // Don't do anything if the problem is solved
        if self.solved || self.complete {
            return;
        }

        // No more nodes to expand, problem is not solvable
        if self.open.len() == 0 {
            self.complete = true;
            self.solved = false;
            return;
        }

        // If we have exhausted all neighbours of the current state,
        // pop a new state from the heap
        if self.neighbour_index >= self.neighbours.len() {
            self.cur = self.open.pop().unwrap();

            // If we have reached the goal, we are done
            if self.cur == self.goal {
                self.solved = true;
                self.complete = true;
                return;
            }

            self.neighbours = self.map.neighbours[self.cur.position].clone();
            self.neighbour_index = 0;

            // If we've found a better way to 'cur', ignore it and
            // force new node expansion on next generation
            if self.cur.f > self.distance[self.cur.position] {
                self.neighbour_index = self.neighbours.len() + 1;
                return;
            }

            self.expanded.push(self.cur.position);
        }

        // Get the new node, update the neighbour index
        let new_pos = self.neighbours[self.neighbour_index];
        let (new_x, new_y) = self.map.ind2sub(new_pos);
        self.neighbour_index += 1;

        // TODO: Only compute once
        let (gx, gy) = self.map.ind2sub(self.goal.position);

        // Create the new state
        let new_state = State::new(
            new_pos,
            self.cur.g + EDGE_COST,
            evaluate_heuristic(self.h, new_x, new_y, gx, gy),
        );

        // // If so, add it to the frontier and continue
        if new_state.f < self.distance[new_pos] {
            // Improve estimate of distance
            self.distance[new_pos] = new_state.f;

            // Add new_state to the heap
            self.open.push(new_state);

            // Update parent
            self.parents[new_pos] = Some(self.cur.position);
        }
    }
}