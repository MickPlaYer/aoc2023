mod parser;

use parser::parse;
use shared::lcm_n;
use std::{collections::HashMap, ops::ControlFlow};

pub fn process_part1(content: String) -> Option<usize> {
    let (instruction, map) = parse(&content);
    find_count("AAA", instruction, &map, "ZZZ")
}

pub fn process_part2(content: String) -> Option<usize> {
    let (instruction, map) = parse(&content);
    let mut currents: Vec<&str> = Vec::new();
    map.keys()
        .filter(|e| e.ends_with('A'))
        .for_each(|e| currents.push(*e));
    let target = "Z";
    let counts: Vec<usize> = currents
        .iter()
        .map(|current| find_count(*current, instruction, &map, target))
        .flatten()
        .collect();
    Some(lcm_n(counts))
}

fn find_count(
    current: &str,
    instruction: &str,
    map: &HashMap<&str, (&str, &str)>,
    target: &str,
) -> Option<usize> {
    let mut current = current;
    let mut steps = instruction.chars().cycle().enumerate();
    let count = steps.try_for_each(|(count, step)| {
        let gate = map.get(current).unwrap();
        current = match step {
            'L' => gate.0,
            'R' => gate.1,
            _ => panic!("error step: {}", step),
        };
        if current.ends_with(target) {
            ControlFlow::Break(count)
        } else {
            ControlFlow::Continue(())
        }
    });
    if let ControlFlow::Break(count) = count {
        Some(count + 1)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 8;
    use super::*;
    use shared::{read_input, read_sample, read_sample2};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(6), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(21409), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample2(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(6), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(21165830176709), answer);
    }
}
