use crate::structs::Hailstone;
use num_bigint::BigInt;

pub(crate) fn parse(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split('@');
            let position = split
                .next()
                .unwrap()
                .split(',')
                .map(|e| e.trim().parse::<BigInt>().unwrap())
                .collect::<Vec<_>>();
            let velocity = split
                .next()
                .unwrap()
                .split(',')
                .map(|e| e.trim().parse::<BigInt>().unwrap())
                .collect::<Vec<_>>();
            Hailstone::new(position, velocity)
        })
        .collect()
}
