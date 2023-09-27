use crate::{heuristic::parser::HeuristicNode, map::util::Map};

use super::problem::{AStar, Problem};

pub struct ProblemCycle<'a> {
    problems: Vec<Problem<'a>>,
    problem_index: usize,
}

impl ProblemCycle<'_> {
    pub fn new(map: &'_ Map, num_problems: usize) -> ProblemCycle<'_> {
        let mut problems = Vec::new();
        let original_start = map.random_free_position();
        let mut start = original_start;

        for _ in 0..num_problems - 1 {
            // Ensure the problem is not trivial
            let mut goal = map.random_free_position();
            while goal == start {
                goal = map.random_free_position();
            }

            problems.push(Problem { map, start, goal });
            start = goal;
        }

        // Push the final problem, to create an actual 'cycle'
        problems.push(Problem {
            map,
            start,
            goal: original_start,
        });

        ProblemCycle {
            problems,
            problem_index: 0,
        }
    }
}

pub struct AStarCycle<'a> {
    astars: Vec<AStar<'a>>,
    astar_index: usize,
    astar_expansions: Vec<Option<usize>>,
    current_step: usize,
}

impl AStarCycle<'_> {
    pub fn new<'a>(map: &'a Map, h: &'a HeuristicNode, num_problems: usize) -> AStarCycle<'a> {
        let pcycle = ProblemCycle::new(map, num_problems);
        Self::from_cycle(pcycle, h)
    }

    pub fn from_cycle<'a>(cycle: ProblemCycle<'a>, h: &'a HeuristicNode) -> AStarCycle<'a> {
        let mut astars = Vec::new();
        for problem in cycle.problems.iter() {
            astars.push(AStar::from_problem(problem, h))
        }

        let astar_expansions = vec![None; astars.len()];
        AStarCycle {
            astars,
            astar_index: 0,
            astar_expansions,
            current_step: 0,
        }
    }

    pub fn solve_cycle(&mut self) -> () {
        let mut num_solved = 0;
        while num_solved != self.astars.len() {
            num_solved += self.step() as usize;
        }
    }

    // Return value is whether the 'step' resulted in a problem being solved
    pub fn step(&mut self) -> bool {
        let just_solved = match self.astars[self.astar_index].is_complete() {
            true => self.step_mimic(),
            false => self.step_actual(),
        };

        // If we just finished solving a problem, update the expansions vector
        // and go to the next problem
        if just_solved {
            let expansions = self.astars[self.astar_index].get_num_expansions();
            self.astar_expansions[self.astar_index] = Some(expansions);
            self.astar_index = (self.astar_index + 1) % self.astars.len();
        }

        just_solved
    }

    // The current problem is not solved
    fn step_actual(&mut self) -> bool {
        self.astars[self.astar_index].step();
        // println!("{}", self.astars[self.astar_index].get_num_expansions());

        // Return whether or not we just completed the problem
        self.astars[self.astar_index].is_complete()
    }

    fn step_mimic(&mut self) -> bool {
        // Sanity check to ensure we have not exceeded the # of steps
        // to solve the problem
        let expansions = self.astar_expansions[self.astar_index].unwrap();
        assert_eq!(self.current_step <= expansions, true);

        // Mimic a step w/o actually changing the problem, since we know that its solved
        self.current_step += 1;

        if self.current_step > self.astar_expansions[self.astar_index].unwrap() {
            self.current_step = 0;
            return true;
        }

        false
    }
}
