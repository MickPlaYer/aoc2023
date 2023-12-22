mod parser;
mod structs;

use crate::parser::parse;
use crate::structs::Brick;
use std::collections::{HashMap, HashSet};

pub fn process_part1(input: String) -> Option<usize> {
    let mut bricks = parse(&input);
    bricks.sort_by_key(|e| e.get_bottom());
    let support_relation = find_support_relation(&bricks);
    let down_graph = get_down_graph(&support_relation);
    let critical_supports = get_critical_supports(&down_graph);
    Some(bricks.len() - critical_supports.len())
}

pub fn process_part2(input: String) -> Option<usize> {
    let mut bricks = parse(&input);
    bricks.sort_by_key(|e| e.get_bottom());
    let support_relation = find_support_relation(&bricks);
    let up_graph = get_up_graph(&support_relation);
    let down_graph = get_down_graph(&support_relation);
    let critical_supports = get_critical_supports(&down_graph);
    let mut sum = 0;
    for i in critical_supports {
        let mut falling = HashSet::new();
        falling.insert(i);
        falling_down(i, &mut falling, &down_graph, &up_graph);
        falling.remove(&i);
        sum += falling.len();
    }
    Some(sum)
}

fn get_up_graph(support_relation: &Vec<(usize, usize)>) -> HashMap<&usize, Vec<usize>> {
    let mut up_graph = HashMap::new();
    for (i, j) in support_relation {
        let list = up_graph.entry(j).or_insert(Vec::new());
        list.push(*i);
    }
    up_graph
}

fn get_down_graph(support_relation: &Vec<(usize, usize)>) -> HashMap<&usize, Vec<usize>> {
    let mut down_graph = HashMap::new();
    for (i, j) in support_relation {
        let list = down_graph.entry(i).or_insert(Vec::new());
        list.push(*j);
    }
    down_graph
}

fn get_critical_supports(down_graph: &HashMap<&usize, Vec<usize>>) -> HashSet<usize> {
    let mut critical_supports = HashSet::new();
    for (_, supports) in down_graph {
        if supports.len() == 1 {
            critical_supports.insert(supports[0]);
        }
    }
    critical_supports
}

fn find_support_relation(bricks: &Vec<Brick>) -> Vec<(usize, usize)> {
    let mut support_by = Vec::new();
    let mut top_bottom_layer = 0;
    let mut bottom_bricks: HashMap<usize, Vec<(usize, Brick)>> = HashMap::new();
    for (i, brick) in bricks.iter().enumerate() {
        let shadow = brick.get_shadow();
        let mut new_z = None;
        for layer in (1..=top_bottom_layer).rev() {
            if let Some(layer_bricks) = bottom_bricks.get(&layer) {
                for (j, layer_brick) in layer_bricks {
                    if layer_brick.is_cross(&shadow) {
                        new_z = Some(layer_brick.get_top() + 1);
                        support_by.push((i, *j));
                    }
                }
                if new_z.is_some() {
                    break;
                }
            }
        }
        let new_z = new_z.unwrap_or(1);
        let brick = brick.down_to(new_z);
        let new_top = brick.get_top();
        if new_top > top_bottom_layer {
            top_bottom_layer = new_top;
        }
        let layer = bottom_bricks.entry(new_top).or_insert(Vec::new());
        layer.push((i, brick));
    }
    support_by
}

fn falling_down(
    from: usize,
    falling: &mut HashSet<usize>,
    down_graph: &HashMap<&usize, Vec<usize>>,
    up_graph: &HashMap<&usize, Vec<usize>>,
) {
    let mut is_falling = falling.contains(&from);
    if !is_falling {
        if let Some(list) = down_graph.get(&from) {
            if list.iter().all(|e| falling.contains(e)) {
                falling.insert(from);
                is_falling = true;
            }
        }
    }
    if is_falling {
        if let Some(list) = up_graph.get(&from) {
            for i in list {
                falling_down(*i, falling, down_graph, up_graph);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 22;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample_1s() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(5), answer);
    }

    #[test]
    fn process_part1_with_input_1i() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(428), answer);
    }

    #[test]
    fn process_part2_with_sample_2s() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(7), answer);
    }

    #[test]
    fn process_part2_with_input_2i() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(35654), answer);
    }
}
