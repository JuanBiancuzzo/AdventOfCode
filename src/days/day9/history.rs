use std::convert::From;

pub struct History {
    values: Vec<i64>,
}

impl History {
    pub fn new(values: Vec<i64>) -> History {
        History { values }
    }

    pub fn predict_next_value(&self) -> i64 {
        let diferencias = self.values.windows(2)
            .map(|slice| match slice {
                &[a, b] => b - a,
                _ => panic!("Invalid slice"),
            })
            .collect::<Vec<i64>>();

        if diferencias.iter().all(|diferencia| *diferencia == 0) {
            match self.values.last() {
                Some(&last) => last,
                None => 0,
            }
        } else {
            let valor_menor = History::new(diferencias).predict_next_value();

            match self.values.last() {
                Some(&last) => last + valor_menor,
                _ => 0,
            }
        }
    }

    pub fn predict_previous_value(&self) -> i64 {
        let diferencias = self.values.windows(2)
            .map(|slice| match slice {
                &[a, b] => b - a,
                _ => panic!("Invalid slice"),
            })
            .collect::<Vec<i64>>();

        if diferencias.iter().all(|diferencia| *diferencia == 0) {
            match self.values.first() {
                Some(&first) => first,
                None => 0,
            }
        } else {
            let valor_menor = History::new(diferencias).predict_previous_value();
            match self.values.first() {
                Some(&first) => first - valor_menor,
                _ => 0,
            }
        }
    }
}

impl<'t> From<&'t str> for History {
    fn from(s: &'t str) -> Self {
        let values: Vec<i64> = s
            .split(" ")
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();

        History::new(values)
    }
}

#[cfg(test)]
mod pruebas_historias {
    use super::*;

    #[test]
    fn ejemplo1_ultimo_valor() {
        let history = History::new(vec![0, 3, 6, 9, 12, 15]);

        let resultado = history.predict_next_value();

        assert_eq!(resultado, 18);
    }

    #[test]
    fn ejemplo2_ultimo_valor() {
        let history = History::new(vec![1, 3, 6, 10, 15, 21]);

        let resultado = history.predict_next_value();

        assert_eq!(resultado, 28);
    }

    #[test]
    fn ejemplo3_ultimo_valor() {
        let history = History::new(vec![10, 13, 16, 21, 30, 45]);

        let resultado = history.predict_next_value();

        assert_eq!(resultado, 68);
    }

    #[test]
    fn ejemplo1_primero_valor() {
        let history = History::new(vec![0, 3, 6, 9, 12, 15]);

        let resultado = history.predict_previous_value();

        assert_eq!(resultado, -3);
    }

    #[test]
    fn ejemplo2_primero_valor() {
        let history = History::new(vec![1, 3, 6, 10, 15, 21]);

        let resultado = history.predict_previous_value();

        assert_eq!(resultado, 0);
    }

    #[test]
    fn ejemplo3_primero_valor() {
        let history = History::new(vec![10, 13, 16, 21, 30, 45]);

        let resultado = history.predict_previous_value();

        assert_eq!(resultado, 5);
    }
}