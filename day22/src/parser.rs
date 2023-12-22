use crate::structs::Brick;

pub fn parse(input: &str) -> Vec<Brick> {
    let mut bricks = Vec::new();
    for line in input.lines() {
        let mut split = line.split('~');
        let head = parse_numbers(&mut split);
        let tail = parse_numbers(&mut split);
        bricks.push(Brick::new(head.into(), tail.into()));
    }
    bricks
}

fn parse_numbers(split: &mut std::str::Split<'_, char>) -> Vec<usize> {
    let head = split
        .next()
        .unwrap()
        .split(',')
        .map(|e| e.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    head
}
