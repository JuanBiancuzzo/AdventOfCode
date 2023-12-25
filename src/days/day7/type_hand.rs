use std::convert::From;
use std::cmp::Ordering;

use super::hand::Hand;
use super::wild_hand::WildHand;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TypeHand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl From<Hand> for TypeHand {
    fn from(hand: Hand) -> Self {
        let mut same_cards: Vec<u32> = hand.same_cards();
        same_cards.sort_by(|a, b| b.cmp(a));
        let same_cards = same_cards.as_slice();

        match same_cards {
            [5] => TypeHand::FiveOfAKind,
            [4, 1] => TypeHand::FourOfAKind,
            [3, 2] => TypeHand::FullHouse,
            [3, 1, 1] => TypeHand::ThreeOfAKind,
            [2, 2, 1] => TypeHand::TwoPairs,
            [2, 1, 1, 1] => TypeHand::OnePair,
            [1, 1, 1, 1, 1] => TypeHand::HighCard,
            _ => panic!("No se puede convertir a TypeHand {:?}", hand)
        }
    }
}

impl From<WildHand> for TypeHand {
    fn from(hand: WildHand) -> Self {
        let mut same_cards: Vec<u32> = hand.same_cards();
        same_cards.sort_by(|a, b| b.cmp(a));
        let same_cards = same_cards.as_slice();

        match same_cards {
            [5] => TypeHand::FiveOfAKind,
            [4, 1] => TypeHand::FourOfAKind,
            [3, 2] => TypeHand::FullHouse,
            [3, 1, 1] => TypeHand::ThreeOfAKind,
            [2, 2, 1] => TypeHand::TwoPairs,
            [2, 1, 1, 1] => TypeHand::OnePair,
            [1, 1, 1, 1, 1] => TypeHand::HighCard,
            _ => panic!("No se puede convertir a TypeHand {:?} con {:?}", hand, same_cards)
        }
    }
}

impl From<TypeHand> for u32 {
    fn from(value: TypeHand) -> u32 {
        match value {
            TypeHand::FiveOfAKind => 8,
            TypeHand::FourOfAKind => 7,
            TypeHand::FullHouse => 6,
            TypeHand::ThreeOfAKind => 5,
            TypeHand::TwoPairs => 4,
            TypeHand::OnePair => 3,
            TypeHand::HighCard => 2,
        }
    }
}

impl PartialOrd for TypeHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_value: u32 = Self::into(*self);
        let other_value: u32 = Self::into(*other);

        self_value.partial_cmp(&other_value)
    }
}