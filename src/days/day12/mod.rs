mod operation;
mod line;

use super::{day::Day, day_count::DayCount};
use line::Line;

pub const ARCHIVO_DIA_12: &str = "src/days/day12/file";

pub struct Day12 {
    springs: Vec<Line>,
}

impl Day for Day12 {
    fn resultado_parte_1(&self) -> u64 {
        self.springs.iter()
            .map(|line| line.get_possible_arrangements())
            .sum()
    }

    fn resultado_parte_2(&self) -> u64 {
        let springs = self.springs.iter()
            .map(|line| line.multiply(5))
            .collect::<Vec<Line>>();
        
        springs.iter()
        .map(|line| line.get_possible_arrangements())
        .sum()
    }

    fn day_count(&self) -> DayCount {
        DayCount::Day12
    }
}

impl Day12 {
    pub fn new(file: String) -> Self {
        let lines: Vec<Line> = file.lines()
            .filter_map(|line| Line::try_from(line).ok())
            .collect::<Vec<Line>>();

        Self { springs: lines }
    }
}

#[cfg(test)]
mod pruebas_dia_12 {
    use super::*;
    use operation::Operation;

    #[test]
    fn parseo_correctly() {
        let file = "???.### 1,1,3
        .??..??...?##. 1,1,3";

        let day12 = Day12::new(file.to_string());
        let expected_springs: Vec<Line> = vec![
            Line::new(
                vec![
                    Operation::Unknwon,
                    Operation::Unknwon,
                    Operation::Unknwon,
                    Operation::Operational,
                    Operation::Damage,
                    Operation::Damage,
                    Operation::Damage,
                ],
                vec![1, 1, 3]
            ),
            Line::new(
                vec![
                    Operation::Operational,
                    Operation::Unknwon,
                    Operation::Unknwon,
                    Operation::Operational,
                    Operation::Operational,
                    Operation::Unknwon,
                    Operation::Unknwon,
                    Operation::Operational,
                    Operation::Operational,
                    Operation::Operational,
                    Operation::Unknwon,
                    Operation::Damage,
                    Operation::Damage,
                    Operation::Operational,
                ],
                vec![1, 1, 3]
            ),
        ];

        assert_eq!(day12.springs, expected_springs);
    }

    #[test]
    fn ejemplo_parte_1() {
        let file = "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";

        let day12 = Day12::new(file.to_string());

        assert_eq!(day12.resultado_parte_1(), 21);
    }

    #[test]
    fn ejemplo_parte_2() {
        let file = "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";

        let day12 = Day12::new(file.to_string());

        assert_eq!(day12.resultado_parte_2(), 525152);
    }
}