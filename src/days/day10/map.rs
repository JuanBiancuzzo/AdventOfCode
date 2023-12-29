use std::convert::From;

pub struct Map<const N: usize, const M: usize, E: Default + From<char> + Copy> {
    map: [[E; N]; M],
}

impl<const N: usize, const M: usize, E: Default + From<char> + Copy> Map<N, M, E> {
    pub fn get_element(&self, position: (i32, i32)) -> Option<E> {
        if position.0 >= N as i32 || position.1 >= M as i32 || position.0 < 0 || position.1 < 0 {
            return None;
        }

        Some(self.map[position.1 as usize][position.0 as usize])
    }
}

impl<const N: usize, const M: usize, E: Default + From<char> + Copy> From<String> for Map<N, M, E> {
    fn from(value: String) -> Self {
        let mut map = [[E::default(); N]; M];

        for (i, linea) in value.lines().enumerate() {
            if i >= M { break; }
            for (j, caracter) in linea.trim().chars().enumerate() {
                if j >= N { break; }
                
                match E::try_from(caracter) {
                    Ok(element) => map[i][j] = element,
                    Err(_) => panic!("Caracter no reconocido: {}", caracter),
                }
            }
        }

        Self { map }
    }
}