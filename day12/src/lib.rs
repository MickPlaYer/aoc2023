mod structs;

use structs::Record;

pub fn process_part1(input: String) -> Option<usize> {
    let records: Vec<Record> = input.lines().map(|line| Record::parse(line)).collect();
    let result = records
        .iter()
        .map(|record| record.count_arrangements())
        .sum();
    Some(result)
}

pub fn process_part2(input: String) -> Option<usize> {
    let records: Vec<Record> = input
        .lines()
        .map(|line| {
            dbg!(&line);
            let mut split = line.split(' ');
            let data = split.next().unwrap();
            let hint = split.next().unwrap();
            let data = [data].repeat(5).join("?");
            let hint = [hint].repeat(5).join(",");
            let line = [&data, " ", &hint].concat();
            Record::parse(&line)
        })
        .collect();
    let result = records
        .iter()
        .map(|record| record.count_arrangements())
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 12;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(21), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(7599), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(525152), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(15454556629917), answer);
    }
}
