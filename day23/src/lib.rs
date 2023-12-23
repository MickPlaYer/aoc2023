mod parser;
mod structs;

use parser::parse;
use std::collections::{HashMap, HashSet};

pub fn process_part1(input: String) -> Option<usize> {
    let map = parse(&input);
    let (end_id, paths, connections) = map.find_paths();
    let start_id = 0;
    let mut done = Vec::new();
    let mut todo = vec![(start_id, Vec::new(), 0)];
    while let Some((id, mut visited, count)) = todo.pop() {
        let next_count = count + paths.get(&id).unwrap();
        if id == end_id {
            let slop_count = visited.len() - 1;
            done.push((id, visited, next_count + slop_count));
            continue;
        }
        visited.push(id);
        for id in connections.get(&id).unwrap() {
            todo.push((*id, visited.clone(), next_count));
        }
    }
    Some(done.iter().map(|(_, _, e)| *e).max().unwrap())
}

pub fn process_part2(input: String) -> Option<usize> {
    let map = parse(&input);
    let (end_id, paths, connections) = map.find_paths();
    let connections = {
        let mut new_connections = HashMap::new();
        for (i, v) in connections {
            for j in v {
                let list = new_connections.entry(i).or_insert(HashSet::new());
                list.insert(j);
                let list = new_connections.entry(j).or_insert(HashSet::new());
                list.insert(i);
            }
        }
        new_connections
    };
    let start_id = 0;
    let mut done = Vec::new();
    let mut todo = vec![(start_id, Vec::new(), 0)];
    while let Some((id, mut visited, count)) = todo.pop() {
        let next_count = count + paths.get(&id).unwrap();
        if id == end_id {
            let slop_count = visited.len() - 1;
            done.push((id, visited, next_count + slop_count));
            continue;
        }
        visited.push(id);
        for id in connections.get(&id).unwrap() {
            if visited.contains(id) {
                continue;
            }
            todo.push((*id, visited.clone(), next_count));
        }
    }
    Some(done.iter().map(|(_, _, e)| *e).max().unwrap())
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 23;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample_1s() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(94), answer);
    }

    #[test]
    fn process_part1_with_input_1i() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(2230), answer);
    }

    #[test]
    fn process_part2_with_sample_2s() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(154), answer);
    }

    #[test]
    fn process_part2_with_input_2i() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(6542), answer);
    }
}
