use super::day::Day;

pub const ARCHIVO_DIA_2: &str = "src/days/day2/file";
pub const CANT_ROJO: u64 = 12;
pub const CANT_VERDE: u64 = 13;
pub const CANT_AZUL: u64 = 14;

pub struct Day2 {
    file: String,
    max_rojo: u64,
    max_verde: u64,
    max_azul: u64,
}

impl Day for Day2 {
    fn resultado(&self) -> (u64, u64) {
        (
            self.calcular_linea(&self.file, Self::filtrar_juego_valido),
            self.calcular_linea(&self.file, Self::filtrar_juego_minimo),
        )
    }
}

impl Day2 {
    pub fn new(file: String, max_rojo: u64, max_verde: u64, max_azul: u64) -> Self {
        Self { 
            file,
            max_rojo,
            max_verde,
            max_azul,
        }
    }

    fn calcular_linea<F>(&self, lineas: &str, filtro: F) -> u64
        where F: Fn(&Self, &str) -> Option<u64>
    {
        lineas.lines()
            .filter_map(|linea| filtro(self, linea))
            .sum()
    }   

    fn filtrar_juego_valido(&self, linea: &str) -> Option<u64> {
        let (juego, linea) = linea.split_once(":")?;

        match linea.split(";")
            .all(|subset| self.es_subset_valido(subset)) {
            true => Some(Self::get_numero_juego(juego)),
            false => None
        }
    }

    fn filtrar_juego_minimo(&self, linea: &str) -> Option<u64> {
        let (_, linea) = linea.split_once(":")?;

        let (rojo, verde, azul) = linea.split(";")
        .map(|subset| {
            subset.split(",")
            .filter_map(|valores| Self::get_cantidad_por_color(valores))
            .reduce(|(rojo1, verde1, azul1), (rojo2, verde2, azul2)| {
                ( rojo1 + rojo2, verde1 + verde2, azul1 + azul2)
            }).unwrap_or((0, 0, 0))
        })
        .reduce(|(rojo1, verde1, azul1), (rojo2, verde2, azul2)| {
            (
                if rojo1 > rojo2 { rojo1 } else { rojo2 }, 
                if verde1 > verde2 { verde1 } else { verde2 },
                if azul1 > azul2 { azul1 } else { azul2 },
            )
        })?;

        Some(rojo * verde * azul)
    }

    fn es_subset_valido(&self, subset: &str) -> bool {
        if let Some((rojo, verde, azul)) = subset.split(",")
            .filter_map(|valores| Self::get_cantidad_por_color(valores))
            .reduce(|(rojo1, verde1, azul1), (rojo2, verde2, azul2)| {
                (
                    if rojo1 > rojo2 { rojo1 } else { rojo2 }, 
                    if verde1 > verde2 { verde1 } else { verde2 },
                    if azul1 > azul2 { azul1 } else { azul2 },
                )
            }) {
            return rojo <= self.max_rojo && verde <= self.max_verde && azul <= self.max_azul;
        }
        false
    }

    fn get_cantidad_por_color(par_valor_color: &str) -> Option<(u64, u64, u64)> {
        let (valor, color) = par_valor_color.trim().split_once(" ")?;
        let valor = valor.parse::<u64>().ok()?;
        
        if color.contains("red") { 
            return Some((valor, 0, 0));
        } else if color.contains("green") { 
            return Some((0, valor, 0));
        } else if color.contains("blue") { 
            return Some((0, 0, valor));
        }

        None
    }

    fn get_numero_juego(juego: &str) -> u64 {
        let numeros: Vec<u64> = juego.chars()
            .filter_map(|c| return match c.to_digit(10) {
                Some(digit) => Some(digit as u64),
                None => None,
            })
            .collect();

        return numeros.iter().enumerate().map(|(idx, numero)| {
            let potencia = (numeros.len() - idx - 1) as u32;
            numero * 10u64.pow(potencia)
        }).sum::<u64>();
    }
}

#[cfg(test)]
mod pruebas_dia_2 {
    use super::*;

    fn day2_default(file: String) -> Day2 {
        Day2::new(file, CANT_ROJO, CANT_VERDE, CANT_AZUL)
    }

    #[test]
    fn linea_valida() {
        let file = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let ejercicio = day2_default(file.to_string());
        let codigo = ejercicio.resultado();
        assert_eq!(codigo.0, 1);
        assert_eq!(codigo.1, 4 * 2 * 6);
    }

