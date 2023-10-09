use std::{collections::HashSet, fmt::Display};

// Enumeration for possible maps on which to search.
pub enum Maps {
    Den009d,
    Den312d,
    Orz103d,
}

impl Maps {
    pub fn path(&self) -> &str {
        match *self {
            Maps::Den009d => "./src/map/data/den009d.map",
            Maps::Den312d => "./src/map/data/den312d.map",
            Maps::Orz103d => "./src/map/data/orz103d.map",
        }
    }

    pub fn name2path(name: &str) -> &str {
        match name {
            "den009d" => Maps::Den009d.path(),
            "den312d" => Maps::Den312d.path(),
            "orz103d" => Maps::Orz103d.path(),
            n => { panic!("{n} is not a known map name!")}
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
    pub fn from(n: usize, m: usize, mut map: Vec<Tile>) -> Map {
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

        (map, neighbours) = trim_map_to_largest_connected_component(map, neighbours);

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
                Tile::Passable => "·",
                Tile::Unpassable => "■",
            });

            result.push_str(" ");

            if (i + 1) % self.m == 0 {
                result.push('\n');
            }
        }

        // Return the result
        write!(f, "{}", result)
    }
}

fn trim_map_to_largest_connected_component(
    mut map: Vec<Tile>,
    mut neighbours: Vec<Vec<usize>>,
) -> (Vec<Tile>, Vec<Vec<usize>>) {
    let mut visited: Vec<bool> = vec![false; map.len()];
    let mut queue: Vec<usize> = Vec::new();
    let mut largest_component: HashSet<usize> = HashSet::new();

    for i in 0..map.len() {
        if visited[i] || map[i] == Tile::Unpassable {
            continue;
        }

        assert!(queue.is_empty());
        queue.push(i);
        visited[i] = true;

        let mut component = HashSet::new();

        // DFS to find connected component.
        while !queue.is_empty() {
            let current = queue.pop().unwrap();
            component.insert(current);

            for neighbour in &neighbours[current] {
                if !visited[*neighbour] {
                    queue.push(*neighbour);
                    visited[*neighbour] = true;
                }
            }
        }

        if largest_component.len() < component.len() {
            let tmp = largest_component;
            largest_component = component;
            component = tmp;
        };

        // 'component' is not the largest component, so remove its tiles from the map
        for tile in component.iter() {
            map[*tile] = Tile::Unpassable;
            neighbours[*tile] = Vec::new();
        }
    }

    (map, neighbours)
}

// TODO: Implement tests for map
