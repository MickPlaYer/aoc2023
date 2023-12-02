use crate::structs::{CubeSet, Game};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn parse_cube_set(input: &str) -> IResult<&str, CubeSet> {
    let colors = alt((tag("red"), tag("green"), tag("blue")));
    let cubes = map(
        tuple((space1, digit1, space1, colors)),
        |(_, number, _, color)| (number, color),
    );
    let (input, set) = separated_list1(tag(","), cubes)(input)?;
    let mut cube_set = CubeSet::empty();
    set.iter().for_each(|(number, color)| {
        let number = number.parse().unwrap();
        match *color {
            "red" => cube_set.red = number,
            "green" => cube_set.green = number,
            "blue" => cube_set.blue = number,
            _ => panic!("unknow color: {}", color),
        }
    });
    Ok((input, cube_set))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, number) = map(tuple((tag("Game "), digit1, tag(":"))), |(_, number, _)| {
        number
    })(input)?;
    let (input, cube_sets) = separated_list1(tag(";"), parse_cube_set)(input)?;
    let game = Game::new(number.parse().unwrap(), cube_sets);
    Ok((input, game))
}

pub fn parse(input: &str) -> Vec<Game> {
    let (_, cube_sets) = separated_list1(newline, parse_game)(input).unwrap();
    cube_sets
}
