#[derive(Debug)]
struct Len<'a> {
    label: &'a str,
    value: usize,
}

fn hash_it(input: &str) -> usize {
    let mut current_value = 0;
    input.chars().for_each(|c| {
        let value = c as usize;
        current_value += value;
        current_value *= 17;
        current_value %= 256;
    });
    current_value
}

pub fn process_part1(input: String) -> Option<usize> {
    let line = input.lines().nth(0).unwrap();
    let sum = line.split(',').map(|e| hash_it(e)).sum();
    Some(sum)
}

pub fn process_part2(input: String) -> Option<usize> {
    let mut boxes: Vec<Vec<Len>> = (0..256).map(|_| Vec::new()).collect();
    let line = input.lines().nth(0).unwrap();
    line.split(',').for_each(|operation| {
        let action;
        let mut split;
        if operation.contains('=') {
            action = '=';
            split = operation.split('=');
        } else if operation.contains('-') {
            action = '-';
            split = operation.split('-');
        } else {
            panic!("operation error: {:?}", operation);
        }
        let label = split.next().unwrap();
        let hash = hash_it(label);
        let current_box = &mut boxes[hash];
        let position = current_box.iter().position(|e| e.label == label);
        if action == '=' {
            let value = split.next().unwrap().parse().unwrap();
            let len = Len { label, value };
            if let Some(position) = position {
                current_box[position] = len;
            } else {
                current_box.push(len);
            }
        } else if action == '-' {
            if let Some(position) = position {
                current_box.remove(position);
            }
        }
    });
    let total = boxes
        .iter()
        .enumerate()
        .map(|(i, a_box)| {
            let box_position = i + 1;
            a_box
                .iter()
                .enumerate()
                .map(|(j, len)| {
                    let len_slot = j + 1;
                    box_position * len_slot * len.value
                })
                .sum::<usize>()
        })
        .sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 15;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(1320), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(509167), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(145), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(259333), answer);
    }
}
