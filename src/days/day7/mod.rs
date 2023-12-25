mod card;
mod wild_card;
mod hand;
mod wild_hand;
mod type_hand;
mod hand_bid;

use super::day::Day;
use hand_bid::HandBid;
use hand::Hand;
use wild_hand::WildHand;

pub const ARCHIVO_DIA_7: &str = "src/days/day7/file";

pub struct Day7 {
    file: String,
}

impl Day for Day7 {
    fn resultado(&self) -> (u64, u64) {
        let mut hand_bids = Self::parse_hands::<Hand>(&self.file);
        let mut wild_hand_bids = Self::parse_hands::<WildHand>(&self.file);
        (
            Self::total_winning(&mut hand_bids), 
            Self::total_winning(&mut wild_hand_bids)
        )
    }
}

impl Day7 {
    pub fn new(file: String) -> Day7 {
        Day7 { file }
    }

    fn parse_hands<'t, H>(file: &'t String) -> Vec<HandBid<'t, H>>
        where H: TryFrom<&'t str> + PartialOrd,
        <H as TryFrom<&'t str>>::Error: std::fmt::Debug
    {
        file.lines()
            .map(|line| HandBid::try_from(line).unwrap())
            .collect::<Vec<HandBid<H>>>()
    }

    fn total_winning<'t, H>(hand_bids: &mut Vec<HandBid<'t, H>>) -> u64
        where H: TryFrom<&'t str> + PartialOrd + std::fmt::Debug + Clone,
        <H as TryFrom<&'t str>>::Error: std::fmt::Debug
    {
        hand_bids.sort_by(|a, b| a.partial_cmp(b).unwrap());

        hand_bids.iter()
            .enumerate()
            .map(|(index, hand_bids)| (index + 1) as u64 * hand_bids.bid as u64)
            .sum::<u64>() as u64
    }
}

#[cfg(test)]
mod pruebas_dia_7 {
    use super::*;

    #[test]
    fn ejemplo() {
        let file = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

        let resultado = Day7::new(file.to_string()).resultado();

        assert_eq!(resultado.0, 6440);
        assert_eq!(resultado.1, 5905);
    }
}