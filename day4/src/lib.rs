mod parser;
mod structs;

use parser::parse;
use std::collections::HashMap;

pub fn process_part1(content: String) -> Option<usize> {
    let cards = parse(&content);
    let total_points = cards.iter().map(|card| card.get_points()).sum();
    Some(total_points)
}

pub fn process_part2(content: String) -> Option<usize> {
    let mut count_mapping = HashMap::new();
    let cards = parse(&content);
    cards.iter().for_each(|card| {
        let id = card.get_id();
        let current_amount = count_mapping.get(&id).or(Some(&0)).unwrap() + 1;
        count_mapping.insert(id, current_amount);
        let match_count = card.get_match_count();
        if match_count > 0 {
            let new_card_ids = (id + 1)..=(id + match_count);
            new_card_ids.for_each(|new_id| {
                let old_number = count_mapping.get(&new_id).or(Some(&0)).unwrap();
                let new_number = old_number + current_amount;
                count_mapping.insert(new_id, new_number);
            });
        }
    });
    let total_card_amount = count_mapping.values().sum();
    Some(total_card_amount)
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 4;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(13), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(26346), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(30), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(8467762), answer);
    }
}
