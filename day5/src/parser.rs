use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

use crate::structs::{Almanac, Map};

fn digit1_as_int(input: &str) -> usize {
    input.parse().unwrap()
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, (_, _, seeds, _)) = tuple((
        tag("seeds:"),
        space1,
        separated_list1(space1, digit1),
        newline,
    ))(input)?;
    let seeds = seeds.into_iter().map(digit1_as_int).collect();
    Ok((input, seeds))
}

fn parse_map<'a>(tag_name: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, Map> {
    let numbers = map(
        tuple((digit1, space1, digit1, space1, digit1, newline)),
        |e| (digit1_as_int(e.0), digit1_as_int(e.2), digit1_as_int(e.4)),
    );
    map(
        tuple((tag(tag_name), space1, tag("map:"), newline, many1(numbers))),
        |(_, _, _, _, data)| Map::new(data),
    )
}

pub fn parse(input: &str) -> Almanac {
    let mut parse_all = map(
        tuple((
            parse_seeds,
            newline,
            parse_map("seed-to-soil"),
            newline,
            parse_map("soil-to-fertilizer"),
            newline,
            parse_map("fertilizer-to-water"),
            newline,
            parse_map("water-to-light"),
            newline,
            parse_map("light-to-temperature"),
            newline,
            parse_map("temperature-to-humidity"),
            newline,
            parse_map("humidity-to-location"),
        )),
        |e| (e.0, e.2, e.4, e.6, e.8, e.10, e.12, e.14),
    );
    let (
        _,
        (
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ),
    ) = parse_all(input).unwrap();
    Almanac::new(
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    )
}
