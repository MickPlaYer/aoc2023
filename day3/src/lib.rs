use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Grid {
    Empty,
    Number { value: usize, width: usize },
    Shadow(char, (usize, usize)),
    Symbol(char),
}

fn parse_mapping(
    content: String,
) -> (
    Vec<(usize, usize)>,
    Vec<(usize, usize)>,
    HashMap<(usize, usize), Grid>,
) {
    let mut symbol_cords = Vec::new();
    let mut number_cords = Vec::new();
    let mut map = HashMap::new();
    content.lines().enumerate().for_each(|(y, line)| {
        let mut last_number_cord: Option<(usize, usize)> = None;
        line.chars().enumerate().for_each(|(x, char)| {
            match char {
                '.' => {
                    last_number_cord = None;
                    map.insert((x, y), Grid::Empty);
                }
                '0'..='9' => match last_number_cord {
                    Some(cord) => {
                        let last_number = map.get_mut(&cord).unwrap();
                        match last_number {
                            Grid::Number { value, width } => {
                                *value *= 10;
                                *value += char as usize - '0' as usize;
                                *width += 1;
                                map.insert((x, y), Grid::Shadow(char, cord));
                            }
                            _ => panic!("last_number shound not be {:?}", last_number),
                        }
                    }
                    None => {
                        let grid = Grid::Number {
                            value: char as usize - '0' as usize,
                            width: 1,
                        };
                        number_cords.push((x, y));
                        map.insert((x, y), grid);
                        last_number_cord = Some((x, y));
                    }
                },
                _ => {
                    last_number_cord = None;
                    symbol_cords.push((x, y));
                    map.insert((x, y), Grid::Symbol(char));
                }
            };
        })
    });
    (symbol_cords, number_cords, map)
}

pub fn process_part1(content: String) -> Option<usize> {
    let (_, number_cords, map) = parse_mapping(content);
    let mut sum = 0;
    number_cords.iter().for_each(|cord| {
        let (x, y) = cord;
        let x = *x as isize;
        let y = *y as isize;
        let number = map.get(&cord).unwrap();
        match number {
            Grid::Number { value, width } => {
                let width = *width as isize;
                let x_range = (x - 1)..=(x - 1 + width + 1);
                x_range.into_iter().for_each(|x| {
                    let y_range = (y - 1)..=(y - 1 + 2);
                    y_range.into_iter().for_each(|y| {
                        if x < 0 || y < 0 {
                            return;
                        }
                        let grid = map.get(&(x as usize, y as usize));
                        if let Some(grid) = grid {
                            if let Grid::Symbol(c) = grid {
                                sum += value;
                                dbg!(value, c);
                            }
                        }
                    })
                })
            }
            _ => panic!("number shound not be {:?}", number),
        }
    });
    Some(sum)
}

pub fn process_part2(content: String) -> Option<usize> {
    let (symbol_cords, _, map) = parse_mapping(content);
    let mut gears = Vec::new();
    symbol_cords.iter().for_each(|cord| {
        let (x, y) = cord;
        let x = *x as isize;
        let y = *y as isize;
        let symbol = map.get(&cord).unwrap();
        let mut power = 1;
        let mut number_cords = HashSet::new();
        match symbol {
            Grid::Symbol(_) => {
                let x_range = (x - 1)..=(x - 1 + 2);
                x_range.into_iter().for_each(|x| {
                    let y_range = (y - 1)..=(y - 1 + 2);
                    y_range.into_iter().for_each(|y| {
                        if x < 0 || y < 0 {
                            return;
                        }
                        let cord = (x as usize, y as usize);
                        let grid = map.get(&cord);
                        if let Some(grid) = grid {
                            match grid {
                                Grid::Number { value, width: _ } => {
                                    if number_cords.insert(cord) {
                                        power *= value;
                                    }
                                }
                                Grid::Shadow(_, cord) => {
                                    let number = map.get(&cord).unwrap();
                                    if let Grid::Number { value, width: _ } = number {
                                        if number_cords.insert(*cord) {
                                            power *= value;
                                        }
                                    }
                                }
                                _ => (),
                            }
                        }
                    })
                })
            }
            _ => panic!("symbol shound not be {:?}", symbol),
        }
        if number_cords.len() == 2 {
            gears.push((power, cord, symbol));
        }
    });
    dbg!(&gears);
    Some(gears.iter().map(|(power, _, _)| power).sum())
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = 3;
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(4361), answer);
    }

    #[test]
    fn process_part1_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(539433), answer);
    }

    #[test]
    fn process_part2_with_sample() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(467835), answer);
    }

    #[test]
    fn process_part2_with_input() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(75847567), answer);
    }
}
