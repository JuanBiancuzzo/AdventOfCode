
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Star {
    position: (usize, usize),
}

impl Star {
    pub fn new(x: usize, y: usize) -> Self {
        Self { position: (x, y) }
    }

    pub fn distance(&self, other: &Self) -> u64 {
        let (x1, y1) = self.position;
        let (x2, y2) = other.position;

        ((x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs()) as u64
    }
}