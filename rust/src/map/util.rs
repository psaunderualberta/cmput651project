use std::fmt::Display;

// Enumeration for possible maps on which to search.
pub enum Maps {
    Den009d,
}

impl Maps {
    pub fn value(&self) -> &str {
        match *self {
            Maps::Den009d => "./src/map/data/den009d.map",
            _ => { unreachable!() }
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
#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Passable,
    Unpassable
}

pub struct Map {
    pub n: i32,
    pub m: i32,
    pub map: Vec<Tile>,
    pub neighbours: Vec<Vec<usize>>,
}

impl Map {
    pub fn from(n: i32, m: i32, map: Vec<Tile>) -> Map {
        let mut neighbours: Vec<Vec<usize>> = Vec::new();

        for i in 0..map.len() {
            let ii32 = i as i32;

            // Add a new level
            neighbours.push(Vec::new());
            
            // Only add neighbours if traversable
            if map[i] == Tile::Unpassable {
                continue;
            }

            // Can go left
            if ii32 % n != 0 {
                neighbours[i].push(i - 1);
            }

            // Can go right
            if (ii32 + 1) % n != 0 {
                neighbours[i].push(i + 1);
            }

            // Can go up
            if ii32 >= m {
                neighbours[i].push(i - m as usize);
            }

            if ii32 < (n - 1) * m {
                neighbours[i].push(i + m as usize);
            }
        }

        Map { n, m, map, neighbours }
    }

    pub fn ind2sub(&self, pos: i32) -> (i32, i32) {
        (pos / self.n, pos % self.m)
    }

    pub fn get_neighbours(&self, pos: i32) -> &Vec<usize> {
        &self.neighbours[pos as usize]
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write height & width
        let mut result = format!("Height: {}\n Width: {}\n", self.n, self.m);

        // Write the map itself
        for i in 1..self.map.len() {
            result.push_str(match self.map[i] {
                Tile::Passable => "0 ",
                Tile::Unpassable => "1 ",
            });

            if (i as i32) % self.m == 0 {
                result.push('\n');
            }
        }

        // Return the result
        write!(f, "{}", result)
    }
}

// TODO: Implement tests for map