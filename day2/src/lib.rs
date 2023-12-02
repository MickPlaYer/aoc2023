mod parser;
mod structs;

use crate::parser::parse;
use crate::structs::{CubeSet, Game};

pub fn process_part1(content: String) -> Option<usize> {
    let bag_cubes = CubeSet::new(12, 13, 14);
    let games = parse(&content);
    let possible_games: Vec<&Game> = games
        .iter()
        .filter(|game| {
            game.cube_sets.iter().all(|cube_set| {
                cube_set.red <= bag_cubes.red
                    && cube_set.green <= bag_cubes.green
                    && cube_set.blue <= bag_cubes.blue
            })
        })
        .collect();
    Some(possible_games.iter().map(|game| game.id).sum())
}

pub fn process_part2(content: String) -> Option<usize> {
    let games = parse(&content);
    let min_sets: Vec<CubeSet> = games
        .iter()
        .map(|game| {
            let mut min_set = CubeSet::empty();
            game.cube_sets.iter().for_each(|cube_set| {
                if cube_set.red > min_set.red {
                    min_set.red = cube_set.red;
                }
                if cube_set.green > min_set.green {
                    min_set.green = cube_set.green;
                }
                if cube_set.blue > min_set.blue {
                    min_set.blue = cube_set.blue;
                }
            });
            min_set
        })
        .collect();
    Some(
        min_sets
            .iter()
            .map(|cube_set| cube_set.red * cube_set.green * cube_set.blue)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 2;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(8), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(2369), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(2286), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(66363), answer);
    }
}
