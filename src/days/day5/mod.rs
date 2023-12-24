mod map;
mod group_map;
mod source_range;

use super::day::Day;
use source_range::SourceRange;
use group_map::GroupMap;

pub const ARCHIVO_DIA_5: &str = "src/days/day5/file";

pub struct Day5 {
    file: String,
}

impl Day for Day5 {
    fn resultado(&self) -> (u32, u32) {
        let (seeds, group_maps) = match self.file.split_once("\n") {
            Some((seeds, group_maps)) => (
                seeds, 
                Self::get_group_maps(group_maps)
            ),
            None => return (0, 0),
        };

        (
            Self::get_minimum_seed(&mut Self::get_seeds(seeds), &group_maps),
            Self::get_minimum_seed(&mut Self::get_seeds_ranges(seeds), &group_maps),
        )
    }
}

impl Day5 {
    pub fn new(file: String) -> Day5 {
        Day5 { file }
    }

    fn get_seed_values(seeds: &str) -> Vec<u32> {
        seeds.split(" ")
            .filter_map(|seed| seed.parse::<u32>().ok())
            .collect::<Vec<u32>>()
    }

    fn get_seeds(seeds_text: &str) -> Vec<SourceRange> {
        Self::get_seed_values(seeds_text).iter()
            .map(|seed| SourceRange::new(*seed, 1).unwrap())
            .collect::<Vec<SourceRange>>()
    }

    fn get_seeds_ranges(seeds_ranges_text: &str) -> Vec<SourceRange> {
        let seed_range_values = Self::get_seed_values(seeds_ranges_text);
        let mut seed_ranges: Vec<SourceRange> = vec![];

        for seed_range_values in seed_range_values.windows(2).step_by(2) {
            let (start, range) = (seed_range_values[0], seed_range_values[1]);
            if let Some(seed_range) = SourceRange::new(start, range) {
                seed_ranges.push(seed_range);
            }
        }

        seed_ranges
    }

    fn get_group_maps(group_maps_text: &str) -> Vec<GroupMap> {
        let mut group_maps: Vec<GroupMap> = vec![];

        let group_maps_text = group_maps_text.lines().collect::<Vec<&str>>();
        let mut indices = group_maps_text.iter()
            .enumerate()
            .filter_map(|(index, linea)| if linea.contains("map") { Some(index) } else { None })
            .collect::<Vec<usize>>();
        indices.push(group_maps_text.len());

        for group_map_index in indices.windows(2) {
            let (index_inicio, index_final) = (group_map_index[0], group_map_index[1]);
            let group_map = group_maps_text[index_inicio + 1..index_final].join("\n");
            group_maps.push(GroupMap::from(group_map.as_str()));
        }

        group_maps
    }


    fn get_minimum_seed(seeds: &Vec<SourceRange>, group_maps: &Vec<GroupMap>) -> u32 {
        let mut destinations: Vec<SourceRange> = seeds.clone();
        group_maps.iter()
            .for_each(|group_map| {
                destinations = group_map.transform(&destinations);
            });

        destinations.iter()
            .map(|destination_range| destination_range.get_min())
            .reduce(|a, b| if a < b { a } else { b }).unwrap()   
    }
}

#[cfg(test)]
mod pruebas_dia_5 {
    use super::*;
    use super::map::Map;

    #[test]
    fn get_one_group_maps_correctly() {
        let group_maps_text = "soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15";
        
        let group_maps = Day5::get_group_maps(group_maps_text);
        let group_maps_esperado = vec![
            GroupMap::new(vec![
                Map::new(0, 15, 37),
                Map::new(37, 52, 2),
                Map::new(39, 0, 15),
            ])
        ];

        assert_eq!(group_maps, group_maps_esperado);
    }

    #[test]
    fn get_multiple_group_maps_correctly() {
        let group_maps_text = "seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4";
        
        let group_maps = Day5::get_group_maps(group_maps_text);
        let group_maps_esperado = vec![
            GroupMap::new(vec![
                Map::new(50, 98, 2),
                Map::new(52, 50, 48),
            ]),
            GroupMap::new(vec![
                Map::new(0, 15, 37),
                Map::new(37, 52, 2),
                Map::new(39, 0, 15),
            ]),
            GroupMap::new(vec![
                Map::new(49, 53, 8),
                Map::new(0, 11, 42),
                Map::new(42, 0, 7),
                Map::new(57, 7, 4),
            ]),
        ];

        assert_eq!(group_maps, group_maps_esperado);
    }

    #[test]
    fn get_minimum_seed_correctly() {
        let file = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        let resultado = Day5::new(file.to_string()).resultado();

        assert_eq!(resultado.0, 35);
        assert_eq!(resultado.1, 46);
    }
}