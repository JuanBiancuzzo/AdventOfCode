use std::fmt::Display;
use super::day_count::DayCount;

pub trait Day {
    fn resultado_parte_1(&self) -> u64;

    fn resultado_parte_2(&self) -> u64;

    fn day_count(&self) -> DayCount;
}

impl Display for dyn Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let day_count = self.day_count();
        let part_1 = self.resultado_parte_1();
        let part_2 = self.resultado_parte_2();

        write!(f, "{day_count}:\n\tPart 1: {part_1}\n\tPart 2: {part_2}")
    }
}