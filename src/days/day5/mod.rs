mod map;
mod group_map;

use super::day::Day;
use group_map::GroupMap;

pub const ARCHIVO_DIA_5: &str = "src/days/day5/file";

pub struct Day5 {
    file: String,
}

impl Day for Day5 {
    fn resultado(&self) -> (u32, u32) {
        let (mut seeds, group_maps) = match self.file.split_once("\n") {
            Some((seeds, group_maps)) => (
                Self::get_seeds(seeds), 
                Self::get_group_maps(group_maps)
            ),
            None => return (0, 0),
        };

        (
            Self::get_minimum_seed(&mut seeds, &group_maps),
            0
        )
    }
}

impl Day5 {
    pub fn new(file: String) -> Day5 {
        Day5 { file }
    }

    pub fn get_seeds(seeds: &str) -> Vec<u32> {
        seeds.split(" ")
            .filter_map(|seed| seed.parse::<u32>().ok())
            .collect::<Vec<u32>>()
    }

    pub fn get_group_maps(group_maps: &str) -> Vec<GroupMap> {
        vec![]
    }


    pub fn get_minimum_seed(seeds: &mut Vec<u32>, group_maps: &Vec<GroupMap>) -> u32 {
        group_maps.iter()
            .for_each(|group_map| group_map.transform(seeds));

        *seeds.iter().min().unwrap_or(&u32::MAX)
    }
}