mod elements;
mod continuous_loop;
mod map;
mod direction;

use super::day::Day;

use elements::Element;
use continuous_loop::ContinuousLoop;
use map::Map;
use direction::Direction;

pub const ARCHIVO_DIA_10: &str = "src/days/day10/file";

pub struct Day10<const N: usize, const M: usize> {
    main_loop: ContinuousLoop,
}

impl<const N: usize, const M: usize> Day for Day10<N, M> {
    fn resultado_parte_1(&self) -> u64 {
        self.main_loop.get_farthest_distance_from_start().expect("El main loop debería tener la distancia más larga")
    }

    fn resultado_parte_2(&self) -> u64 {
        self.main_loop.get_inner_area()
    }
}

impl<const N: usize, const M: usize> Day10<N, M> {
    pub fn new(file: String) -> Self {
        let map = Map::<N, M, Element>::from(file);
        let mut starting_point: Option<(u32, u32)> = None;
        
        for i in 0..N {
            for j in 0..M {                
                if let Some(Element::StartingPoint) = map.get_element((i as i32, j as i32)) {
                    starting_point = Some((i as u32, j as u32));
                }
            }
        }        
        let starting_point = starting_point.expect("No se encontró el punto de inicio");

        Self { 
            main_loop: Self::get_main_loop(&map, &starting_point),
        }
    }

    fn get_main_loop(map: &Map<N, M, Element>, starting_point: &(u32, u32)) -> ContinuousLoop {
        let starting_point = (starting_point.0 as i32, starting_point.1 as i32);

        let starting_directions: Vec<Direction> = vec![
            Direction::Right,
            Direction::Up,
            Direction::Down,
            Direction::Left,
        ];

        let posible_loops = starting_directions.iter()
            .filter_map(|starting_direction| {
                let mut posible_loop = ContinuousLoop::new(starting_point);
                let mut current_position = starting_point;
                let mut next_direction = Some(*starting_direction);

                while let Some(direction) = next_direction {
                    let next_positon = direction.get_coordinate(current_position);
                    let next_element = match map.get_element(next_positon) {
                        Some(element) => element,
                        None => break,
                    };

                    if !posible_loop.add_element(next_positon) {
                        println!("No se pudo agregar el elemento {:?}", next_positon);
                        break;
                    }

                    next_direction = next_element.new_direction(direction);
                    current_position = next_positon;
                }   

                if posible_loop.is_complete_loop() {
                    Some(posible_loop)
                } else {
                    None
                }
            }).collect::<Vec<ContinuousLoop>>();
        
        posible_loops.first().expect("No se encontró un loop completo").clone()
    }   
}

#[cfg(test)]
mod pruebas_dia_10 {
    use super::*;

    #[test]
    fn ejemplo_1_parte_1() {
        let file = "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ";

        let day10 = Day10::<5, 5>::new(file.to_string());

        let resultado_parte_1 = day10.resultado_parte_1();

        assert_eq!(resultado_parte_1, 8);
    }

    #[test]
    fn ejemplo_2_parte_1() {
        let file = "-L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF";

        let day10 = Day10::<5, 5>::new(file.to_string());

        let resultado_parte_1 = day10.resultado_parte_1();

        assert_eq!(resultado_parte_1, 4);
    }

    #[test]
    fn ejemplo_1_parte_2() {
        let file = "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";

        let day10 = Day10::<11, 9>::new(file.to_string());

        let resultado_parte_2 = day10.resultado_parte_2();

        assert_eq!(resultado_parte_2, 4);
    }

    #[test]
    fn ejemplo_2_parte_2() {
        let file = ".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...";

        let day10 = Day10::<20, 10>::new(file.to_string());

        let resultado_parte_2 = day10.resultado_parte_2();

        assert_eq!(resultado_parte_2, 8);
    }

    #[test]
    fn ejemplo_3_parte_2() {
        let file = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";

        let day10 = Day10::<20, 10>::new(file.to_string());

        let resultado_parte_2 = day10.resultado_parte_2();

        assert_eq!(resultado_parte_2, 10);
    }
}