use super::{elemento::Elemento, Day3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gear {
    pub posicion: (u32, u32),
}

impl Gear {
    pub fn new(posicion: (u32, u32)) -> Self {
        Self { posicion }
    }

    pub fn gear_ratio<const N: usize, const M: usize>(&self, mapa: &[[char; N]; M]) -> Option<u32> {
        let elementos: Vec<Elemento> = self.obtener_elemento_rodeando(mapa)?;

        let elementos_adyacentes = elementos.iter()
            .filter(|elemento| {
                elemento.posiciones_adyacentes().iter()
                    .any(|(i, j)| *i == self.posicion.0 as i32 && *j == self.posicion.1 as i32)
            }).collect::<Vec<&Elemento>>();
        
        if elementos_adyacentes.len() == 2 {
            return Some(elementos_adyacentes.iter()
                .map(|elemento| elemento.valor)
                .product::<u32>());
        }

        None
    }

    fn obtener_elemento_rodeando<const N: usize, const M: usize>(&self, mapa: &[[char; N]; M]) -> Option<Vec<Elemento>> {
        let mut elementos: Vec<Elemento> = vec![];
        
        let i: i32 = self.posicion.0.try_into().ok()?;
        let mut nuevo_mapa: [[char; N]; 1] = [['.'; N]; 1];
        for j in i - 1 .. i + 2 {
            if let Ok(j) = usize::try_from(j) {
                if let Some(mapa_reducido) = mapa.get(j) {
                    nuevo_mapa[0] = *mapa_reducido;
                    let mut elementos_nuevos = Day3::<N, 1>::obtener_elementos(&nuevo_mapa)
                        .iter_mut()
                        .map(|elemento| {elemento.inicio.0 = j; *elemento})
                        .collect::<Vec<Elemento>>();
                    elementos.append(&mut elementos_nuevos);
                }
            }
        }

        Some(elementos)
    }
}