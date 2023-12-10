use crate::ejercicio::Ejercicio;

pub struct Day2 {
    file: String,
}

impl Ejercicio for Day2 {
    fn resultado(&self) -> (u32, u32) {
        (
            0,
            0
        )
    }

}

impl Day2 {
    pub fn new(file: String) -> Self {
        Self { file }
    }
}