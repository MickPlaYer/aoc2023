mod parser;
mod structs;

use crate::structs::{Analyzer, Factory};
use parser::parse;
use structs::Ranges;

pub fn process_part1(input: String) -> Option<usize> {
    let (workflows, parts) = parse(&input);
    let factory = Factory::new(workflows);
    let mut sum = 0;
    for part in parts {
        if factory.process(&part) {
            sum += part.get_score();
        }
    }
    Some(sum)
}

pub fn process_part2(input: String) -> Option<usize> {
    let (workflows, _) = parse(&input);
    let ranges = Ranges::new((1, 4000), (1, 4000), (1, 4000), (1, 4000));
    let analyzer = Analyzer::new(workflows);
    let accepts = analyzer.analyze(ranges);
    let mut sum = 0;
    for ranges in accepts {
        sum += ranges.get_score();
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 19;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(19114), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(446935), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(167409079868000), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(141882534122898), answer);
    }
}
