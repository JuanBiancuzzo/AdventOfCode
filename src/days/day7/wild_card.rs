use std::convert::{From, TryFrom};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, Hash)]
pub enum WildCard {
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

impl PartialOrd for WildCard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_value: u32 = Self::into(*self);
        let other_value: u32 = Self::into(*other);

        self_value.partial_cmp(&other_value)
    }
}

impl From<WildCard> for u32 {
    fn from(value: WildCard) -> u32 {
        match value {
            WildCard::Jack => 1,
            WildCard::Two => 2,
            WildCard::Three => 3,
            WildCard::Four => 4,
            WildCard::Five => 5,
            WildCard::Six => 6,
            WildCard::Seven => 7,
            WildCard::Eight => 8,
            WildCard::Nine => 9,
            WildCard::Ten => 10,
            WildCard::Queen => 12,
            WildCard::King => 13,
            WildCard::Ace => 14,
        }
    }
}

impl TryFrom<char> for WildCard {
    type Error = ();

    fn try_from(value: char) -> Result<WildCard, Self::Error> {
        match value {
            '2' => Ok(WildCard::Two),
            '3' => Ok(WildCard::Three),
            '4' => Ok(WildCard::Four),
            '5' => Ok(WildCard::Five),
            '6' => Ok(WildCard::Six),
            '7' => Ok(WildCard::Seven),
            '8' => Ok(WildCard::Eight),
            '9' => Ok(WildCard::Nine),
            'T' => Ok(WildCard::Ten),
            'J' => Ok(WildCard::Jack),
            'Q' => Ok(WildCard::Queen),
            'K' => Ok(WildCard::King),
            'A' => Ok(WildCard::Ace),
            _ => Err(()),
        }
    }
}