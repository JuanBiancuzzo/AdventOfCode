use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RL {
    Left,
    Right,
}

impl TryFrom<char> for RL {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(RL::Left),
            'R' => Ok(RL::Right),
            _ => Err(()),
        }
    }
}