fn parse(content: &str) -> Vec<Vec<isize>> {
    content
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

fn find_histories_steps(histories: &Vec<Vec<isize>>) -> Vec<Vec<Vec<isize>>> {
    histories
        .iter()
        .map(|history| {
            let mut diff_steps = vec![history.clone()];
            while !diff_steps.last().unwrap().iter().all(|e| *e == 0) {
                let next_diff_step = find_diff_step(diff_steps.last().unwrap());
                diff_steps.push(next_diff_step);
            }
            diff_steps
        })
        .collect()
}

fn find_diff_step(input: &Vec<isize>) -> Vec<isize> {
    input.windows(2).map(|e| e[1] - e[0]).collect()
}

pub fn process_part1(content: String) -> Option<isize> {
    let histories = parse(&content);
    let histories_steps = find_histories_steps(&histories);
    let forward_predicts: Vec<isize> = histories_steps
        .iter()
        .map(|history_steps| history_steps.iter().map(|e| e.last().unwrap()).sum())
        .collect();
    Some(forward_predicts.iter().sum())
}

pub fn process_part2(content: String) -> Option<isize> {
    let histories = parse(&content);
    let histories_steps = find_histories_steps(&histories);
    let backward_predicts: Vec<isize> = histories_steps
        .iter()
        .map(|history_steps| {
            history_steps
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    let first = e.first().unwrap();
                    if i % 2 == 0 {
                        *first
                    } else {
                        -first
                    }
                })
                .sum()
        })
        .collect();
    Some(backward_predicts.iter().sum())
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 9;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(114), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(1806615041), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(2), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(1211), answer);
    }
}
