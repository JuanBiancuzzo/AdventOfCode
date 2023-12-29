mod rl;
mod key;
mod instruction_loop;

use std::collections::HashMap;
use super::day::Day;
use super::day_count::DayCount;

use rl::RL;
use key::Key;
use instruction_loop::InstructionLoop;

pub const ARCHIVO_DIA_8: &str = "src/days/day8/file";

pub struct Day8 {
    instructions: InstructionLoop,
    key_value: HashMap<Key, (Key, Key)>,
}

impl Day for Day8 {
    fn resultado_parte_1(&self) -> u64 {
        Self::number_of_steps(
            &vec![Key::new(['A', 'A', 'A'])],
            |key| key.is_final(),
            self.instructions.clone(), 
            &self.key_value
        )
    }

    fn resultado_parte_2(&self) -> u64 {
        let steps_per_key = Self::initial_values(&self.key_value).iter()
            .map(|key| Self::number_of_steps(
                &vec![*key], 
                |key| key.is_final(), 
                self.instructions.clone(), 
                &self.key_value
            ))
            .collect::<Vec<u64>>();

        Self::minimo_comun_multiplo_vec(&steps_per_key)
    }

    fn day_count(&self) -> DayCount {
        DayCount::Day8
    }
}

impl Day8 {
    pub fn new(file: String) -> Day8 {
        let lines = file.lines().collect::<Vec<&str>>();
        let instructions = InstructionLoop::new(Self::get_instructions(lines[0]));
        let key_value = Self::get_key_value(lines[2..].to_vec());

        Day8 { instructions, key_value }
    }

    fn get_instructions(line: &str) -> Vec<RL> {
        line.chars()
            .filter_map(|c| RL::try_from(c).ok())
            .collect::<Vec<RL>>()
    }

    fn get_key_value(lines: Vec<&str>) -> HashMap<Key, (Key, Key)> {
        let mut key_value: HashMap<Key, (Key, Key)> = HashMap::new();
        
        lines.iter()
            .for_each(|line| {
                let (key, values) = line.split_once('=').unwrap();
                let key = Key::try_from(key).unwrap();
                let values = values.trim()
                    .chars()
                    .filter(|c| *c != '(' && *c != ')')
                    .collect::<String>();
                let (left, right) = values
                    .split_once(',')
                    .unwrap();
                
                key_value.insert(key, (Key::try_from(left).unwrap(), Key::try_from(right).unwrap()));
            });

        key_value
    }

    fn initial_values(key_value: &HashMap<Key, (Key, Key)>) -> Vec<Key> {
        key_value.keys()
            .filter_map(|key| if key.is_initial() { Some(*key) } else { None })
            .collect::<Vec<Key>>()
    }

    fn number_of_steps<F>(
        initial_keys: &Vec<Key>, 
        ending_condition: F, 
        instructions: InstructionLoop, 
        key_value: &HashMap<Key, (Key, Key)>
    ) -> u64 
        where F: Fn(Key) -> bool,
    {        
        let mut steps = 0;
        let mut next_keys = initial_keys.clone();

        for instruction in instructions {
            next_keys = next_keys.iter_mut()
                .map(|key| Self::get_next_key(&instruction, key, key_value))
                .collect::<Vec<Key>>();

            steps += 1;
            if next_keys.iter().all(|key| ending_condition(*key)) {
                break;
            }
        }   
        
        steps
    }

    fn minimo_comun_multiplo(initial_a: u64, initial_b: u64) -> u64 {
        let mut a = initial_a;
        let mut b = initial_b;
        
        while a != b {
            if a < b {
                a += initial_a;
            } else {
                b += initial_b;
            }
        }

        a
    }

    fn minimo_comun_multiplo_vec(vec: &[u64]) -> u64 {
        let mut vec = vec.to_owned();
        vec.sort();
        vec.dedup();

        vec.iter()
            .fold(1, |acc, &value| Self::minimo_comun_multiplo(acc, value))
    }

    fn get_next_key(instruccion: &RL, key: &Key, key_value: &HashMap<Key, (Key, Key)>) -> Key {
        let value = match key_value.get(key) {
            Some(value) => value,
            None => panic!("No se encontro la key: {:?}", key),
        };
        
        match instruccion {
            RL::Left => value.0,
            RL::Right => value.1,
        }
    }
}

#[cfg(test)]
mod pruebas_dia_8 {
    use super::*;

    #[test]
    fn ejemplo_1() {
        let file = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

        let resultado = Day8::new(file.to_string()).resultado_parte_1();

        assert_eq!(resultado, 2);
    }

    #[test]
    fn ejemplo_2() {
        let file = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

        let resultado = Day8::new(file.to_string()).resultado_parte_1();

        assert_eq!(resultado, 6);
    }

    #[test]
    fn ejemplo_3() {
        let file = "RRL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, CCC)
        EEE = (EEE, DDD)
        GGG = (EEE, GGG)
        ZZZ = (ZZZ, ZZZ)";

        let resultado = Day8::new(file.to_string()).resultado_parte_1();

        assert_eq!(resultado, 6);
    }

    #[test]
    fn ejemplo_4() {
        let file = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        let resultado = Day8::new(file.to_string()).resultado_parte_2();

        assert_eq!(resultado, 6);
    }

    #[test]
    fn minimo_comun_multiplo_ejemplo() {
        let resultado = Day8::minimo_comun_multiplo(2, 3);

        assert_eq!(resultado, 6);
    }

    #[test]
    fn minimo_comun_multiplo_vec_ejemplo() {
        let vec = vec![2, 3, 4, 5, 6];
        let resultado = Day8::minimo_comun_multiplo_vec(&vec);

        assert_eq!(resultado, 60);
    }
}