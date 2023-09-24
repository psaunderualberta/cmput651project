use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct State {
    pub position: usize,
    pub g: i32,
    pub h: i32,
    pub f: i32,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for State {}

impl State {
    pub fn new(position: usize, g: i32, h: i32) -> State {
        State { position, g, h, f: g + h }
    }
}

// Implemented for min-heaps
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // First test 'f' value
        // Then test 'g' value
        // Then test position (tie-breaker, guranteed to work).
        (other.f).cmp(&self.f)
            .then_with(|| other.g.cmp(&self.g))
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}