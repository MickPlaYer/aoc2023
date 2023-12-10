mod parser;
mod structs;

use parser::parse;
use std::collections::HashMap;
use structs::{Direction, MapSize, Tile};

fn print_map(map: &HashMap<(isize, isize), Tile>, map_size: &MapSize) {
    (0..map_size.height).for_each(|y| {
        (0..map_size.width).for_each(|x| {
            let x = x as isize;
            let y = y as isize;
            let tile = map.get(&(x, y)).unwrap();
            print!("{:?}", tile);
        });
        print!("\n");
    });
}

fn get_loop_points(
    start_point: &(isize, isize),
    map: &HashMap<(isize, isize), Tile>,
) -> Vec<(isize, isize)> {
    let mut loop_points = Vec::new();
    let mut last_point = *start_point;
    let mut last_tile = map.get(&start_point).unwrap();
    let mut last_direction = last_tile.default_direction();
    loop {
        let next_direction = last_tile.connect_to(&last_direction);
        let next_point = match next_direction {
            Direction::Up => (last_point.0, last_point.1 - 1),
            Direction::Down => (last_point.0, last_point.1 + 1),
            Direction::Left => (last_point.0 - 1, last_point.1),
            Direction::Right => (last_point.0 + 1, last_point.1),
        };
        if next_point == *start_point {
            break;
        } else {
            loop_points.push(next_point);
            last_point = next_point;
            last_tile = map.get(&next_point).unwrap();
            last_direction = next_direction.invert();
        }
    }
    loop_points
}

pub fn process_part1(content: String) -> Option<usize> {
    let (start_point, map, map_size) = parse(&content);
    print_map(&map, &map_size);
    let loop_points = get_loop_points(&start_point, &map);
    Some(loop_points.len() / 2 + 1)
}

pub fn process_part2(content: String) -> Option<usize> {
    let (start_point, map, map_size) = parse(&content);
    print_map(&map, &map_size);
    let width = map_size.width as isize;
    let height = map_size.height as isize;
    let mut loop_map = HashMap::new();
    loop_map.insert(start_point, true);
    get_loop_points(&start_point, &map)
        .into_iter()
        .for_each(|e| {
            loop_map.insert(e, true);
        });
    let mut points = Vec::new();
    (0..height).for_each(|y| {
        (0..width).for_each(|x| {
            let point = (x, y);
            if loop_map.get(&point).is_some() {
                let tile = map.get(&(x, y)).unwrap();
                print!("{:?}", tile);
                return;
            }
            let is_pipe_in_loop_or_ground_on_x = |x| {
                let point = (x, y);
                if loop_map.get(&point).is_some() {
                    map.get(&(x, y)).unwrap().clone()
                } else {
                    Tile::Ground
                }
            };
            let is_pipe_in_loop_or_ground_on_y = |y| {
                let point = (x, y);
                if loop_map.get(&point).is_some() {
                    map.get(&(x, y)).unwrap().rotate()
                } else {
                    Tile::Ground
                }
            };
            let left_tiles = (0..x).map(is_pipe_in_loop_or_ground_on_x).collect();
            let right_tiles = (x..width).map(is_pipe_in_loop_or_ground_on_x).collect();
            let up_tiles = (0..y).map(is_pipe_in_loop_or_ground_on_y).collect();
            let down_tiles = (y..height).map(is_pipe_in_loop_or_ground_on_y).collect();
            let left_count = count_pipe_h(left_tiles);
            let right_count = count_pipe_h(right_tiles);
            let up_count = count_pipe_h(up_tiles);
            let down_count = count_pipe_h(down_tiles);
            let result = (left_count % 2 == 1)
                && (right_count % 2 == 1)
                && (up_count % 2 == 1)
                && (down_count % 2 == 1);
            print!("{}", if result { "T" } else { "F" });
            if result {
                points.push(point);
            }
        });
        print!("\n");
    });
    Some(points.len())
}

fn count_pipe_h(tiles: Vec<Tile>) -> usize {
    let mut count = 0;
    let mut in_pipe = false;
    let mut from_up = false;
    tiles.into_iter().for_each(|tile| {
        if !tile.is_pipe() {
            return;
        }
        if in_pipe {
            if tile.to_left() && !tile.to_right() {
                if (tile.to_up() && from_up) || (tile.to_down() && !from_up) {
                    count -= 1;
                }
                in_pipe = false;
            }
        } else {
            count += 1;
            if tile.to_right() {
                from_up = tile.to_up();
                in_pipe = true;
            }
        }
    });
    count
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 10;
    use super::*;
    use shared::{read_input, read_sample, read_sample2};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(4), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(6979), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample2(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(10), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(443), answer);
    }
}
