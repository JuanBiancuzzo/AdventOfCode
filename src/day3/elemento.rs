
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Elemento {
    pub valor: u32,
    pub inicio: (usize, usize),
}

impl Elemento {
    pub fn new(valor: u32, inicio: (usize, usize)) -> Self {
        Self { valor, inicio }
    }

    pub fn longitud(&self) -> u32 {
        self.valor.to_string().len() as u32
    }
}