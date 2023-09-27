use std::{cmp::Ordering, hash::Hash};

#[derive(Clone, Debug)]
pub struct State {
    pub position: usize,
    pub g: f32,
    pub h: f32,
    pub f: f32,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for State {}

impl State {
    pub fn new(position: usize, g: f32, h: f32) -> State {
        State {
            position,
            g,
            h,
            f: g + h,
        }
    }
}

// Implemented for min-heaps
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.f < other.f {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}