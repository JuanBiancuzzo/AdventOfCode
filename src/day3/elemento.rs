
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Elemento {
    pub valor: u32,
    pub inicio: (usize, usize),
}

impl Elemento {
    pub fn new(valor: u32, inicio: (usize, usize)) -> Self {
        Self { valor, inicio }
    }

    fn longitud(&self) -> u32 {
        self.valor.to_string().len() as u32
    }

    pub fn elemento_valido<const N: usize, const M: usize>(&self, mapa: &[[char; N]; M]) -> Option<u32> {
        if self.posiciones_adyacentes().iter()
            .filter_map(|(i, j)| {
                let i = usize::try_from(*i).ok()?;
                let j = usize::try_from(*j).ok()?;

                mapa.get(i)?.get(j)
            })
            .any(|caracter| *caracter != '.') {
            return Some(self.valor);
        }

        None
    }

    pub fn posiciones_adyacentes(&self) -> Vec<(i32, i32)> {
        let posicion = (
            self.inicio.0 as i32,
            self.inicio.1 as i32
        );
        let longitud = self.longitud() as i32;

        let mut posiciones: Vec<(i32, i32)> = vec![
            (posicion.0, posicion.1 - 1),
            (posicion.0, posicion.1 + longitud),
        ];

        for j in (posicion.1 - 1)..(posicion.1 + longitud + 1) {
            posiciones.push((posicion.0 - 1, j));
            posiciones.push((posicion.0 + 1, j));
        }

        posiciones
    }
}