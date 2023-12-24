use std::convert::TryFrom;

use super::source_range::SourceRange;

type SourceInprocess = Vec<SourceRange>;
type Destination = Vec<SourceRange>;

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    destination_start: u32,
    source_start: u32,
    range: u32,
}

impl Map {
    pub fn new(destination_start: u32, source_start: u32, range: u32) -> Self {
        Self {
            destination_start,
            source_start,
            range,
        }
    }

    pub fn transform(&self, sources: &Vec<SourceRange>) -> (SourceInprocess, Destination) {
        let mut inprocess: SourceInprocess = vec![];
        let mut destinations: Destination = vec![];

        for source in sources.iter() {
            let (
                mut new_inprocess, 
                mut new_destinations
            ) = self.transform_source_range(source);

            inprocess.append(&mut new_inprocess);
            destinations.append(&mut new_destinations);
        }

        (inprocess, destinations)
    }

    
    fn transform_source_range(&self, source: &SourceRange) -> (SourceInprocess, Destination) {
        let mut inprocess: SourceInprocess = vec![];
        let mut destinations: Destination = vec![];

        if source.start < self.source_start {
            if source.range < self.source_start - source.start {
                return (vec![*source], vec![]);
            } // source.range > self.source_start - source.start = inprocess_range

            let inprocess_range = self.source_start - source.start;
            if let Some(source_range) = SourceRange::new(source.start, inprocess_range) {
                inprocess.push(source_range);
            }

            let destination_range = std::cmp::min(source.range - inprocess_range, self.range);
            if let Some(source_range) = SourceRange::new(self.destination_start, destination_range) {
                destinations.push(source_range); 
            }

            if source.range - inprocess_range > self.range {
                let inprocess_range = source.range - inprocess_range - self.range;
                if let Some(source_range) = SourceRange::new(self.source_start + self.range, inprocess_range) {
                    inprocess.push(source_range); 
                }
            }
        } else {
            if self.range <= source.start - self.source_start {
                return (vec![*source], vec![]);
            }

            let difference = source.start - self.source_start;  // 5
            let destination_range = std::cmp::min(source.range, self.range - difference); // min(5, 20) = 5
            
            if let Some(source_range) = SourceRange::new(self.destination_start + difference, destination_range) {
                destinations.push(source_range);  
            }

            if source.range > self.range - difference {
                let inprocess_range = source.range - (self.range - difference);
                if let Some(source_range) = SourceRange::new(source.start - difference + self.range, inprocess_range) {
                    inprocess.push(source_range); 
                }
            }
        }

        (inprocess, destinations)
    }
}

impl TryFrom<&str> for Map {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let numeros = value.split(" ")
            .filter_map(|numero| numero.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let destination_start = match numeros.get(0) {
            Some(numero) => *numero,
            None => return Err(()),
        };

        let source_start = match numeros.get(1) {
            Some(numero) => *numero,
            None => return Err(()),
        };

        let range = match numeros.get(2) {
            Some(numero) => *numero,
            None => return Err(()),
        };

        Ok(Self::new(destination_start, source_start, range))
    }
}

#[cfg(test)]
mod pruebas_interseccion {
    use super::*;

    #[test]
    fn source_range_menor_que_map() {
        let source_range = SourceRange::new(0, 5).unwrap();
        let map = Map::new(0, 10, 5);

        let (inprocess, destinations) = map.transform_source_range(&source_range);

        assert_eq!(inprocess, vec![source_range]);
        assert_eq!(destinations, vec![]);
    }

    #[test]
    fn map_menor_que_source_range() {
        let source_range = SourceRange::new(10, 5).unwrap();
        let map = Map::new(0, 0, 5);

        let (inprocess, destinations) = map.transform_source_range(&source_range);

        assert_eq!(inprocess, vec![source_range]);
        assert_eq!(destinations, vec![]);
    }

    #[test]
    fn source_range_intersecta_completametamente_map() {
        let source_range = SourceRange::new(15, 5).unwrap();
        let map = Map::new(0, 10, 25);

        let (inprocess, destinations) = map.transform_source_range(&source_range);

        let destino_esperado = SourceRange::new(5, 5).unwrap();

        assert_eq!(inprocess, vec![]);
        assert_eq!(destinations, vec![destino_esperado]);
    }

