use regex::Regex;

pub fn process_part1(content: String) -> Option<usize> {
    let collected_numbers: Vec<usize> = content
        .lines()
        .map(|line| {
            let numbers: Vec<char> = line
                .chars()
                .filter(|char| match char {
                    '0'..='9' => true,
                    _ => false,
                })
                .collect();
            [*numbers.first().unwrap(), *numbers.last().unwrap()]
                .iter()
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect();
    Some(collected_numbers.iter().sum())
}

pub fn process_part2(content: String) -> Option<usize> {
    let pattern_front = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let pattern_back = Regex::new(r"(enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|\d)").unwrap();
    let collected_numbers: Vec<usize> = content
        .lines()
        .map(|line| {
            let line_backward: String = line.chars().rev().collect();
            let front = &pattern_front.captures(line).unwrap()[0];
            let back = &pattern_back.captures(&line_backward).unwrap()[0];
            let front = match front {
                "one" => '1',
                "two" => '2',
                "three" => '3',
                "four" => '4',
                "five" => '5',
                "six" => '6',
                "seven" => '7',
                "eight" => '8',
                "nine" => '9',
                _ => front.chars().nth(0).unwrap(),
            };
            let back = match back {
                "eno" => '1',
                "owt" => '2',
                "eerht" => '3',
                "ruof" => '4',
                "evif" => '5',
                "xis" => '6',
                "neves" => '7',
                "thgie" => '8',
                "enin" => '9',
                _ => back.chars().nth(0).unwrap(),
            };
            dbg!(line, front, back);
            [front, back].iter().collect::<String>().parse().unwrap()
        })
        .collect();
    Some(collected_numbers.iter().sum())
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 1;
    use super::*;
    use shared::{read_input, read_sample, read_sample2};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(142), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(54081), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample2(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(281), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(54649), answer);
    }
}
