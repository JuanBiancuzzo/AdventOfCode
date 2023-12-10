mod ejercicio;
mod day1;
mod day2;

use ejercicio::Ejercicio;
use day1::ejercicio::Day1;
use day2::ejercicio::Day2;

fn get_file_content(file_name: &str) -> String {
    std::fs::read_to_string(file_name).expect("Error al leer el archivo")
}

fn main() {
    let ejercicios: Vec<Box<dyn Ejercicio>> = vec![
        Box::new(Day1::new(get_file_content("src/day1/file"))),
        Box::new(Day2::new(get_file_content("src/day2/file"))),
    ];

    for (idx, ejercicio) in ejercicios.iter().enumerate() {
        let (parte1, parte2) = ejercicio.resultado();
        println!("Ejercicio {}: \n\tParte 1: {}, \n\tParte 2: {}", idx + 1, parte1, parte2);
    }
}
