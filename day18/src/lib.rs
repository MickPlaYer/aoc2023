mod parser;

use parser::parse_map;
use parser::parse_steps;

enum Part {
    One,
    Two,
}

pub fn process_part1(input: String) -> Option<usize> {
    let mut map = parse_map(&input);
    let fill_start = (1, 1);
    let mut fill = vec![fill_start];
    while let Some(point) = fill.pop() {
        map.insert(point);
        let (x, y) = point;
        let points = vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];
        for point in points {
            let tile = map.get(&point);
            if tile.is_none() {
                fill.push(point);
            }
        }
    }
    Some(map.len())
}

// Shoelace formula
pub fn process_part2(input: String) -> Option<isize> {
    let steps = parse_steps(&input, Part::Two);
    let mut perimeter = 0;
    let mut current_point = (0, 0);
    let mut points = vec![current_point];
    for (dir, length) in steps {
        perimeter += length;
        match dir {
            "L" => current_point = (current_point.0 - length, current_point.1),
            "R" => current_point = (current_point.0 + length, current_point.1),
            "U" => current_point = (current_point.0, current_point.1 - length),
            "D" => current_point = (current_point.0, current_point.1 + length),
            _ => panic!("error input direction: {:?}", dir),
        }
        points.push(current_point);
    }
    let mut sum = 0;
    for pair in points.windows(2) {
        let point_a = pair[0];
        let point_b = pair[1];
        sum += (point_b.0 * point_a.1) - (point_a.0 * point_b.1)
    }
    Some(((sum.abs() + perimeter) / 2) + 1)
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 18;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(62), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(40761), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(952408144115), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(106920098354636), answer);
    }
}
