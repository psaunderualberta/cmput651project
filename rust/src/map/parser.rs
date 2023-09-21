use std::fs;

use super::util::{Map, Tile};

pub fn parse_map_file(mapfile: &str) -> Map {
    // Read the contents of the map file
    let contents = fs::read_to_string(mapfile)
            .expect("Map file {mapfile} doesn't exist!");

    // Double check that the first few lines are as expected
    let lines = Vec::from_iter(contents.split('\n'));
    assert_eq!(lines.len() >= 5, true);
    assert_eq!(lines[0], "type octile");
    assert_eq!(lines[1].starts_with("height "), true);
    assert_eq!(lines[2].starts_with("width "), true);
    assert_eq!(lines[3], "map");

    // Extract size of the map
    let n = lines[1][7..lines[1].len()].parse::<i32>().unwrap();
    let m = lines[2][6..lines[2].len()].parse::<i32>().unwrap();
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
                _ => { unreachable!("{c}")}
            })
        }
    }

    Map::from(n, m, map)
}