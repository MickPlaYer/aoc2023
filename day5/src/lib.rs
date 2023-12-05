mod parser;
mod structs;

use parser::parse;

pub fn process_part1(content: String) -> Option<usize> {
    let a = parse(&content);
    Some(*a.get_locations().iter().min().unwrap())
}

pub fn process_part2(content: String) -> Option<usize> {
    let a = parse(&content);
    Some(
        *a.get_location_ranges()
            .iter()
            .map(|r| r.start())
            .min()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 5;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(35), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(382895070), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(46), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(17729182), answer);
    }
}
