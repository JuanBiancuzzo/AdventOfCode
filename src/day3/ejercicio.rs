use crate::ejercicio::Ejercicio;
use super::elemento::Elemento;

pub const ARCHIVO_DIA_3: &str = "src/day3/file";

pub struct Day3<const N: usize, const M: usize> {
    file: String,
}

impl<const N: usize, const M: usize> Ejercicio for Day3<N, M> {
    fn resultado(&self) -> (u32, u32) {
        let mapa = Self::crear_mapa(&self.file);
        (
            Self::calcular_valores(&mapa), 
            Self::calcular_valores(&mapa)
        )
    }
}

impl<const N: usize, const M: usize> Day3<N, M> {
    pub fn new(file: String) -> Self {
        Self { file }
    }

    fn crear_mapa(text: &str) -> [[char; N]; M] {
        let mut mapa = [['.'; N]; M];

        for (i, linea) in text.lines().enumerate() {
            if i >= M { break; }
            for (j, caracter) in linea.trim().chars().enumerate() {
                if j >= N { break; }
                mapa[i][j] = caracter;
            }
        }

        mapa
    }

    fn calcular_valores(mapa: &[[char; N]; M]) -> u32 {
        Self::obtener_elementos(mapa).iter()
            .filter_map(|elemento| Self::elemento_valido(elemento, mapa))
            .sum::<u32>()
    }

    fn obtener_elementos(mapa: &[[char; N]; M]) -> Vec<Elemento> {
        let mut elementos: Vec<Elemento> = vec![];

        for (i, linea) in mapa.iter().enumerate() {
            let mut j = 0;
            let mut linea_iterable = linea.iter();

            while let Some(caracter) = linea_iterable.next() {
                let mut valor = match caracter.to_digit(10) {
                    Some(valor) => valor,
                    None => {
                        j += 1;
                        continue
                    }
                };
                let inicio = j;
                while let Some(caracter) = linea_iterable.next() {
                    j += 1;
                    match caracter.to_digit(10) {
                        Some(siguiente_valor) => valor = 10 * valor + siguiente_valor,
                        None => break,
                    };
                }
                elementos.push(Elemento::new(valor, (i, inicio)));
                j += 1;
            }
        }

        elementos
    }

    fn elemento_valido(elemento: &Elemento, mapa: &[[char; N]; M]) -> Option<u32> {
        let posicion = (
            elemento.inicio.0 as i32,
            elemento.inicio.1 as i32
        );
        let longitud = elemento.longitud() as i32;

        let mut posiciones: Vec<(i32, i32)> = vec![
            (posicion.0, posicion.1 - 1),
            (posicion.0, posicion.1 + longitud),
        ];

        for j in (posicion.1 - 1)..(posicion.1 + longitud + 1) {
            posiciones.push((posicion.0 - 1, j));
            posiciones.push((posicion.0 + 1, j));
        }

        if posiciones.iter()
            .filter_map(|(i, j)| {
                let i = usize::try_from(*i).ok()?;
                let j = usize::try_from(*j).ok()?;

                mapa.get(i)?.get(j)
            })
            .any(|caracter| *caracter != '.') {
            return Some(elemento.valor);
        }

        None
    }
}

#[cfg(test)]
mod pruebas_dia_3 {
    use super::*;

    #[test]
    fn una_linea_con_tres_numero() {
        let mapa = Day3::<35, 1>::crear_mapa(
        ".431...535...*...............622.-."
        );
        let elementos: Vec<Elemento> = Day3::<35, 1>::obtener_elementos(&mapa);
        let elemento431 = Elemento::new(431, (0, 1));
        let elemento535 = Elemento::new(535, (0, 7));
        let elemento622 = Elemento::new(622, (0, 29));

        assert_eq!(elemento431, elementos[0]);
        assert_eq!(elemento535, elementos[1]);
        assert_eq!(elemento622, elementos[2]);
    }

    #[test]
    fn una_linea_con_dos_numero_en_extremos() {
        let mapa = Day3::<35, 1>::crear_mapa(
        "431..........*................-.622"
        );
        let elementos: Vec<Elemento> = Day3::<35, 1>::obtener_elementos(&mapa);
        let elemento431 = Elemento::new(431, (0, 0));
        let elemento622 = Elemento::new(622, (0, 32));

        assert_eq!(elemento431, elementos[0]);
        assert_eq!(elemento622, elementos[1]);
    }

    #[test]
    fn elemento_valido_simbolo_a_los_costados() {
        let mapa = Day3::<35, 1>::crear_mapa(
        ".431$..535...*...............622-.."
        );
        let elemento431 = Elemento::new(431, (0, 1));
        let elemento535 = Elemento::new(535, (0, 7));
        let elemento622 = Elemento::new(622, (0, 29));

        assert_eq!(Some(431), Day3::<35, 1>::elemento_valido(&elemento431, &mapa));
        assert_eq!(None, Day3::<35, 1>::elemento_valido(&elemento535, &mapa));
        assert_eq!(Some(622), Day3::<35, 1>::elemento_valido(&elemento622, &mapa));
    }

    #[test]
    fn elemento_valido_simbolo_en_diagonales() {
        let mapa = Day3::<35, 3>::crear_mapa(
        "....&...........*..................\n.431...535.......356...123...622...\n......#.....................-......"
        );
        let elemento431 = Elemento::new(431, (1, 1));
        let elemento535 = Elemento::new(535, (1, 7));
        let elemento356 = Elemento::new(356, (1, 17));
        let elemento123 = Elemento::new(123, (1, 23));
        let elemento622 = Elemento::new(622, (1, 29));

        assert_eq!(Some(431), Day3::<35, 3>::elemento_valido(&elemento431, &mapa));
        assert_eq!(Some(535), Day3::<35, 3>::elemento_valido(&elemento535, &mapa));
        assert_eq!(Some(356), Day3::<35, 3>::elemento_valido(&elemento356, &mapa));
        assert_eq!(None,      Day3::<35, 3>::elemento_valido(&elemento123, &mapa));
        assert_eq!(Some(622), Day3::<35, 3>::elemento_valido(&elemento622, &mapa));
    }

}