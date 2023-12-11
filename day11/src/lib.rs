mod parser;
mod structs;

use parser::parse;

pub fn process_part1(content: String) -> Option<usize> {
    let image = parse(&content);
    let image = image.simulate_space_expands();
    let galaxies_pixels = image.find_galaxies_points();
    let mut result = 0;
    for i in 0..galaxies_pixels.len() {
        for j in i + 1..galaxies_pixels.len() {
            let (xa, ya) = galaxies_pixels[i];
            let (xb, yb) = galaxies_pixels[j];
            result += xb.abs_diff(xa) + yb.abs_diff(ya);
        }
    }
    Some(result)
}

pub fn process_part2(content: String, expand_speed: usize) -> Option<usize> {
    let mut image = parse(&content);
    image.set_expand_speed(expand_speed);
    let image = image.simulate_space_expands();
    let galaxies_pixels = image.find_galaxies_points();
    let mut result = 0;
    for i in 0..galaxies_pixels.len() {
        for j in i + 1..galaxies_pixels.len() {
            let point_a = galaxies_pixels[i];
            let point_b = galaxies_pixels[j];
            result += image.get_distance(point_a, point_b);
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 11;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(374), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(10154062), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content.clone(), 10);
        assert_eq!(Some(1030), answer);
        let answer = process_part2(content, 100);
        assert_eq!(Some(8410), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content, 1000000);
        assert_eq!(Some(553083047914), answer);
    }
}
