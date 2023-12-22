use std::convert::TryFrom;

pub struct Map {
    destination_start: u32,
    source_start: u32,
    range: u32,
}

impl Map {
    pub fn new(destination_start: u32, source_start: u32, range: u32) -> Self {
        Self {
            destination_start,
            source_start,
            range,
        }
    }

    pub fn transform(&self, source: u32) -> Option<u32> {
        if self.source_start > source || source > self.source_start + self.range {
            return None;
        }
        Some(self.destination_start + (source - self.source_start))
    }
}

impl TryFrom<&str> for Map {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let numeros = value.split(" ")
            .filter_map(|numero| numero.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let destination_start = match numeros.get(0) {
            Some(numero) => *numero,
            None => return Err(()),
        };

        let source_start = match numeros.get(1) {
            Some(numero) => *numero,
            None => return Err(()),
        };

        let range = match numeros.get(2) {
            Some(numero) => *numero,
            None => return Err(()),
        };

        Ok(Self::new(destination_start, source_start, range))
    }
}