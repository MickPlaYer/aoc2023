mod parser;
mod solver;
mod structs;

use num_bigint::BigInt;
use parser::parse;

pub fn process_part1(input: String, test_range: (BigInt, BigInt)) -> Option<usize> {
    let hailstones = parse(&input);
    let mut count = 0;
    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            let (x, y) = hailstones[i].get_cross_xy(&hailstones[j]);
            if x.gte_by(&test_range.0)
                && x.lte_by(&test_range.1)
                && y.gte_by(&test_range.0)
                && y.lte_by(&test_range.1)
            {
                let will_through_a = hailstones[i].will_through((&x, &y));
                let will_through_b = hailstones[j].will_through((&x, &y));
                if will_through_a && will_through_b {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

pub fn process_part2(input: String) -> Option<isize> {
    let hailstones = parse(&input);
    Some(solver::solve(hailstones))
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 24;
    use super::*;
    use num_bigint::ToBigInt;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample_1s() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content, (7.to_bigint().unwrap(), 27.to_bigint().unwrap()));
        assert_eq!(Some(2), answer);
    }

    #[test]
    fn process_part1_with_input_1i() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(
            content,
            (
                (200000000000000 as isize).to_bigint().unwrap(),
                (400000000000000 as isize).to_bigint().unwrap(),
            ),
        );
        assert_eq!(Some(15318), answer);
    }

    #[test]
    fn process_part2_with_sample_2s() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(47), answer);
    }

    #[test]
    fn process_part2_with_input_2i() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(870379016024859), answer);
    }
}
