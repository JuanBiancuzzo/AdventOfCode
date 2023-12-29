use std::cmp::Ordering;
use super::direction::Direction;


#[derive(Debug, Clone, PartialEq)]
pub struct ContinuousLoop {
    pub elements: Vec<(i32, i32)>,
    clock_direction: Option<ClockDirection>,
}

#[derive(Debug, Clone, PartialEq)]
enum ClockDirection {
    ClockWise,
    AntiClockWise,
}

impl ContinuousLoop {
    pub fn new(starting_point: (i32, i32)) -> Self {
        Self { elements: vec![starting_point], clock_direction: None }
    }

    pub fn add_element(&mut self, element: (i32, i32)) -> bool {
        match self.elements.last() {
            Some(last_element) 
                if Self::taxicab_geometry(&element, last_element) == 1 => self.elements.push(element),
            None => self.elements.push(element),
            _ => return false,
        }

        if element.cmp(&self.elements[0]) == Ordering::Equal {
            self.get_clockdirection();
        }

        true
    }

    pub fn get_farthest_distance_from_start(&self) -> Option<u64> {
        if self.elements.is_empty() || !self.is_complete_loop() {
            return None;
        }

        Some((self.elements.len() as f64 / 2.0).floor() as u64)
    }

    pub fn get_inner_area(&self) -> u64 {        
        let mut posible_inner: Vec<(i32, i32)> = vec![];

        self.elements.windows(2).for_each(|window| if let &[primero, segundo] = window {
            if let Some(direction) = Direction::get_direction(primero, segundo) {
                let direction = match self.clock_direction.clone().expect("No debería poder ser que no tenga clock direction") {
                    ClockDirection::ClockWise => direction.move_clockwise(),
                    ClockDirection::AntiClockWise => direction.move_anticlockwise(),
                };

                let position = direction.get_coordinate(primero);
                if !posible_inner.contains(&position) {
                    posible_inner.push(position);
                }
            }
        });

        let mut inner: Vec<(i32, i32)> = vec![];

        while let Some((x, y)) = posible_inner.pop() {
            if self.elements.contains(&(x, y)) || inner.contains(&(x, y)) {
                continue;
            }

            inner.push((x, y));

            posible_inner.append(&mut vec![
                (x + 1, y),
                (x - 1, y),
                (x, y + 1),
                (x, y - 1),
            ]);
        }

        inner.len() as u64
    }

    
    fn get_clockdirection(&mut self) {
        let mut clockwise_count = 0;
        let mut anticlockwise_count = 0;

        for window in self.elements.windows(3) {
            if let &[first, second, third] = window {
                if let (Some(first_second_direction), Some(second_third_direction)) = (
                    Direction::get_direction(first, second), 
                    Direction::get_direction(second, third)
                ) {
                    if first_second_direction.move_clockwise() == second_third_direction {
                        clockwise_count += 1;
                    } else if first_second_direction.move_anticlockwise() == second_third_direction {
                        anticlockwise_count += 1;
                    }
                }
            }    
        }

        self.clock_direction = match clockwise_count.cmp(&anticlockwise_count) {
            Ordering::Greater => Some(ClockDirection::ClockWise),
            Ordering::Less => Some(ClockDirection::AntiClockWise),
            Ordering::Equal => None,
        };
    }

    pub fn is_complete_loop(&self) -> bool {
        if self.elements.len() <= 1 {
            return false;
        }

        match (self.elements.first(), self.elements.last()) {
            (Some(first), Some(last)) => Self::same_element(first, last),
            _ => false,
        }
    }

    fn taxicab_geometry(primero: &(i32, i32), segundo: &(i32, i32)) -> i32 {
        Self::abs_difference(primero.0, segundo.0) + Self::abs_difference(primero.1, segundo.1)
    }

    fn abs_difference(a: i32, b: i32) -> i32 {
        if a > b { a - b } else { b - a }
    }

    fn same_element(primero: &(i32, i32), segundo: &(i32, i32)) -> bool {
        primero.0 == segundo.0 && primero.1 == segundo.1
    }
}

#[cfg(test)]
mod pruebas_loop {
    use super::*;

    /*  0   1   2   3 -> x
     0  x-|-x-|-x-|-x 
     1  x | _ | _ | x
     2  x | _ | _ | x
     3  x-|-x-|-x-|-x
    
     */
    #[test]
    fn area_rectangulo() {
        let dx = 0;
        let dy = 0;

        let mut cloop = ContinuousLoop::new((0 + dx, 0 + dy));
        let positions: Vec<(i32, i32)> = vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 3),
            (2, 3),
            (3, 3),
            (3, 2),
            (3, 1),
            (3, 0),
            (2, 0),
            (1, 0),
            (0, 0),
        ];

        positions.iter().map(|(x, y)| (*x + dx, *y + dy)).for_each(|position| {
            cloop.add_element(position);
        });

        assert_eq!(cloop.get_inner_area(), 4);
    }

    /*  0   1   2   3   4   5-> x
     0  x-|-x | _ | _ | _ | _ 
     1  X | x-|-x-|-x-|-x-|-x
     2  x | x-|-x-|-X | _ | x
     3  x | x | _ | x-|-x-|-x
     4  x | x-|-X-|-X | _ | _
     5  x-|-x-|-X-|-X | _ | _

     */
    #[test]
    fn area_loop_irregular() {
        let dx = 84;
        let dy = 46;

        let mut cloop = ContinuousLoop::new((0 + dx, 0 + dy));
        let positions: Vec<(i32, i32)> = vec![
            (1, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (5, 2),
            (5, 3),
            (4, 3),
            (3, 3),
            (3, 2),
            (2, 2),
            (1, 2),
            (1, 3),
            (1, 4),
            (2, 4),
            (3, 4),
            (3, 5),
            (2, 5),
            (1, 5),
            (0, 5),
            (0, 4),
            (0, 3),
            (0, 2),
            (0, 1),
            (0, 0),
        ];

        positions.iter().map(|(x, y)| (*x + dx, *y + dy)).for_each(|position| {
            cloop.add_element(position);
        });

        assert_eq!(cloop.get_inner_area(), 1);
    }
}