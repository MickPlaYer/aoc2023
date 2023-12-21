mod parser;
mod structs;

use parser::parse;
use std::collections::HashMap;
use structs::Map;

pub fn process_part1(input: String, steps: usize) -> Option<usize> {
    let is_odd_or_even = steps % 2;
    let map = parse(&input);
    let points = map.find_points(None);
    let points_count = points
        .into_iter()
        .filter(|(_, s)| s % 2 == is_odd_or_even && *s <= steps)
        .count();
    Some(points_count)
}

pub fn process_part2(input: String, steps: usize) -> Option<usize> {
    let map = parse(&input);
    let original_map_size = map.get_size();
    let size = 5;
    let n_for_test = (size - 1) / 2;
    let map = map.make_bigger(size);
    let one_chunk_size = original_map_size.0;
    let extra_size = (original_map_size.0 - 1) / 2;
    let counts = find_count_array(
        size,
        extra_size + one_chunk_size * n_for_test,
        original_map_size,
        &map,
    );
    let group_a_size = counts[2][0] + counts[4][2] + counts[2][4] + counts[0][2];
    let group_b_size = counts[3][1] + counts[3][3] + counts[1][3] + counts[1][1];
    let group_c_size = counts[3][0] + counts[3][4] + counts[1][4] + counts[1][0];
    let even_block_size = counts[2][1];
    let odd_block_size = counts[2][2];
    let n_from_input = (steps - extra_size) / one_chunk_size;
    let group_b_group_count = n_from_input - 1;
    let group_c_group_count = n_from_input;
    let even_block_count = n_from_input * n_from_input;
    let odd_block_count = (n_from_input - 1) * (n_from_input - 1);
    Some(
        group_a_size
            + group_b_size * group_b_group_count
            + group_c_size * group_c_group_count
            + even_block_size * even_block_count
            + odd_block_size * odd_block_count,
    )
}

fn find_count_array(
    size: usize,
    steps: usize,
    map_size: (usize, usize),
    map: &Map,
) -> Vec<Vec<usize>> {
    let is_odd_or_even = steps % 2;
    let points = map
        .find_points(None)
        .into_iter()
        .filter(|(_, s)| s % 2 == is_odd_or_even && *s <= steps)
        .collect::<HashMap<_, _>>();
    print_points_with_map(&map, &points);
    let mut counts = Vec::new();
    for y in 0..size {
        let mut row = Vec::new();
        for x in 0..size {
            let count = count_chunk((x, y), map_size, &points);
            print!("{}, ", count);
            row.push(count);
        }
        counts.push(row);
        println!()
    }
    counts
}

fn count_chunk(
    chunk: (usize, usize),
    map_size: (usize, usize),
    points: &HashMap<(usize, usize), usize>,
) -> usize {
    let mut count = 0;
    let min_y = map_size.1 * chunk.1;
    let max_y = min_y + map_size.1;
    let min_x = map_size.0 * chunk.0;
    let max_x = min_x + map_size.0;
    for y in min_y..max_y {
        for x in min_x..max_x {
            if let Some(_) = points.get(&(x, y)) {
                count += 1;
            }
        }
    }
    count
}

fn print_points_with_map(map: &structs::Map, points: &HashMap<(usize, usize), usize>) {
    let map_size = map.get_size();
    for y in 0..map_size.1 {
        for x in 0..map_size.0 {
            if let Some(step) = points.get(&(x, y)) {
                if *step >= 10 {
                    print!("O");
                } else {
                    print!("{}", step)
                }
            } else {
                print!("{}", map.get(&(x, y)).unwrap());
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 21;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample_1s() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content, 6);
        assert_eq!(Some(16), answer);
    }

    #[test]
    fn process_part1_with_input_1i() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content, 64);
        assert_eq!(Some(3649), answer);
    }

    #[test]
    fn process_part2_with_input_2i() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content, 26501365);
        assert_eq!(Some(612941134797232), answer);
    }
}
