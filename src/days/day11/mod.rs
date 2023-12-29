mod star;
mod map;

use super::{day::Day, day_count::DayCount};

use star::Star;
use map::Map;

pub const ARCHIVO_DIA_11: &str = "src/days/day11/file";

pub struct Day11<const N: usize, const M: usize> {
    file: String,
}

impl<const N: usize, const M: usize> Day for Day11<N, M> {
    fn resultado_parte_1(&self) -> u64 {
        let stars = self.get_stars(1);
        Self::get_shortest_distances(stars)
    }

    fn resultado_parte_2(&self) -> u64 {
        let stars = self.get_stars(1000000 - 1);
        Self::get_shortest_distances(stars)
    }

    fn day_count(&self) -> DayCount {
        DayCount::Day11
    }
}

/* N = x horizontal (aumenta a la derecha) , M = y vertical (aumenta hacia abajo) */
impl<const N: usize, const M: usize> Day11<N, M> {
    pub fn new(file: String) -> Self {
        Self { file }
    }

    fn get_stars(&self, space_in_between: usize) -> Vec<Star> {
        let mut space_y = vec![];
        let mut map = Map::<N, M>::new();
        for (j, line) in self.file.lines().enumerate() {
            if j >= M { break; }

            if line.trim().chars().all(|c| c == '.') {
                space_y.push(j);
                continue;
            }

            for (i, char) in line.trim().chars().enumerate() {
                if i >= N { break; }
                match char {
                    '#' => map.set(i, j, true),
                    '.' => {},
                    _ => panic!("Caracter invalido"),
                }
            }
        }
        
        let mut space_x = vec![];        
        for i in 0..N {
            let mut space = false;
            for j in 0..M {
                space |= map.get(i, j);
            }
            if !space {
                space_x.push(i);
            }
        }
        
        let mut stars = vec![];
        for j in 0..M {
            let mut j_space = 0;
            space_y.iter().for_each(|y| if *y < j { j_space += space_in_between;});
            
            for i in 0..N {
                let mut i_space = 0;
                space_x.iter().for_each(|x| if *x < i { i_space += space_in_between;});

                if map.get(i, j) {
                    stars.push(Star::new(i + i_space, j + j_space));
                }
            }
        }

        stars
    }

    fn get_shortest_distances(stars: Vec<Star>) -> u64 {
        let count_star = stars.len();

        let mut distance: u64 = 0;

        for i in 0..count_star {
            for j in (i + 1)..count_star {
                let star_i = stars[i];
                let star_j = stars[j];

                distance += star_i.distance(&star_j);
            }
        }

        distance
    }
}

#[cfg(test)]
mod pruebas_dia_11 {
    use super::*;

    #[test]
    fn espacio_de_estrellas() {
        let file = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        let day = Day11::<10, 10>::new(file.to_string());
        let stars = day.get_stars(1);
        
        assert_eq!(Star::new(4, 0), stars[0]);
        assert_eq!(Star::new(9, 1), stars[1]);
        assert_eq!(Star::new(0, 2), stars[2]);
        assert_eq!(Star::new(8, 5), stars[3]);
        assert_eq!(Star::new(1, 6), stars[4]);
        assert_eq!(Star::new(12, 7), stars[5]);
        assert_eq!(Star::new(9, 10), stars[6]);
        assert_eq!(Star::new(0, 11), stars[7]);
        assert_eq!(Star::new(5, 11), stars[8]);
    }

    #[test]
    fn ejemplo_1() {
        let file = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        let day = Day11::<10, 10>::new(file.to_string());
        assert_eq!(374, day.resultado_parte_1());
    }

    #[test]
    fn ejemplo_2() {
        let file = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        let day = Day11::<10, 10>::new(file.to_string());
        let stars = day.get_stars(9);

        assert_eq!(1030, Day11::<10, 10>::get_shortest_distances(stars));
    }

    #[test]
    fn ejemplo_3() {
        let file = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        let day = Day11::<10, 10>::new(file.to_string());
        let stars = day.get_stars(99);

        assert_eq!(8410, Day11::<10, 10>::get_shortest_distances(stars));
    }
}