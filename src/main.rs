mod ejercicio;
mod day1;
mod day2;
mod day3;
mod day4;

use ejercicio::Ejercicio;
use day1::ejercicio::{Day1, ARCHIVO_DIA_1};
use day2::ejercicio::{Day2, ARCHIVO_DIA_2, CANT_ROJO, CANT_VERDE, CANT_AZUL};
use day3::ejercicio::{Day3, ARCHIVO_DIA_3};
use day4::ejercicio::{Day4, ARCHIVO_DIA_4};
use std::fs::read_to_string;

pub fn get_file_content(file_name: &str) -> String {
    read_to_string(file_name).expect("Error al leer el archivo")
}

fn main() {
    let ejercicios: Vec<Box<dyn Ejercicio>> = vec![
        Box::new(Day1::new(get_file_content(ARCHIVO_DIA_1))),
        Box::new(Day2::new(
            get_file_content(ARCHIVO_DIA_2),
            CANT_ROJO,
            CANT_VERDE,
            CANT_AZUL,
        )),
        Box::new(Day3::<140, 140>::new(get_file_content(ARCHIVO_DIA_3))),
        Box::new(Day4::new(get_file_content(ARCHIVO_DIA_4))),
    ];

    for (idx, ejercicio) in ejercicios.iter().enumerate() {
        let (parte1, parte2) = ejercicio.resultado();
        println!("Ejercicio {}: \n\tParte 1: {}, \n\tParte 2: {}", idx + 1, parte1, parte2);
    }
}
