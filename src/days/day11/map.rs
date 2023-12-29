
pub struct Map<const N: usize, const M: usize> {
    stars: [[bool; N]; M],
}

impl<const N: usize, const M: usize> Map<N, M> {
    pub fn new() -> Self {
        Self { stars: [[false; N]; M] }
    }

    pub fn set(&mut self, x: usize, y: usize, state: bool) {
        self.stars[x][y] = state;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.stars[x][y]
    }
}

