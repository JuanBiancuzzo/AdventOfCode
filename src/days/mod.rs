pub mod day;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;

use std::fs::read_to_string;

use {day::Day,
    day1::{Day1, ARCHIVO_DIA_1},
    day2::{Day2, ARCHIVO_DIA_2, CANT_ROJO, CANT_VERDE, CANT_AZUL},
    day3::{Day3, ARCHIVO_DIA_3},
    day4::{Day4, ARCHIVO_DIA_4},
    day5::{Day5, ARCHIVO_DIA_5},
    day6::{Day6, ARCHIVO_DIA_6},
    day7::{Day7, ARCHIVO_DIA_7},
    day8::{Day8, ARCHIVO_DIA_8},
    day9::{Day9, ARCHIVO_DIA_9},
    day10::{Day10, ARCHIVO_DIA_10},
};

pub fn get_file_content(file_name: &str) -> String {
    read_to_string(file_name).expect("Error al leer el archivo")
}

pub fn get_days() -> Vec<Box<dyn Day>> {
    vec![
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
        Box::new(Day10::<140, 140>::new(get_file_content(ARCHIVO_DIA_10))),
    ]
}