use std::convert::From;

use super::map::Map;

pub struct GroupMap {
    maps: Vec<Map>,
}

impl GroupMap {
    pub fn new(maps: Vec<Map>) -> Self {
        Self { maps }
    }

    pub fn transform(&self, sources: &mut Vec<u32>) {
        for source in sources.iter_mut() {
            *source = self.transform_source(*source);
        }
    }

    fn transform_source(&self, source: u32) -> u32 {
        for map in self.maps.iter() {
            if let Some(destination) = map.transform(source) {
                return destination;
            }
        }
        source
    }
}

impl From<&str> for GroupMap {
    fn from(value: &str) -> Self {
        Self::new(
            value.lines()
                .filter_map(|line| Map::try_from(line).ok())
                .collect::<Vec<Map>>(),
        )
    }
}