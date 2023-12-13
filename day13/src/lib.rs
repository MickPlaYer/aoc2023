mod parser;
mod structs;

use parser::parse;

pub fn process_part1(input: String) -> Option<usize> {
    let maps = parse(&input);
    let scores: Vec<usize> = maps
        .iter()
        .map(|e| {
            let binding = e.find_horizontal_mirror();
            let h_score = binding.get(0).unwrap_or(&0);
            let e = e.transpose();
            let binding = e.find_horizontal_mirror();
            let v_score = binding.get(0).unwrap_or(&0);
            h_score * 100 + v_score
        })
        .collect();
    Some(scores.iter().sum())
}

pub fn process_part2(input: String) -> Option<usize> {
    let mut maps = parse(&input);
    let scores: Vec<usize> = maps
        .iter_mut()
        .map(|e| {
            let binding = e.find_horizontal_mirror_with_smudge();
            let h_score = binding.get(0).unwrap_or(&0);
            let mut e = e.transpose();
            let binding = e.find_horizontal_mirror_with_smudge();
            let v_score = binding.get(0).unwrap_or(&0);
            let total_score = h_score * 100 + v_score;
            if total_score == 0 {
                panic!("Some map did not find new lines!\n{:#?}", e);
            }
            total_score
        })
        .collect();
    Some(scores.iter().sum())
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 13;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(405), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(30487), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(400), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(31954), answer);
    }
}
