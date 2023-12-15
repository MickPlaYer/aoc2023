mod parser;
mod structs;

use parser::parse;
use std::collections::HashMap;

pub fn process_part1(input: String) -> Option<usize> {
    let map = parse(&input);
    let tilted_map = map.tilt_north();
    Some(tilted_map.count_total_load())
}

pub fn process_part2(input: String) -> Option<usize> {
    let mut cache = HashMap::new();
    let map = parse(&input);
    let mut current_map = map;
    for i in 0..1000000000 {
        if cache.contains_key(&current_map) {
            let last_hit = cache.get(&current_map).unwrap();
            let loop_len = i - last_hit;
            let needed_cycle_count = last_hit + (1000000000 - last_hit) % loop_len;
            for _ in 0..(needed_cycle_count - last_hit) {
                current_map = current_map
                    .tilt_north()
                    .tilt_west()
                    .tilt_south()
                    .tilt_east();
            }
            break;
        } else {
            let next_map = current_map
                .tilt_north()
                .tilt_west()
                .tilt_south()
                .tilt_east();
            cache.insert(current_map.clone(), i);
            current_map = next_map;
        }
    }
    Some(current_map.count_total_load())
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 14;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(136), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(107430), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(64), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(96317), answer);
    }
}
