use itertools::Itertools;
use std::cmp::Reverse;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
enum C {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    N9 = 9,
    N8 = 8,
    N7 = 7,
    N6 = 6,
    N5 = 5,
    N4 = 4,
    N3 = 3,
    N2 = 2,
    Joker = 1,
}

impl C {
    fn new(c: char) -> C {
        match c {
            'A' => C::A,
            'K' => C::K,
            'Q' => C::Q,
            'J' => C::J,
            'T' => C::T,
            '9' => C::N9,
            '8' => C::N8,
            '7' => C::N7,
            '6' => C::N6,
            '5' => C::N5,
            '4' => C::N4,
            '3' => C::N3,
            '2' => C::N2,
            'j' => C::Joker,
            _ => panic!("illegal card: {:?}", c),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum HandType {
    FiveKing = 7,
    FourKing = 6,
    FullHouse = 5,
    ThreeKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Ord, Eq, PartialEq)]
pub struct Hand {
    bit: usize,
    cards: [C; 5],
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.get_type().partial_cmp(&other.get_type()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.cards.partial_cmp(&other.cards)
    }
}

impl Hand {
    pub fn new(bit: usize, chars: [char; 5]) -> Self {
        Self {
            bit,
            cards: [
                C::new(chars[0]),
                C::new(chars[1]),
                C::new(chars[2]),
                C::new(chars[3]),
                C::new(chars[4]),
            ],
        }
    }

    pub fn get_bit(&self) -> usize {
        self.bit
    }

    fn get_type(&self) -> HandType {
        let mut joker_count = 0;
        let mut amounts = Vec::new();
        let mut cards = Vec::new();
        cards.extend_from_slice(&self.cards);
        cards.sort();
        cards
            .into_iter()
            .group_by(|c| *c)
            .into_iter()
            .for_each(|(c, group)| match c {
                C::Joker => joker_count = group.count(),
                _ => amounts.push(group.count()),
            });
        amounts.sort_by_key(|n| Reverse(*n));
        let first_amount = amounts.get(0);
        if first_amount.is_some() {
            amounts[0] += joker_count;
        } else {
            amounts.push(joker_count);
        }
        self.match_hand_type(amounts)
    }

    fn match_hand_type(&self, amounts: Vec<usize>) -> HandType {
        match amounts[0] {
            5 => HandType::FiveKing,
            4 => HandType::FourKing,
            3 => match amounts[1] {
                2 => HandType::FullHouse,
                1 => HandType::ThreeKind,
                _ => panic!("illegal hand: {:?} {:?}", self.cards, amounts),
            },
            2 => match amounts[1] {
                2 => HandType::TwoPair,
                1 => HandType::OnePair,
                _ => panic!("illegal hand: {:?} {:?}", self.cards, amounts),
            },
            1 => HandType::HighCard,
            _ => panic!("illegal hand: {:?} {:?}", self.cards, amounts),
        }
    }
}