    #[test]
    fn source_range_intersecta_perfectamente_sin_sobra_map() {
        let source_range = SourceRange::new(15, 15).unwrap();
        let map = Map::new(0, 10, 20);

        let (inprocess, destinations) = map.transform_source_range(&source_range);

        let destino_esperado = SourceRange::new(5, 15).unwrap();

        assert_eq!(inprocess, vec![]);
        assert_eq!(destinations, vec![destino_esperado]);
    }

    #[test]
    fn map_intersecta_completametamente_source_range() {
        let source_range = SourceRange::new(10, 25).unwrap();
        let map = Map::new(0, 15, 5);

        let (inprocess, destinations) = map.transform_source_range(&source_range);

        let inprocess_inferior_esperado = SourceRange::new(10, 5).unwrap();
        let inprocess_superior_esperado = SourceRange::new(20, 5).unwrap();
        let destino_esperado = SourceRange::new(0, 5).unwrap();

        assert_eq!(inprocess, vec![inprocess_inferior_esperado, inprocess_superior_esperado]);
        assert_eq!(destinations, vec![destino_esperado]);
    }

    #[test]
    fn map_intersecta_perfectamente_sin_sobra_source_range() {
        let source_range = SourceRange::new(10, 15).unwrap();
        let map = Map::new(0, 15, 10);

        let (inprocess, destinations) = map.transform_source_range(&source_range);

        let inprocess_esperado = SourceRange::new(10, 5).unwrap();
        let destino_esperado = SourceRange::new(0, 10).unwrap();

        assert_eq!(inprocess, vec![inprocess_esperado]);
        assert_eq!(destinations, vec![destino_esperado]);
    }

    #[test]
    fn map_intersecta_parcialmente_source_range() {
        let source_range = SourceRange::new(10, 10).unwrap();
        let map = Map::new(0, 15, 10);

        let (inprocess, destinations) = map.transform_source_range(&source_range);

        let inprocess_esperado = SourceRange::new(10, 5).unwrap();
        let destino_esperado = SourceRange::new(0, 5).unwrap();

        assert_eq!(inprocess, vec![inprocess_esperado]);
        assert_eq!(destinations, vec![destino_esperado]);
    }

    #[test]
    fn map_source_range_empiezan_mismo_valor_map_cubre_completamente() {
        let source_range = SourceRange::new(15, 10).unwrap();
        let map = Map::new(0, 15, 15);

        let (inprocess, destinations) = map.transform_source_range(&source_range);

        let destino_esperado = SourceRange::new(0, 10).unwrap();

        assert_eq!(inprocess, vec![]);
        assert_eq!(destinations, vec![destino_esperado]);
    }

    #[test]
    fn map_source_range_empiezan_mismo_valor_source_range_cubre_completamente() {
        let source_range = SourceRange::new(15, 15).unwrap();
        let map = Map::new(0, 15, 10);

        let (inprocess, destinations) = map.transform_source_range(&source_range);

        let inprocess_esperado = SourceRange::new(25, 5).unwrap();
        let destino_esperado = SourceRange::new(0, 10).unwrap();

        assert_eq!(inprocess, vec![inprocess_esperado]);
        assert_eq!(destinations, vec![destino_esperado]);
    }

    #[test] 
    fn source_range_con_rango_uno_mapeado() {
        let source_range = SourceRange::new(20, 1).unwrap();
        let map = Map::new(0, 15, 10);

        let (inprocess, destinations) = map.transform_source_range(&source_range);

        let destino_esperado = SourceRange::new(5, 1).unwrap();

        assert_eq!(inprocess, vec![]);
        assert_eq!(destinations, vec![destino_esperado]);
    }

    #[test] 
    fn source_range_con_rango_uno_y_empieza_por_debajo_queda_en_proceso() {
        let source_range = SourceRange::new(10, 1).unwrap();
        let map = Map::new(0, 15, 10);

        let (inprocess, destinations) = map.transform_source_range(&source_range);

        let inprocess_esperado = SourceRange::new(10, 1).unwrap();

        assert_eq!(inprocess, vec![inprocess_esperado]);
        assert_eq!(destinations, vec![]);
    }

    #[test] 
    fn source_range_con_rango_uno_y_empieza_por_encima_queda_en_proceso() {
        let source_range = SourceRange::new(30, 1).unwrap();
        let map = Map::new(0, 15, 10);

        let (inprocess, destinations) = map.transform_source_range(&source_range);

        let inprocess_esperado = SourceRange::new(30, 1).unwrap();

        assert_eq!(inprocess, vec![inprocess_esperado]);
        assert_eq!(destinations, vec![]);
    }
}