use std::convert::From;

use super::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    VerticalPipe,
    HorizontalPipe,
    BendPipeUpRight,
    BendPipeUpLeft,
    BendPipeDownLeft,
    BendPipeDownRight,
    Ground,
    StartingPoint,
}

impl Element {
    pub fn new_direction(&self, direction: Direction) -> Option<Direction> {
        Some(match self {
            Element::VerticalPipe => {
                match direction {
                    Direction::Up | Direction::Down => direction,
                    _ => return None,
                }
            },
            Element::HorizontalPipe => {
                match direction {
                    Direction::Left | Direction::Right => direction,
                    _ => return None,
                }
            },
            Element::BendPipeUpRight => {
                match direction {
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    _ => return None,
                }
            },
            Element::BendPipeUpLeft => {
                match direction {
                    Direction::Down => Direction::Left,
                    Direction::Right => Direction::Up,
                    _ => return None,
                }
            },
            Element::BendPipeDownLeft => {
                match direction {
                    Direction::Right => Direction::Down,
                    Direction::Up => Direction::Left,
                    _ => return None,
                }
            },
            Element::BendPipeDownRight => {
                match direction {
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Right,
                    _ => return None,
                }
            },
            Element::Ground | Element::StartingPoint => return None,
        })
    }
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            '|' => Element::VerticalPipe,
            '-' => Element::HorizontalPipe,
            'L' => Element::BendPipeUpRight,
            'J' => Element::BendPipeUpLeft,
            '7' => Element::BendPipeDownLeft,
            'F' => Element::BendPipeDownRight,
            '.' => Element::Ground,
            'S' => Element::StartingPoint,
            _ => panic!("Caracter no reconocido: {}", value),
        }
    }
}

impl Default for Element {
    fn default() -> Self {
        Element::Ground
    }
}