pub mod day;
pub mod day_count;

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
pub mod day11;
pub mod day12;

use std::{fs::read_to_string, sync::{Mutex, Arc}};

type ArcMut<T> = Arc<Mutex<T>>;

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
    day11::{Day11, ARCHIVO_DIA_11},
    day12::{Day12, ARCHIVO_DIA_12},
};

fn get_file_content(file_name: &str) -> String {
    read_to_string(file_name).expect("Error al leer el archivo")
}

fn get_arc_mutex(day: impl Day + 'static) -> ArcMut<dyn Day> {
    Arc::new(Mutex::new(day))
}

pub fn get_days() -> Vec<ArcMut<dyn Day>> {
    vec![
        get_arc_mutex(Day1::new(get_file_content(ARCHIVO_DIA_1))),
        get_arc_mutex(Day2::new(get_file_content(ARCHIVO_DIA_2), CANT_ROJO, CANT_VERDE, CANT_AZUL)),
        get_arc_mutex(Day3::<140, 140>::new(get_file_content(ARCHIVO_DIA_3))),
        get_arc_mutex(Day4::new(get_file_content(ARCHIVO_DIA_4))),
        get_arc_mutex(Day5::new(get_file_content(ARCHIVO_DIA_5))),
        get_arc_mutex(Day6::new(get_file_content(ARCHIVO_DIA_6))),
        get_arc_mutex(Day7::new(get_file_content(ARCHIVO_DIA_7))),
        get_arc_mutex(Day8::new(get_file_content(ARCHIVO_DIA_8))),
        get_arc_mutex(Day9::new(get_file_content(ARCHIVO_DIA_9))),
        get_arc_mutex(Day10::<140, 140>::new(get_file_content(ARCHIVO_DIA_10))),
        get_arc_mutex(Day11::<140, 140>::new(get_file_content(ARCHIVO_DIA_11))),
        get_arc_mutex(Day12::new(get_file_content(ARCHIVO_DIA_12))),
    ]
}