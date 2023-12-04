use crate::structs::Card;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, numbers) = separated_list1(space1, digit1)(input)?;
    let numbers: Vec<usize> = numbers.iter().map(|n| n.parse().unwrap()).collect();
    Ok((input, numbers))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, (_, id, _, winning_numbers, _, numbers)) = tuple((
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
        parse_numbers,
        tuple((space1, tag("|"), space1)),
        parse_numbers,
    ))(input)?;
    let id = id.parse().unwrap();
    let card = Card::new(id, winning_numbers, numbers);
    Ok((input, card))
}

pub fn parse(input: &str) -> Vec<Card> {
    let (_, cards) = separated_list1(newline, parse_card)(input).unwrap();
    cards
}
