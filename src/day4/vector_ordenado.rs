pub trait InsersionOrdenada<T: PartialEq + PartialOrd> {
    /// Se asume que el vector está ordenado
    fn insertar_ordenadamente(&mut self, item: T);
}

impl<T: PartialEq + PartialOrd> InsersionOrdenada<T> for Vec<T> {
    fn insertar_ordenadamente(&mut self, item: T) {
        let mut idx = 0;
        let mut encontrado: bool = false;
        // 1 2 3 5 y tenemos 4
        // 0 1 2 3 y deberiamos insertarlo en el index 3
        for (i, numero) in self.iter().enumerate() {
            if item > *numero {
                idx = i + 1;
            } else {
                encontrado = true;
                break;
            }
        }

        if !encontrado {
            self.push(item);
        } else {
            self.insert(idx, item);
        }
    }
}

#[cfg(test)]
mod pruebas_vector_ordenado {
    use super::*;

    #[test]
    fn vector_con_5_elementos() {
        let esperado: Vec<u32> = vec![17, 41, 48, 83, 86];
        
        let vector: Vec<u32> = vec![41, 48, 83, 86, 17];
        let mut ordenados: Vec<u32> = vec![];
        
        for numero in vector {
            ordenados.insertar_ordenadamente(numero);
        }        
        
        assert_eq!(ordenados, esperado);
    }

    #[test]
    fn vector_con_8_elementos() {
        let esperado: Vec<u32> = vec![6, 9, 17, 31, 48, 53, 83, 86];
        
        let vector: Vec<u32> = vec![83, 86, 6, 31, 17, 9, 48, 53];
        let mut ordenados: Vec<u32> = vec![];

        for numero in vector {
            ordenados.insertar_ordenadamente(numero);
        }        

        assert_eq!(ordenados, esperado);
    }

    #[test]
    fn vector_con_elementos_repetidos() {
        let esperado: Vec<u32> = vec![6, 6, 9, 17, 48, 53, 83, 86];
        
        let vector: Vec<u32> = vec![83, 86, 6, 17, 9, 48, 6, 53];
        let mut ordenados: Vec<u32> = vec![];

        for numero in vector {
            ordenados.insertar_ordenadamente(numero);
        }        

        assert_eq!(ordenados, esperado);
    }

    #[test]
    fn vector_ya_ordenado() {
        let esperado: Vec<u32> = vec![6, 9, 17, 31, 48, 53, 83, 86];
        
        let vector: Vec<u32> = vec![6, 9, 17, 31, 48, 53, 83, 86];
        let mut ordenados: Vec<u32> = vec![];

        for numero in vector {
            ordenados.insertar_ordenadamente(numero);
        }        

        assert_eq!(ordenados, esperado);
    }
}