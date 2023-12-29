mod race;

use super::day::Day;
use super::day_count::DayCount;
use race::Race;

pub const ARCHIVO_DIA_6: &str = "src/days/day6/file";

pub struct Day6 {
    file: String,
}

impl Day for Day6 {
    fn resultado_parte_1(&self) -> u64 {
        Self::get_possibilities_holding_time(Self::get_races(&self.file).unwrap())
    }

    fn resultado_parte_2(&self) -> u64 {
        Self::get_possibilities_holding_time(vec![Self::get_race(&self.file).unwrap()])
    }

    fn day_count(&self) -> DayCount {
        DayCount::Day6
    }
}

impl Day6 {
    pub fn new(file: String) -> Day6 {
        Day6 { file }
    }

    fn get_races(file: &String) -> Option<Vec<Race>> {
        let lines = file.lines().collect::<Vec<&str>>();
        let times = lines.get(0)?
            .split_once(':')?.1
            .split(' ')
            .filter_map(|number| number.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        let distances = lines.get(1)?
            .split_once(':')?.1
            .split(' ')
            .filter_map(|number| number.parse::<u64>().ok())
            .collect::<Vec<u64>>();

        if times.len() != distances.len() {
            return None;
        }

        let mut races: Vec<Race> = vec![];
        for i in 0..times.len() {
            let time = times.get(i)?;
            let distance = distances.get(i)?;

            races.push(Race::new(*time, *distance));
        }

        Some(races)
    }

    fn get_race(file: &String) -> Option<Race> {
        let lines = file.lines().collect::<Vec<&str>>();
        let time: u64 = lines.get(0)?
            .split_once(':')?.1
            .chars()
            .filter_map(|number| match number.to_digit(10) {
                Some(number) => Some(number as u64),
                None => None,
            })
            .rev()
            .enumerate()
            .map(|(i, number)| number * 10u64.pow(i as u32))
            .sum();
        let distance: u64 = lines.get(1)?
            .split_once(':')?.1
            .chars()
            .filter_map(|number| match number.to_digit(10) {
                Some(number) => Some(number as u64),
                None => None,
            })
            .rev()
            .enumerate()
            .map(|(i, number)| number * 10u64.pow(i as u32))
            .sum();

        Some(Race::new(time, distance))
    }

    fn get_possibilities_holding_time(races: Vec<Race>) -> u64 {
        races.iter()
            .map(|race| race.get_number_of_posible_holding_time())
            .product()
    }
}

#[cfg(test)]
mod pruebas_dia_6 {
    use super::*;

    #[test]
    fn ejemplo() {
        let file = "Time:      7  15   30
            Distance:  9  40  200";
        
        let resultado_parte_1 = Day6::new(file.to_string()).resultado_parte_1();
        let resultado_parte_2 = Day6::new(file.to_string()).resultado_parte_2();
        
        assert_eq!(resultado_parte_1, 288);
        assert_eq!(resultado_parte_2, 71503);
    }
}