use crate::structs::{MapSize, Tile};
use std::collections::HashMap;

fn get_start_point_tile(map: &HashMap<(isize, isize), Tile>, start: &(isize, isize)) -> Tile {
    let left_tile = map.get(&(start.0 - 1, start.1));
    let right_tile = map.get(&(start.0 + 1, start.1));
    let up_tile = map.get(&(start.0, start.1 - 1));
    let down_tile = map.get(&(start.0, start.1 + 1));
    let result = (
        if let Some(tile) = left_tile {
            tile.to_right()
        } else {
            false
        },
        if let Some(tile) = right_tile {
            tile.to_left()
        } else {
            false
        },
        if let Some(tile) = up_tile {
            tile.to_down()
        } else {
            false
        },
        if let Some(tile) = down_tile {
            tile.to_up()
        } else {
            false
        },
    );
    match result {
        (true, true, false, false) => Tile::Horizontal,
        (true, false, true, false) => Tile::NorthWest,
        (true, false, false, true) => Tile::SouthWest,
        (false, true, true, false) => Tile::NorthEast,
        (false, true, false, true) => Tile::SouthEast,
        (false, false, true, true) => Tile::Verticle,
        _ => panic!("invalid result: {:?}", result),
    }
}

pub fn parse(input: &str) -> ((isize, isize), HashMap<(isize, isize), Tile>, MapSize) {
    let mut start_point = None;
    let mut map = HashMap::new();
    input.lines().enumerate().for_each(|(j, line)| {
        line.chars().enumerate().for_each(|(i, c)| {
            let i = i as isize;
            let j = j as isize;
            if c == 'S' {
                start_point = Some((i, j));
            }
            map.insert((i, j), c.into());
        });
    });
    let start_point = start_point.unwrap();
    let tile = get_start_point_tile(&map, &start_point);
    map.insert(start_point, tile);
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    (start_point, map, MapSize::new(width, height))
}
