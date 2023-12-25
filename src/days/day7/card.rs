use std::convert::{From, TryFrom};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, Hash)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten, // T
    Jack, // J
    Queen, // Q
    King, // K
    Ace, // A
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_value: u32 = Self::into(*self);
        let other_value: u32 = Self::into(*other);

        self_value.partial_cmp(&other_value)
    }
}

impl From<Card> for u32 {
    fn from(value: Card) -> u32 {
        match value {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Jack => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Card, Self::Error> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(()),
        }
    }
}