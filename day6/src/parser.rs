use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::structs::Record;

fn parse_time_line(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, (_, _, numbers)) =
        tuple((tag("Time:"), space1, separated_list1(space1, digit1)))(input)?;
    Ok((input, numbers))
}

fn parse_distance_line(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, (_, _, numbers)) =
        tuple((tag("Distance:"), space1, separated_list1(space1, digit1)))(input)?;
    Ok((input, numbers))
}

fn parse_lines(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (_, (times, _, distances)) =
        tuple((parse_time_line, newline, parse_distance_line))(input).unwrap();
    Ok((input, (times, distances)))
}

pub fn parse_part1(input: &str) -> Vec<Record> {
    let (_, (times, distances)) = parse_lines(input).unwrap();
    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Record::new(time.parse().unwrap(), distance.parse().unwrap()))
        .collect()
}

pub fn parse_part2(input: &str) -> Record {
    let (_, (times, distances)) = parse_lines(input).unwrap();
    let time = times.concat();
    let distance = distances.concat();
    Record::new(time.parse().unwrap(), distance.parse().unwrap())
}
