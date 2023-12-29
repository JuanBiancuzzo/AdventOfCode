use std::fmt::{Display, Formatter, Result};
use std::convert::From;

pub const NUMBER_DAYS: usize = 25;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DayCount {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl Display for DayCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let number_of_day = usize::from(*self);
        write!(f, "Day {}", number_of_day.to_string())
    }
}

impl From<DayCount> for usize {
    fn from(value: DayCount) -> Self {
        match value {
            DayCount::Day1 => 1,
            DayCount::Day2 => 2,
            DayCount::Day3 => 3,
            DayCount::Day4 => 4,
            DayCount::Day5 => 5,
            DayCount::Day6 => 6,
            DayCount::Day7 => 7,
            DayCount::Day8 => 8,
            DayCount::Day9 => 9,
            DayCount::Day10 => 10,
            DayCount::Day11 => 11,
            DayCount::Day12 => 12,
            DayCount::Day13 => 13,
            DayCount::Day14 => 14,
            DayCount::Day15 => 15,
            DayCount::Day16 => 16,
            DayCount::Day17 => 17,
            DayCount::Day18 => 18,
            DayCount::Day19 => 19,
            DayCount::Day20 => 20,
            DayCount::Day21 => 21,
            DayCount::Day22 => 22,
            DayCount::Day23 => 23,
            DayCount::Day24 => 24,
            DayCount::Day25 => 25,
        }
    }
}