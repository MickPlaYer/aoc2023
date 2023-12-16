mod parser;
mod structs;

use parser::parse;

pub fn process_part1(input: String) -> Option<usize> {
    let map = parse(&input);
    Some(map.count_energized(map.generate_init_beam()))
}

pub fn process_part2(input: String) -> Option<usize> {
    let mut map = parse(&input);
    map.use_memo();
    let beams = map.generate_init_beams();
    Some(
        beams
            .into_iter()
            .map(|beam| map.count_energized(beam))
            .max()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 16;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(46), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(7046), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(51), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(7313), answer);
    }
}
