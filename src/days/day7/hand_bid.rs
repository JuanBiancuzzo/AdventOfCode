use std::convert::TryFrom;
use std::cmp::{PartialOrd, Ordering};
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord)]
pub struct HandBid<'t, H> 
    where H: TryFrom<&'t str> + PartialOrd,
    <H as TryFrom<&'t str>>::Error: std::fmt::Debug
{
    pub hand: H,
    pub bid: u32,
    phantom: PhantomData<&'t H>,
}

impl<'t, H> HandBid<'t, H>
    where H: TryFrom<&'t str> + PartialOrd,
        <H as TryFrom<&'t str>>::Error: std::fmt::Debug
{
    pub fn new(hand: H, bid: u32) -> HandBid<'t, H> {
        HandBid { hand, bid, phantom: PhantomData }
    }
}

impl<'t, H> TryFrom<&'t str> for HandBid<'t, H>
    where H: TryFrom<&'t str> + PartialOrd,
        <H as TryFrom<&'t str>>::Error: std::fmt::Debug
{
    type Error = ();

    fn try_from(value: &'t str) -> Result<Self, Self::Error> {
        let hand = H::try_from(&value[..5]).unwrap();
        let bid = value[6..].parse::<u32>().unwrap();

        Ok(HandBid::new(hand, bid))
    }
}

impl<'t, H> PartialOrd for HandBid<'t, H>
    where H: TryFrom<&'t str> + PartialOrd,
    <H as TryFrom<&'t str>>::Error: std::fmt::Debug
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}