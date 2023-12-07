mod parser;
mod structs;

use parser::parse;

enum Part {
    One,
    Two,
}

pub fn process_part1(content: String) -> Option<usize> {
    let mut hands = parse(&content, Part::One);
    hands.sort();
    Some(
        hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| hand.get_bit() * (rank + 1))
            .sum(),
    )
}

pub fn process_part2(content: String) -> Option<usize> {
    let mut hands = parse(&content, Part::Two);
    hands.sort();
    Some(
        hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| hand.get_bit() * (rank + 1))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 7;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(6440), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(251927063), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(5905), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(255632664), answer);
    }
}
