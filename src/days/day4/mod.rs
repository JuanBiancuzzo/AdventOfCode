mod vector_ordenado;

use vector_ordenado::InsersionOrdenada;
use super::day::Day;

pub const ARCHIVO_DIA_4: &str = "src/days/day4/file";

pub struct Day4 {
    file: String,
}

impl Day for Day4 {
    fn resultado(&self) -> (u32, u32) {
        (
            Self::calcular_linea(&self.file),
            Self::calcular_cantidad_scratchcards(&self.file),
        )
    }
}

impl Day4 {
    pub fn new(file: String) -> Self {
        Day4 { file }
    }

    fn calcular_linea(lineas: &str) -> u32
    {
        lineas.lines()
            .filter_map(|linea| Self::cantidad_matcheos(linea))
            .map(|puntos| {
                match puntos {
                    0 => 0,
                    1 => 1,
                    n => 2u32.pow(n - 1),
                }
            })
            .sum()
    }

    fn calcular_cantidad_scratchcards(lineas: &str) -> u32 {
        let mut scratchcards: Vec<u32> = vec![1; lineas.lines().map(|_| 1).sum::<usize>()];

        lineas.lines().enumerate().for_each(|(index, linea)| {
            Self::modificar_cantidad_scratchards(index, linea, &mut scratchcards)
        });        

        scratchcards.iter().sum()
    }

    fn modificar_cantidad_scratchards(index: usize, linea: &str, scratchcards: &mut Vec<u32>) {
        let puntos = Self::cantidad_matcheos(linea).unwrap_or(0);
        let cantidad_actual = scratchcards[index];
        for i in 0..puntos {
            let index = index + 1 + i as usize;
            scratchcards[index] += cantidad_actual;
        }
    }

    fn cantidad_matcheos(linea: &str) -> Option<u32> {
        let (_, linea) = linea.split_once(":")?;
        let (winning_numbers, numbers) = linea.split_once("|")?;
        
        let winning_numbers = Self::numeros_ordenados(winning_numbers);
        let mut winning_numbers = winning_numbers.iter();

        let numbers = Self::numeros_ordenados(numbers);
        let mut numbers = numbers.iter();

        let mut next_winning_number = winning_numbers.next();
        let mut next_number = numbers.next();
        
        let mut puntos: u32 = 0;

        while let (Some(winning_number), Some(number)) = (next_winning_number, next_number) {
            if winning_number == number {
                puntos += 1;
                next_winning_number = winning_numbers.next();
                next_number = numbers.next();
            } else if winning_number > number {
                next_number = numbers.next();
            } else {
                next_winning_number = winning_numbers.next();
            }
        }

        Some(puntos)
    }

    fn numeros_ordenados(numeros: &str) -> Vec<u32> {
        let mut ordenados: Vec<u32> = vec![];

        numeros.split_whitespace()
            .filter_map(|numero| numero.parse::<u32>().ok())
            .for_each(|numero| ordenados.insertar_ordenadamente(numero));
        
        ordenados
    }
}

#[cfg(test)]
mod pruebas_dia_4 {
    use super::*;

    #[test]
    fn linea_con_numeros_ganadores() {
        let linea = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let resultado = Day4::new(linea.to_string()).resultado();

        assert_eq!(resultado.0, 8);
    }

    #[test]
    fn linea_sin_numeros_ganadores() {
        let linea = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";

        let resultado = Day4::new(linea.to_string()).resultado();

        assert_eq!(resultado.0, 0);
    }

    #[test]
    fn varias_lineas_intercaladas() {
        let lineas = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let resultado = Day4::new(lineas.to_string()).resultado();

        assert_eq!(resultado.0, 13);
    }

    #[test]
    fn cantidad_scratchcards_correcta() {
        let lineas = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let resultado = Day4::new(lineas.to_string()).resultado();

        assert_eq!(resultado.1, 30);
    }
}