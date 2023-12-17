mod parser;
mod structs;

use parser::parse;

pub fn process_part1(input: String) -> Option<usize> {
    let map = parse(&input);
    Some(map.get_minimize_heat_loss(false))
}

pub fn process_part2(input: String) -> Option<usize> {
    let map = parse(&input);
    Some(map.get_minimize_heat_loss(true))
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 17;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(102), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(684), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(94), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(822), answer);
    }
}
