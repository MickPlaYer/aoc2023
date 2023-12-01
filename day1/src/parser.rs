use nom::{
    branch::alt,
    complete::tag,
    complete::take,
    multi::{count, many1},
    IResult,
};

fn parse_digit(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, digits) = many1(alt((
        tag("one"),
        tag("two"),
        tag("three"),
        tag("four"),
        tag("five"),
        tag("six"),
        tag("seven"),
        tag("eight"),
        tag("nine"),
        tag("1"),
        tag("2"),
        tag("3"),
        tag("4"),
        tag("5"),
        tag("6"),
        tag("7"),
        tag("8"),
        tag("9"),
        count(take(1), 1),
    )))(input)?;
    dbg!(&digits);
    // todo!()
    Ok((input, digits))
}

pub fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            let (_, result) = parse_digit(line).unwrap();
            let result: Vec<char> = result
                .iter()
                .map(|n| match *n {
                    "one" => '1',
                    "two" => '2',
                    "three" => '3',
                    "four" => '4',
                    "five" => '5',
                    "six" => '6',
                    "seven" => '7',
                    "eight" => '8',
                    "nine" => '9',
                    _ => n.chars().nth(0).unwrap(),
                })
                .collect();
            [*result.first().unwrap(), *result.last().unwrap()]
                .iter()
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_digit() {
        let (_, result) = parse_digit("eightthree3ninekzhtlqsevenssprmrqhhgncrs").unwrap();
        assert_eq!(vec!["eight", "three", "3", "nine"], result);

        // let (_, result) = parse_digit("qhklfjd39rpjxhqtftwopfvrrj2eight").unwrap();
        // assert_eq!(vec!["eight", "three", "3", "nine"], result);
    }
}
