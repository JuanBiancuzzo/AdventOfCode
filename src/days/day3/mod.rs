mod elemento;
mod gear;

use super::day::Day;
use elemento::Elemento;
use gear::Gear;

pub const ARCHIVO_DIA_3: &str = "src/days/day3/file";

pub struct Day3<const N: usize, const M: usize> {
    file: String,
}

impl<const N: usize, const M: usize> Day for Day3<N, M> {
    fn resultado_parte_1(&self) -> u64 {
        let mapa = Self::crear_mapa(&self.file);
        Self::calcular_valores(&mapa)
    }

    fn resultado_parte_2(&self) -> u64 {
        let mapa = Self::crear_mapa(&self.file);
        Self::calcular_gear_ratios(&mapa)
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

    fn calcular_valores(mapa: &[[char; N]; M]) -> u64 {
        Self::obtener_elementos(mapa).iter()
            .filter_map(|elemento| elemento.elemento_valido::<N, M>(mapa))
            .sum::<u64>()
    }

    fn calcular_gear_ratios(mapa: &[[char; N]; M]) -> u64 {
        Self::obtener_gears(mapa).iter()
            .filter_map(|gear| gear.gear_ratio(mapa))
            .sum::<u64>()
    }

    pub fn obtener_elementos(mapa: &[[char; N]; M]) -> Vec<Elemento> {
        let mut elementos: Vec<Elemento> = vec![];

        for (i, linea) in mapa.iter().enumerate() {
            let mut j = 0;
            let mut linea_iterable = linea.iter();

            while let Some(caracter) = linea_iterable.next() {
                let mut valor = match caracter.to_digit(10) {
                    Some(valor) => valor as u64,
                    None => {
                        j += 1;
                        continue
                    }
                };
                let inicio = j;
                while let Some(caracter) = linea_iterable.next() {
                    j += 1;
                    match caracter.to_digit(10) {
                        Some(siguiente_valor) => valor = 10 * valor + siguiente_valor as u64,
                        None => break,
                    };
                }
                elementos.push(Elemento::new(valor, (i, inicio)));
                j += 1;
            }
        }

        elementos
    }

    fn obtener_gears(mapa: &[[char; N]; M]) -> Vec<Gear> {
        let mut gears: Vec<Gear> = vec![];

        for (i, linea) in mapa.iter().enumerate() {
            for (j, caracter) in linea.iter().enumerate() {
                if *caracter == '*' {
                    gears.push(Gear::new((i as u64, j as u64)));
                }
            }
        }

        gears
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

        assert_eq!(Some(431), elemento431.elemento_valido::<35, 1>(&mapa));
        assert_eq!(None, elemento535.elemento_valido::<35, 1>(&mapa));
        assert_eq!(Some(622), elemento622.elemento_valido::<35, 1>(&mapa));
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

        assert_eq!(Some(431), elemento431.elemento_valido::<35, 3>(&mapa));
        assert_eq!(Some(535), elemento535.elemento_valido::<35, 3>(&mapa));
        assert_eq!(Some(356), elemento356.elemento_valido::<35, 3>(&mapa));
        assert_eq!(None,      elemento123.elemento_valido::<35, 3>(&mapa));
        assert_eq!(Some(622), elemento622.elemento_valido::<35, 3>(&mapa));
    }

    #[test]
    fn obtener_gears_del_mapa() {
        let mapa = Day3::<35, 3>::crear_mapa(
            "..*.......25..13.....64....48......\n....*....*....*...87*.......*24....\n............76......20....58.57...."
            );
        
        let gear1 = Gear::new((0, 2));
        let gear2 = Gear::new((1, 4));
        let gear3 = Gear::new((1, 9));
        let gear4 = Gear::new((1, 14));
        let gear5 = Gear::new((1, 20));
        let gear6 = Gear::new((1, 28));

        let gears = Day3::<35, 3>::obtener_gears(&mapa);

        assert_eq!(gears[0], gear1);
        assert_eq!(gears[1], gear2);
        assert_eq!(gears[2], gear3);
        assert_eq!(gears[3], gear4);
        assert_eq!(gears[4], gear5);
        assert_eq!(gears[5], gear6);
    }

    #[test]
    fn gear_con_cero_uno_dos_tres_cuatro_elementos() {
        let mapa = Day3::<35, 3>::crear_mapa(
            "..*.......25..13.....64....48......\n....*....*....*...87*.......*24....\n............76......20....58.57...."
            );
        
        let gear1 = Gear::new((0, 2));
        let gear2 = Gear::new((1, 4));
        let gear3 = Gear::new((1, 9));
        let gear4 = Gear::new((1, 14));
        let gear5 = Gear::new((1, 20));
        let gear6 = Gear::new((1, 28));

        assert_eq!(None, gear1.gear_ratio(&mapa));
        assert_eq!(None, gear2.gear_ratio(&mapa));
        assert_eq!(None, gear3.gear_ratio(&mapa));
        assert_eq!(Some(13 * 76), gear4.gear_ratio(&mapa));
        assert_eq!(None, gear5.gear_ratio(&mapa));
        assert_eq!(None, gear6.gear_ratio(&mapa));
    }

    #[test]
    fn gear_con_varias_configuraciones() {
        let mapa = Day3::<35, 3>::crear_mapa(
            "...*762.....24.......12*754....647.\n.34...*..57*...123..............*..\n....59.......58*......23*45....873."
            );
        
        let gear1 = Gear::new((0, 3));
        let gear2 = Gear::new((0, 23));
        let gear3 = Gear::new((1, 6));
        let gear4 = Gear::new((1, 11));
        let gear5 = Gear::new((1, 32));
        let gear6 = Gear::new((2, 15));
        let gear7 = Gear::new((2, 24));

        assert_eq!(Some(762 * 34), gear1.gear_ratio(&mapa));
        assert_eq!(Some(754 * 12), gear2.gear_ratio(&mapa));
        assert_eq!(Some(762 * 59), gear3.gear_ratio(&mapa));
        assert_eq!(Some(57 * 24), gear4.gear_ratio(&mapa));
        assert_eq!(Some(647 * 873), gear5.gear_ratio(&mapa));
        assert_eq!(Some(58 * 123), gear6.gear_ratio(&mapa));
        assert_eq!(Some(23 * 45), gear7.gear_ratio(&mapa));
    }
}