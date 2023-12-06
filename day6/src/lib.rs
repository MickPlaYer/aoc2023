mod parser;
mod structs;

use parser::{parse_part1, parse_part2};

pub fn process_part1(content: String) -> Option<usize> {
    let records = parse_part1(&content);
    Some(records.into_iter().map(|r| r.count_successes()).product())
}

pub fn process_part2(content: String) -> Option<usize> {
    let record = parse_part2(&content);
    Some(record.count_successes())
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 6;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(288), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(771628), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(71503), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(27363861), answer);
    }
}
