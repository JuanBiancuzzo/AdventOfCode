mod days;

use std::fs::read_to_string;

use days::{
    day::Day,
    day1::{Day1, ARCHIVO_DIA_1},
    day2::{Day2, ARCHIVO_DIA_2, CANT_ROJO, CANT_VERDE, CANT_AZUL},
    day3::{Day3, ARCHIVO_DIA_3},
    day4::{Day4, ARCHIVO_DIA_4},
    day5::{Day5, ARCHIVO_DIA_5},
};

pub fn get_file_content(file_name: &str) -> String {
    read_to_string(file_name).expect("Error al leer el archivo")
}

fn main() {
    let ejercicios: Vec<Box<dyn Day>> = vec![
        Box::new(Day1::new(get_file_content(ARCHIVO_DIA_1))),
        Box::new(Day2::new(
            get_file_content(ARCHIVO_DIA_2),
            CANT_ROJO,
            CANT_VERDE,
            CANT_AZUL,
        )),
        Box::new(Day3::<140, 140>::new(get_file_content(ARCHIVO_DIA_3))),
        Box::new(Day4::new(get_file_content(ARCHIVO_DIA_4))),
        Box::new(Day5::new(get_file_content(ARCHIVO_DIA_5))),
    ];

    for (idx, ejercicio) in ejercicios.iter().enumerate() {
        let (parte1, parte2) = ejercicio.resultado();
        println!("Ejercicio {}: \n\tParte 1: {}, \n\tParte 2: {}", idx + 1, parte1, parte2);
    }
}
