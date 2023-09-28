use std::fs;
use std::io::{BufRead, BufReader};

use super::util::{Map, Tile};

pub fn parse_map_file(mapfile: &str) -> Map {
    // Read the contents of the map file
    let contents = fs::read_to_string(mapfile).expect("Map file {mapfile} doesn't exist!");

    // Parse the contents into a Map
    parse_map_string(&contents)
}

fn parse_map_string(mapstring: &str) -> Map {
    // Split the contents into lines
    let lines: Vec<String> = BufReader::new(mapstring.as_bytes())
        .lines()
        .map(|l| l.unwrap())
        .collect();

    // Double check that the first few lines are as expected
    assert_eq!(lines.len() >= 5, true);
    assert_eq!(lines[0], "type octile");
    assert_eq!(lines[1].starts_with("height "), true);
    assert_eq!(lines[2].starts_with("width "), true);
    assert_eq!(lines[3], "map");

    // Extract size of the map
    let n = lines[1][7..lines[1].len()].parse::<usize>().unwrap();
    let m = lines[2][6..lines[2].len()].parse::<usize>().unwrap();
    let mut map: Vec<Tile> = Vec::new();

    // Extract individual tiles from map
    for line in lines[4..lines.len()].iter() {
        for c in (*line).chars() {
            map.push(match c {
                '.' => Tile::Passable,
                'G' => Tile::Passable,
                '@' => Tile::Unpassable,
                'O' => Tile::Unpassable,
                'T' => Tile::Unpassable,
                'S' => Tile::Unpassable,
                'W' => Tile::Unpassable,
                _ => {
                    unreachable!("{c}")
                }
            })
        }
    }

    Map::from(n, m, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clone_sort(v: &Vec<usize>) -> Vec<usize> {
        let mut vc = v.clone();
        vc.sort();
        vc
    }

    #[test]
    fn test_parse_map_string_2x2() {
        let mapstring = "type octile\nheight 2\nwidth 2\nmap\n..\n..\n";
        let map = parse_map_string(mapstring);

        // Correct size
        assert_eq!(map.n, 2);
        assert_eq!(map.m, 2);

        // Correct map
        assert_eq!(map.map.len(), 4);
        assert_eq!(map.map, vec![Tile::Passable; 4]);

        // Correct neighbours
        assert_eq!(map.neighbours.len(), 4);
        assert_eq!(clone_sort(&map.neighbours[0]), vec![1, 2]);
        assert_eq!(clone_sort(&map.neighbours[1]), vec![0, 3]);
        assert_eq!(clone_sort(&map.neighbours[2]), vec![0, 3]);
        assert_eq!(clone_sort(&map.neighbours[3]), vec![1, 2]);
    }

    #[test]
    fn test_parse_map_string_3x3() {
        let mapstring = "type octile\nheight 3\nwidth 3\nmap\n...\n...\n...\n";
        let map = parse_map_string(mapstring);

        // Correct size
        assert_eq!(map.n, 3);
        assert_eq!(map.m, 3);

        // Correct map
        assert_eq!(map.map.len(), 9);
        assert_eq!(map.map, vec![Tile::Passable; 9]);

        // Correct neighbours
        assert_eq!(map.neighbours.len(), 9);
        assert_eq!(clone_sort(&map.neighbours[0]), vec![1, 3]);
        assert_eq!(clone_sort(&map.neighbours[1]), vec![0, 2, 4]);
        assert_eq!(clone_sort(&map.neighbours[2]), vec![1, 5]);
        assert_eq!(clone_sort(&map.neighbours[3]), vec![0, 4, 6]);
        assert_eq!(clone_sort(&map.neighbours[4]), vec![1, 3, 5, 7]);
        assert_eq!(clone_sort(&map.neighbours[5]), vec![2, 4, 8]);
        assert_eq!(clone_sort(&map.neighbours[6]), vec![3, 7]);
        assert_eq!(clone_sort(&map.neighbours[7]), vec![4, 6, 8]);
        assert_eq!(clone_sort(&map.neighbours[8]), vec![5, 7]);
    }

    #[test]
    fn test_parse_map_string_4x4_walls() {
        let mapstring = "type octile\nheight 4\nwidth 4\nmap\n@@@@\n@..@\n@.@@\n@@@@\n";
        let map = parse_map_string(mapstring);

        // Correct size
        assert_eq!(map.n, 4);
        assert_eq!(map.m, 4);

        // Correct map
        assert_eq!(map.map.len(), 16);
        assert_eq!(
            map.map,
            vec![
                Tile::Unpassable,
                Tile::Unpassable,
                Tile::Unpassable,
                Tile::Unpassable,
                Tile::Unpassable,
                Tile::Passable,
                Tile::Passable,
                Tile::Unpassable,
                Tile::Unpassable,
                Tile::Passable,
                Tile::Unpassable,
                Tile::Unpassable,
                Tile::Unpassable,
                Tile::Unpassable,
                Tile::Unpassable,
                Tile::Unpassable,
            ]
        );

        // Correct neighbours
        assert_eq!(map.neighbours.len(), 16);
        assert_eq!(clone_sort(&map.neighbours[0]), vec![]);
        assert_eq!(clone_sort(&map.neighbours[1]), vec![]);
        assert_eq!(clone_sort(&map.neighbours[2]), vec![]);
        assert_eq!(clone_sort(&map.neighbours[3]), vec![]);
        assert_eq!(clone_sort(&map.neighbours[4]), vec![]);
        assert_eq!(clone_sort(&map.neighbours[5]), vec![6, 9]);
        assert_eq!(clone_sort(&map.neighbours[6]), vec![5]);
        assert_eq!(clone_sort(&map.neighbours[7]), vec![]);
        assert_eq!(clone_sort(&map.neighbours[8]), vec![]);
        assert_eq!(clone_sort(&map.neighbours[9]), vec![5]);
        assert_eq!(clone_sort(&map.neighbours[10]), vec![]);
        assert_eq!(clone_sort(&map.neighbours[11]), vec![]);
        assert_eq!(clone_sort(&map.neighbours[12]), vec![]);
        assert_eq!(clone_sort(&map.neighbours[13]), vec![]);
        assert_eq!(clone_sort(&map.neighbours[14]), vec![]);
        assert_eq!(clone_sort(&map.neighbours[15]), vec![]);
    }
}
