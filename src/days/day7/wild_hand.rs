use std::collections::HashMap;
use std::cmp::{Ordering, PartialOrd};
use std::convert::TryFrom;

use super::{
    wild_card::WildCard,
    type_hand::TypeHand,
};

pub const NCARDAS: usize = 5;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord)]
pub struct WildHand {
    cards: [WildCard; NCARDAS],
}

impl WildHand {
    pub fn new(cards: [WildCard; NCARDAS]) -> Self {
        Self { cards }
    }

    pub fn same_cards(&self) -> Vec<u32> {
        let mut same_cards: HashMap<WildCard, u32> = HashMap::new();

        for card in self.cards.iter() {
            if same_cards.contains_key(card) {
                let count = same_cards.get_mut(card).unwrap();
                *count += 1;
            } else {
                same_cards.insert(*card, 1);
            }
        }

        if let Some(jack) = same_cards.clone().get(&WildCard::Jack) {
            let mut bigger_card: Option<WildCard> = None;
            let mut bigger_count: Option<u32> = None;

            for (card, count) in same_cards.iter() {
                if card == &WildCard::Jack {
                    continue;
                }
                match bigger_count {
                    Some(this_count) if this_count < *count => {
                        bigger_card = Some(*card);
                        bigger_count = Some(*count);
                    },
                    None => {
                        bigger_card = Some(*card);
                        bigger_count = Some(*count);
                    } 
                    _ => {},
                }
            }

            if let Some(bigger_card) = bigger_card {
                same_cards.entry(bigger_card).and_modify(|count| *count += jack);
                same_cards.remove(&WildCard::Jack);
            }

        }

        same_cards.values().map(|a| *a).collect::<Vec<u32>>()
    }
}

impl PartialOrd for WildHand {
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

impl TryFrom<&str> for WildHand {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.chars().collect::<Vec<char>>().as_slice() {
            [a, b, c, d, e] => {
                let a = WildCard::try_from(*a)?;
                let b = WildCard::try_from(*b)?;
                let c = WildCard::try_from(*c)?;
                let d = WildCard::try_from(*d)?;
                let e = WildCard::try_from(*e)?;

                Ok(WildHand::new([a, b, c, d, e]))
            },
            _ => Err(()),
        }
    }
}