use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Operational,
    Damage,
    Unknwon,
}

impl TryFrom<char> for Operation {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Operation::Operational,
            '#' => Operation::Damage,
            '?' => Operation::Unknwon,
            _ => return Err(()),
        })
    }
}