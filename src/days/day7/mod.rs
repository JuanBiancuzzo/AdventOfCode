

use super::day::Day;

pub const ARCHIVO_DIA_7: &str = "src/days/day7/file";

pub struct Day7 {
    file: String,
}

impl Day for Day7 {
    fn resultado(&self) -> (u64, u64) {
        (0, 0)
    }
}

impl Day7 {
    pub fn new(file: String) -> Day7 {
        Day7 { file }
    }
}
