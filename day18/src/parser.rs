use crate::Part;
use std::collections::HashSet;

pub(crate) fn parse_map(input: &str) -> HashSet<(isize, isize)> {
    let mut loop_map = HashSet::new();
    let mut current_point = (0, 0);
    loop_map.insert(current_point);
    let steps = parse_steps(input, Part::One);
    steps.into_iter().for_each(|(dir, length)| match dir {
        "L" => {
            for _ in 0..length {
                current_point = (current_point.0 - 1, current_point.1);
                loop_map.insert(current_point);
            }
        }
        "R" => {
            for _ in 0..length {
                current_point = (current_point.0 + 1, current_point.1);
                loop_map.insert(current_point);
            }
        }
        "U" => {
            for _ in 0..length {
                current_point = (current_point.0, current_point.1 - 1);
                loop_map.insert(current_point);
            }
        }
        "D" => {
            for _ in 0..length {
                current_point = (current_point.0, current_point.1 + 1);
                loop_map.insert(current_point);
            }
        }
        _ => panic!("error input direction: {:?}", dir),
    });
    loop_map
}

pub(crate) fn parse_steps(input: &str, part: Part) -> Vec<(&str, isize)> {
    let mut steps = Vec::new();
    input.lines().for_each(|line| {
        let dir;
        let length: isize;
        match part {
            Part::One => {
                let mut split = line.split(' ').into_iter();
                dir = split.next().unwrap();
                length = split.next().unwrap().parse().unwrap();
                let _color = split.next().unwrap();
            }
            Part::Two => {
                let mut split = line.split(' ').into_iter();
                let _dir = split.next().unwrap();
                let _length = split.next().unwrap();
                let color = split.next().unwrap();
                let dir_code = color.chars().nth(7).unwrap();
                dir = match dir_code {
                    '0' => "R",
                    '1' => "D",
                    '2' => "L",
                    '3' => "U",
                    _ => panic!("error dir_code: {:?}", dir_code),
                };
                let length_code = color.chars().skip(2).take(5).collect::<String>();
                length = isize::from_str_radix(&length_code, 16).unwrap();
            }
        }
        steps.push((dir, length));
    });
    steps
}
