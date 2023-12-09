use std::fs;

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
        match get_numero(&linea[i..]) {
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

fn calcular_codigo<F>(file: &str, calculo_de_linea: F) -> u32 
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

pub fn resultado() {
    let file = match fs::read_to_string("src/day1/file") {
        Ok(file) => file,
        Err(e) => return println!("Error reading file: {}", e),
    };
    
    let codigo: u32 = calcular_codigo(file.as_str(), calcular_linea_digitos);
    println!("El codigo de digitos es: {}", codigo);
    
    let codigo: u32 = calcular_codigo(file.as_str(), calcular_linea_completo);
    println!("El codigo completo es: {}", codigo);
}

#[cfg(test)]
mod pruebas {
    use super::*;
    
    #[test]
    fn simple() {
        let file = "123\n456\n789";
        let codigo = calcular_codigo(file, calcular_linea_digitos);
        assert_eq!(codigo, 138);
    }
    
    #[test]
    fn medio() {
        let file = "asfa12dafa3sdfasdf\nasdfa45fasdf6asdf\n7asdfasd89asdf\nasddfasdf7asd5fasdf";
        let codigo = calcular_codigo(file, calcular_linea_digitos);
        assert_eq!(codigo, 213);
    }

    #[test]
    fn unico_valor() {
        let file = "asfadafa3sdfasdf";
        let codigo = calcular_codigo(file, calcular_linea_digitos);
        assert_eq!(codigo, 33);
    }

    #[test]
    fn con_letras() {
        let file = "one\ntwo\nthree\nfour\nfive\nsix\nseven\neight\nnine";
        let codigo = calcular_codigo(file, calcular_linea_completo);

        let codigo_esperado = (1..10).map(|valor| valor * 11).sum();
        assert_eq!(codigo, codigo_esperado);
    }

    #[test]
    fn completo() {
        let file = "oneasdjfkj1\nkalsjdf2asdfjalsdkjftwoaksjdf\nasdfasd3asdfasdfthreeasdfasd\n4asdfasdffourasdfasdf\nasdfad5fiveasdfasd\nsadfasdf6asdfasdfsix\nasdfadsf7asdfasdfsevenasdfaea\n8fasdfasdfasdfeight\nasdfadsfnineasdfasdfa9asdfad";
        let codigo = calcular_codigo(file, calcular_linea_completo);

        let codigo_esperado = (1..10).map(|valor| valor * 11).sum();
        assert_eq!(codigo, codigo_esperado);
    }
}