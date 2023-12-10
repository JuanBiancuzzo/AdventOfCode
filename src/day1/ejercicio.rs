use crate::ejercicio::Ejercicio;

pub struct Day1 {
    file: String,
}

impl Ejercicio for Day1 {
    fn resultado(&self) -> (u32, u32) {
        (
            self.calcular_codigo(&self.file, Self::calcular_linea_digitos),
            self.calcular_codigo(&self.file, Self::calcular_linea_completo)
        )
    }
}

impl Day1 {

    pub fn new(file: String) -> Self {
        Self { file }
    }

    fn get_numero(linea: &str) -> Option<u32> {
        match linea.chars().next() {
            Some(c) if c.is_digit(10) => return c.to_digit(10),
            _ => {}
        }

        if linea.len() < 3 {
            return None;
        }

        match &linea[..3] {
            "one" => return Some(1),
            "two" => return Some(2),
            "six" => return Some(6),
            _ => {}        
        }

        if linea.len() < 4 {
            return None;
        }

        match &linea[..4] {
            "four" => return Some(4),
            "five" => return Some(5),
            "nine" => return Some(9),
            _ => {}        
        }

        if linea.len() < 5 {
            return None;
        }

        match &linea[..5] {
            "three" => Some(3),
            "seven" => Some(7),
            "eight" => Some(8),
            _ => None        
        }
    }

    fn calcular_linea_digitos(linea: &str) -> Option<u32> {
        let numeros: Vec<u32> = linea.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        let primero = numeros.first();
        let ultimo = numeros.last();

        match (primero, ultimo) {
            (Some(primero), Some(ultimo)) => Some(primero * 10 + ultimo),
            _ => None
        }
    }

    fn calcular_linea_completo(linea: &str) -> Option<u32> {
        let mut numeros: Vec<u32> = Vec::new();
        let linea = linea.to_string();
        
        for i in 0..linea.len() {
            match Self::get_numero(&linea[i..]) {
                Some(numero) => numeros.push(numero),
                None => continue,
            }
        }

        let primero = numeros.first();
        let ultimo = numeros.last();

        match (primero, ultimo) {
            (Some(primero), Some(ultimo)) => Some(primero * 10 + ultimo),
            _ => None
        }
    }

    fn calcular_codigo<F>(&self, file: &String, calculo_de_linea: F) -> u32 
        where F: Fn(&str) -> Option<u32>
    {
        file.lines()
            .map(|linea| {
                let numero = calculo_de_linea(linea);
                return match numero {
                    Some(numero) => numero,
                    None => 0,
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    
    #[test]
    fn simple() {
        let file = "123\n456\n789";
        let ejercicio = Day1::new(file.to_string());
        let codigo = ejercicio.resultado();
        assert_eq!(codigo.0, 138);
    }
    
    #[test]
    fn medio() {
        let file = "asfa12dafa3sdfasdf\nasdfa45fasdf6asdf\n7asdfasd89asdf\nasddfasdf7asd5fasdf";
        let ejercicio = Day1::new(file.to_string());
        let codigo = ejercicio.resultado();
        assert_eq!(codigo.0, 213);
    }

    #[test]
    fn unico_valor() {
        let file = "asfadafa3sdfasdf";
        let ejercicio = Day1::new(file.to_string());
        let codigo = ejercicio.resultado();
        assert_eq!(codigo.0, 33);
    }

    #[test]
    fn con_letras() {
        let file = "one\ntwo\nthree\nfour\nfive\nsix\nseven\neight\nnine";
        let ejercicio = Day1::new(file.to_string());
        let codigo = ejercicio.resultado();

        let codigo_esperado = (1..10).map(|valor| valor * 11).sum();
        assert_eq!(codigo.1, codigo_esperado);
    }

    #[test]
    fn completo() {
        let file = "oneasdjfkj1\nkalsjdf2asdfjalsdkjftwoaksjdf\nasdfasd3asdfasdfthreeasdfasd\n4asdfasdffourasdfasdf\nasdfad5fiveasdfasd\nsadfasdf6asdfasdfsix\nasdfadsf7asdfasdfsevenasdfaea\n8fasdfasdfasdfeight\nasdfadsfnineasdfasdfa9asdfad";
        let ejercicio = Day1::new(file.to_string());
        let codigo = ejercicio.resultado();

        let codigo_esperado = (1..10).map(|valor| valor * 11).sum();
        assert_eq!(codigo.1, codigo_esperado);
    }
}