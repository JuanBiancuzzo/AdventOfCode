use std::collections::HashMap;
use std::cmp::{Ordering, PartialOrd};
use std::convert::TryFrom;

use super::{
    card::Card,
    type_hand::TypeHand,
};

pub const NCARDAS: usize = 5;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord)]
pub struct Hand {
    cards: [Card; NCARDAS],
}

impl Hand {
    pub fn new(cards: [Card; NCARDAS]) -> Hand {
        Hand { cards }
    }

    pub fn same_cards(&self) -> Vec<u32> {
        let mut same_cards: HashMap<Card, u32> = HashMap::new();

        for card in self.cards.iter() {
            if same_cards.contains_key(card) {
                let count = same_cards.get_mut(card).unwrap();
                *count += 1;
            } else {
                same_cards.insert(*card, 1);
            }
        }

        same_cards.values().map(|a| *a).collect::<Vec<u32>>()
    }
}


impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let hand_type = TypeHand::from(*self);
        let other_hand_type = TypeHand::from(*other);

        let mut comparacion = hand_type.partial_cmp(&other_hand_type);
        let mut i = 0;

        while Some(Ordering::Equal) == comparacion {
            if i >= NCARDAS {
                break;
            }

            comparacion = self.cards[i].partial_cmp(&other.cards[i]);
            i += 1;
        }
        
        comparacion
    }
}

impl TryFrom<&str> for Hand {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.chars().collect::<Vec<char>>().as_slice() {
            [a, b, c, d, e] => {
                let a = Card::try_from(*a)?;
                let b = Card::try_from(*b)?;
                let c = Card::try_from(*c)?;
                let d = Card::try_from(*d)?;
                let e = Card::try_from(*e)?;

                Ok(Hand::new([a, b, c, d, e]))
            },
            _ => Err(()),
        }
    }
}