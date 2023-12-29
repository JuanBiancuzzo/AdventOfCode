use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_coordinate(&self, position: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (position.0, position.1 - 1),
            Direction::Down => (position.0, position.1 + 1),
            Direction::Left => (position.0 - 1, position.1),
            Direction::Right => (position.0 + 1, position.1),
        }
    }

    pub fn get_direction(primero: (i32, i32), segundo: (i32, i32)) -> Option<Self> {
        let dx = segundo.0 - primero.0;
        let dy = segundo.1 - primero.1;

        match (dx.cmp(&0), dy.cmp(&0)) {
            (Ordering::Equal, Ordering::Less) => Some(Direction::Up),
            (Ordering::Equal, Ordering::Greater) => Some(Direction::Down),
            (Ordering::Less, Ordering::Equal) => Some(Direction::Left),
            (Ordering::Greater, Ordering::Equal) => Some(Direction::Right),

            _ => None,
        }
    }

    pub fn move_clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn move_anticlockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}