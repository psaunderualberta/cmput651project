use std::fmt::Display;

// Enumeration for possible maps on which to search.
pub enum Maps {
    Den009d,
    Den312d,
    Orz103d
}

impl Maps {
    pub fn value(&self) -> &str {
        match *self {
            Maps::Den009d => "./src/map/data/den009d.map",
            Maps::Den312d => "./src/map/data/den312d.map",
            Maps::Orz103d => "./src/map/data/orz103d.map"
        }
    }
}

// . - passable terrain
// G - passable terrain
// @ - out of bounds
// O - out of bounds
// T - trees (unpassable)
// S - swamp (passable from regular terrain)
// W - water (traversable, but not passable from terrain)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Passable,
    Unpassable,
}

pub struct Map {
    pub n: usize,
    pub m: usize,
    pub map: Vec<Tile>,
    pub neighbours: Vec<Vec<usize>>,
}

impl Map {
    pub fn from(n: usize, m: usize, map: Vec<Tile>) -> Map {
        let mut neighbours: Vec<Vec<usize>> = Vec::new();

        for i in 0..map.len() {
            // Add a new level
            neighbours.push(Vec::new());

            // Only add neighbours if traversable
            if map[i] == Tile::Unpassable {
                continue;
            }

            // Can go left
            if i % m != 0 && map[i - 1] == Tile::Passable {
                neighbours[i].push(i - 1);
            }

            // Can go right
            if (i + 1) % m != 0 && map[i + 1] == Tile::Passable {
                neighbours[i].push(i + 1);
            }

            // Can go up
            if i >= m && map[i - m as usize] == Tile::Passable {
                neighbours[i].push(i - m as usize);
            }

            // Can go down
            if i < (n - 1) * m && map[i + m as usize] == Tile::Passable {
                neighbours[i].push(i + m as usize);
            }
        }

        Map {
            n,
            m,
            map,
            neighbours,
        }
    }

    pub fn ind2sub(&self, pos: usize) -> (usize, usize) {
        (pos / self.m, pos % self.m)
    }

    pub fn sub2ind(&self, x: usize, y: usize) -> usize {
        x * self.m + y
    }

    pub fn random_free_position(&self) -> usize {
        let mut pos = fastrand::choice(0..self.map.len()).unwrap();

        while self.map[pos] == Tile::Unpassable {
            pos = fastrand::choice(0..self.map.len()).unwrap();
        }

        pos
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write height & width
        let mut result = format!("Height: {}\n Width: {}\n", self.n, self.m);

        // Write the map itself
        for i in 1..self.map.len() {
            result.push_str(match self.map[i] {
                Tile::Passable => "_ ",
                Tile::Unpassable => ". ",
            });

            if (i + 1) % self.m == 0 {
                result.push('\n');
            }
        }

        // Return the result
        write!(f, "{}", result)
    }
}

// TODO: Implement tests for map