    #[test]
    fn linea_invalida() {
        let file = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let ejercicio = day2_default(file.to_string());
        let codigo = ejercicio.resultado();
        assert_eq!(codigo.0, 0);
        assert_eq!(codigo.1, 20 * 13 * 6);
    }

    #[test]
    fn lineas_intercaladas() {
        let file = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n
                          Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n
                          Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n
                          Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n
                          Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n";
        let ejercicio = day2_default(file.to_string());
        let codigo = ejercicio.resultado();
        assert_eq!(codigo.0, 8);
        assert_eq!(codigo.1, 48 + 12 + 1560 + 630 + 36)
    }

    pub const CANT_ROJO: u64 = 12;
    pub const CANT_VERDE: u64 = 13;
    pub const CANT_AZUL: u64 = 14;
    #[test]
    fn varias_lineas_intercaladas() {
        let file = "Game 1: 4 red, 3 blue; 6 blue, 16 green; 9 blue, 13 green, 1 red; 10 green, 4 red, 6 blue\n
        Game 2: 2 green, 3 blue; 11 red; 2 green, 5 red, 1 blue\n
        Game 3: 19 green, 4 blue, 13 red; 1 green, 1 blue, 1 red; 17 red, 18 green\n
        Game 4: 4 green, 8 blue, 20 red; 19 red, 3 green, 14 blue; 15 red, 4 green, 1 blue; 18 blue, 14 red; 19 red, 10 blue; 3 green, 11 blue, 15 red\n
        Game 5: 1 red, 3 blue, 15 green; 13 green, 2 blue; 6 green; 6 green, 8 blue; 4 green, 9 blue, 1 red\n
        Game 6: 2 green, 10 red; 4 blue, 1 red, 2 green; 2 red, 2 blue, 1 green; 5 red, 3 green, 1 blue\n
        Game 7: 4 green, 2 blue, 10 red; 1 green, 12 red; 5 green, 12 red, 2 blue; 10 red, 1 blue, 5 green; 1 green, 1 blue, 11 red\n
        Game 8: 8 blue, 3 red, 1 green; 9 blue, 14 green, 6 red; 3 red, 15 blue, 16 green; 9 red, 4 green, 6 blue\n
        Game 9: 9 blue, 9 red, 5 green; 6 red, 1 green, 12 blue; 7 blue, 3 green; 4 red, 12 blue, 1 green; 5 red, 4 green, 1 blue\n
        Game 10: 1 blue, 2 red, 19 green; 7 green, 5 blue, 7 red; 2 blue, 1 red, 3 green; 2 blue, 9 red, 10 green\n
        Game 11: 2 red, 17 blue, 12 green; 5 green, 3 blue; 14 green, 2 red, 15 blue\n
        Game 12: 4 blue, 13 green, 1 red; 5 blue, 3 green, 4 red; 8 blue, 15 green; 12 blue, 5 red, 6 green; 2 green, 5 blue, 4 red; 11 blue, 18 green, 4 red\n
        Game 13: 8 blue, 11 red, 2 green; 18 red, 7 blue, 7 green; 6 green, 9 red; 7 green, 3 blue, 12 red; 1 green, 4 red, 4 blue\n
        Game 14: 3 green, 11 blue, 1 red; 3 green, 1 red, 13 blue; 5 green, 6 blue, 1 red; 1 red, 5 blue, 5 green; 10 blue, 2 green\n
        Game 15: 3 red, 8 green, 1 blue; 8 green, 10 red, 3 blue; 1 blue, 4 green, 2 red; 10 red, 10 green; 3 blue, 4 green, 3 red; 12 green, 7 red\n
        Game 16: 13 red, 9 blue; 2 green, 7 red, 7 blue; 9 blue, 7 red, 7 green; 13 blue, 10 red\n
        Game 17: 12 red, 19 green, 4 blue; 2 blue, 5 red, 11 green; 4 red, 7 green, 8 blue; 6 red, 10 green; 3 green, 7 red, 10 blue\n
        Game 18: 2 blue, 6 red; 5 red, 3 green; 12 red, 1 blue, 3 green; 1 green, 19 red, 5 blue; 3 green, 2 blue, 16 red\n
        Game 19: 10 red, 5 green; 10 red; 9 red, 7 blue; 1 blue, 8 red\n
        Game 20: 11 green, 5 red, 7 blue; 7 green, 12 red, 11 blue; 13 green, 3 blue, 5 red; 3 red, 3 blue, 1 green";

        let ejercicio = day2_default(file.to_string());
        let codigo = ejercicio.resultado();
        assert_eq!(codigo.0, 92);
    }
}