mod days;

use std::fs::read_to_string;

use days::{
    day::Day,
    day1::{Day1, ARCHIVO_DIA_1},
    day2::{Day2, ARCHIVO_DIA_2, CANT_ROJO, CANT_VERDE, CANT_AZUL},
    day3::{Day3, ARCHIVO_DIA_3},
    day4::{Day4, ARCHIVO_DIA_4},
    day5::{Day5, ARCHIVO_DIA_5},
    day6::{Day6, ARCHIVO_DIA_6},
    day7::{Day7, ARCHIVO_DIA_7},
    day8::{Day8, ARCHIVO_DIA_8},
    day9::{Day9, ARCHIVO_DIA_9},
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
        Box::new(Day6::new(get_file_content(ARCHIVO_DIA_6))),
        Box::new(Day7::new(get_file_content(ARCHIVO_DIA_7))),
        Box::new(Day8::new(get_file_content(ARCHIVO_DIA_8))),
        Box::new(Day9::new(get_file_content(ARCHIVO_DIA_9))),
        
    ];

    for (idx, ejercicio) in ejercicios.iter().enumerate() {
        let parte_1 = ejercicio.resultado_parte_1();
        println!("Ejercicio {}: \n\tParte 1: {}", idx + 1, parte_1);

        let parte_2 = ejercicio.resultado_parte_2();
        println!("\tParte 2: {}", parte_2);
    }
}
