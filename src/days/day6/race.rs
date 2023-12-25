
#[derive(Debug, Clone, Copy)]
pub struct Race {
    pub time: u64,
    pub distance: u64,
}

impl Race {
    pub fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    /*
        speed = time_holded
        distance = speed * (time - time_holded)
        distance = time_holded * time - time_holded ** 2
        0 = time_holded ** 2 - time_holded * time + distance
        time_holded_1 = (time + sqrt(time ** 2 - 4 * distance)) / 2
        time_holded_2 = (time - sqrt(time ** 2 - 4 * distance)) / 2
     */
    pub fn get_posible_holding_time(&self) -> Vec<u64> {
        let (minimun_speed, maximun_speed) = Self::resolvente(1.0, -(self.time as f64), self.distance as f64).unwrap();
        let minimun_speed = (minimun_speed + 0.001).ceil() as u64;
        let maximun_speed = (maximun_speed - 0.001).floor() as u64;        
        
        (minimun_speed..=maximun_speed).collect::<Vec<u64>>()
    }

    pub fn get_number_of_posible_holding_time(&self) -> u64 {
        let (minimun_speed, maximun_speed) = Self::resolvente(1.0, -(self.time as f64), self.distance as f64).unwrap();
        let minimun_speed = (minimun_speed + 0.001).ceil() as u64;
        let maximun_speed = (maximun_speed - 0.001).floor() as u64; 

        maximun_speed - minimun_speed + 1
    }

    fn resolvente(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
        let discriminant = b.powf(2.0) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let x1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let x2 = (-b + discriminant.sqrt()) / (2.0 * a);
        
        Some((x1, x2))
    }
}

#[cfg(test)]
mod prueba_race {
    use super::*;

    #[test]
    fn primer_ejemplo() {
        let race = Race::new(7, 9);
        assert_eq!(race.get_posible_holding_time(), vec![2, 3, 4, 5]);
        assert_eq!(race.get_number_of_posible_holding_time(), 4);
    }

    #[test]
    fn segundo_ejemplo() {
        let race = Race::new(15, 40);

        assert_eq!(race.get_posible_holding_time(), vec![4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(race.get_number_of_posible_holding_time(), 8);
    }

    #[test]
    fn tercer_ejemplo() {
        let race = Race::new(30, 200);

        assert_eq!(race.get_posible_holding_time(), vec![11, 12, 13, 14, 15, 16, 17, 18, 19]);
        assert_eq!(race.get_number_of_posible_holding_time(), 9);
    }
}