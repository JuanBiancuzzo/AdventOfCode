use std::convert::From;

use super::map::Map;
use super::source_range::SourceRange;

#[derive(Debug, Clone, PartialEq)]
pub struct GroupMap {
    maps: Vec<Map>,
}

impl GroupMap {
    pub fn new(maps: Vec<Map>) -> Self {
        Self { maps }
    }

    pub fn transform(&self, sources: &Vec<SourceRange>) -> Vec<SourceRange> {
        let mut destinations: Vec<SourceRange> = vec![];
        let mut inprocess: Vec<SourceRange> = sources.clone();

        for map in self.maps.iter() {
            let (
                mut new_inprocess, 
                mut new_destinations
            ) = map.transform(&inprocess);
            inprocess.clear();
            inprocess.append(&mut new_inprocess);
            destinations.append(&mut new_destinations);
        }

        destinations.append(&mut inprocess);
        destinations
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