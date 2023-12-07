use crate::{structs::Hand, Part};

pub fn parse(input: &str, part: Part) -> Vec<Hand> {
    let mut hands = Vec::new();
    input.lines().for_each(|line| {
        let mut split = line.split(" ");
        let mut cards = split.next().unwrap().to_string();
        if let Part::Two = part {
            cards = cards.replace("J", "j");
        }
        let cards = cards.chars().collect::<Vec<char>>();
        let cards: [char; 5] = cards[0..5].try_into().unwrap();
        let bit = split.next().unwrap().parse().unwrap();
        let hand = Hand::new(bit, cards);
        hands.push(hand);
    });
    hands
}
