mod parser;
mod structs;

use crate::structs::Module;
use parser::parse;
use std::collections::{HashMap, VecDeque};

pub fn process_part1(input: String) -> Option<usize> {
    let mut high_count = 0;
    let mut low_count = 0;
    let mut modules = parse(&input);
    for _ in 0..1000 {
        let mut pulses = VecDeque::from(vec![(
            false,
            "broadcaster".to_string(),
            "buttom".to_string(),
        )]);
        while let Some((pulse, to, from)) = pulses.pop_front() {
            if pulse {
                high_count += 1;
            } else {
                low_count += 1;
            }
            if let Some(module) = modules.get_mut(&to) {
                for pulse in module.send(pulse, from) {
                    pulses.push_back(pulse);
                }
            }
        }
    }
    dbg!(&high_count, &low_count);
    Some(high_count * low_count)
}

pub fn process_part2(input: String) -> Option<usize> {
    let mut modules = parse(&input);
    // the graph split into four parts... idk, this is what I can do.
    let diffs = [
        idk(&mut modules, "sh".into(), "hf".into()),
        idk(&mut modules, "nm".into(), "pk".into()),
        idk(&mut modules, "ps".into(), "pm".into()),
        idk(&mut modules, "fs".into(), "mk".into()),
    ];
    Some(diffs.iter().product())
}

fn idk(modules: &mut HashMap<String, Module>, from: String, target: String) -> usize {
    let mut diffs = Vec::new();
    modules.insert(
        "broadcaster".to_string(),
        Module::broadcaster("broadcaster".to_string(), vec![from]),
    );
    let mut last_i = 0;
    for i in 0..10000 {
        let mut pulses = VecDeque::from(vec![(
            false,
            "broadcaster".to_string(),
            "buttom".to_string(),
        )]);
        while let Some((pulse, to, from)) = pulses.pop_front() {
            if pulse && target == from {
                if last_i != 0 {
                    let diff = i - last_i;
                    diffs.push(diff);
                }
                last_i = i;
            }
            if let Some(module) = modules.get_mut(&to) {
                for pulse in module.send(pulse, from) {
                    pulses.push_back(pulse);
                }
            }
        }
    }
    diffs.dedup();
    if diffs.len() == 1 {
        diffs.pop().unwrap()
    } else {
        panic!("IDK I CANT DO IT.")
    }
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 20;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(32000000), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(680278040), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(243548140870057), answer);
    }
}
