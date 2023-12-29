mod history;

use super::day::Day;
use super::day_count::DayCount;
use history::History;

pub const ARCHIVO_DIA_9: &str = "src/days/day9/file";

pub struct Day9 {
    histories: Vec<History>,
}

impl Day for Day9 {
    fn resultado_parte_1(&self) -> u64 {
        u64::try_from(self.histories.iter()
            .map(|history| history.predict_next_value())
            .sum::<i64>()
        ).unwrap()
    }

    fn resultado_parte_2(&self) -> u64 {
        u64::try_from(self.histories.iter()
            .map(|history| history.predict_previous_value())
            .sum::<i64>()
        ).unwrap()
    }

    fn day_count(&self) -> DayCount {
        DayCount::Day9
    }
}

impl Day9 {
    pub fn new(file: String) -> Day9 {
        let histories = file.lines()
            .map(History::from)
            .collect::<Vec<History>>();

        Day9 { histories }
    }
}

#[cfg(test)]
mod pruebas_dia_9 {
    use super::*;

    #[test]
    fn ejemplo() {
        let file = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        
        let dia_9 = Day9::new(file.to_string());
        let resultado_parte_1 = dia_9.resultado_parte_1();
        let resultado_parte_2 = dia_9.resultado_parte_2();

        assert_eq!(resultado_parte_1, 114);
        assert_eq!(resultado_parte_2, 2);
    }
}