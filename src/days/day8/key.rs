use std::convert::TryFrom;

const NKEYVALUES: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key {
    pub key: [char; NKEYVALUES],
}

impl Key {
    pub fn new(key: [char; NKEYVALUES]) -> Key {
        Key { key }
    }

    pub fn is_final(&self) -> bool {
        self.key[2] == 'Z' || self.key[2] == 'z'
    }

    pub fn is_initial(&self) -> bool {
        self.key[2] == 'A' || self.key[2] == 'a'
    }
}

impl<'t> TryFrom<&'t str> for Key {
    type Error = ();

    fn try_from(value: &'t str) -> Result<Self, Self::Error> {
        let key: [char; NKEYVALUES] = match value.trim().chars().collect::<Vec<char>>()[..] {
            [a, b, c] => [a, b, c],
            _ => return Err(()),
        };

        Ok(Key::new(key))
    }
}