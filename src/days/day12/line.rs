use std::convert::TryFrom;

use super::operation::Operation;

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    operations: Vec<Operation>,
    values: Vec<u64>,
}

impl Line {
    pub fn new(operations: Vec<Operation>, values: Vec<u64>) -> Self {
        Self {
            operations,
            values,
        }
    }

    pub fn multiply(&self, times: usize) -> Self {
        let mut operations: Vec<Operation> = vec![];
        let mut values: Vec<u64> = vec![];

        for _ in 0..times {
            operations.append(&mut self.operations.clone());
            values.append(&mut self.values.clone());
        }

        Self::new(operations, values)
    }

    // ?###???????? 3,2,1 como máximo se tiene 2^9 = 512 combinaciones, solo 10 son validas
    // ?#?#?#?#?#?#?#? 1,3,1,6 como máximo se tiene 2^8 = 256 combinaciones, solo una es valida
    pub fn get_possible_arrangements(&self) -> u64 {
        Self::get_possible_arrangments_sep(&mut self.operations.clone(), &self.values)
    }

    fn get_possible_arrangments_sep(operations: &mut Vec<Operation>, values: &Vec<u64>) -> u64 {
        if let Some(valid_arrangment) = Self::valid_arrangment(operations, values) {
            return if valid_arrangment { 1 } else { 0 };
        }

        let pos_operation_unknonw = operations.iter()
            .enumerate()
            .filter_map(|(i, operation)| if *operation == Operation::Unknwon { Some(i) } else { None })
            .reduce(|a, b| if a < b { a } else { b });
    
        match pos_operation_unknonw {
            Some(pos_operation_unknonw) => {
                operations[pos_operation_unknonw] = Operation::Operational;
                let count_operational = Self::get_possible_arrangments_sep(operations, values);
                
                operations[pos_operation_unknonw] = Operation::Damage;
                let count_damage = Self::get_possible_arrangments_sep(operations, values);

                operations[pos_operation_unknonw] = Operation::Unknwon;
                count_operational + count_damage
            },
            None => 0,
        }
    }

    fn valid_arrangment(operations: &Vec<Operation>, values: &Vec<u64>) -> Option<bool> {
        if operations.iter().any(|operation| *operation == Operation::Unknwon) {
            return None;
        }

        Some(Self::get_damage_operations_count(operations) == *values)
    }

    fn get_damage_operations_count(operations: &Vec<Operation>) -> Vec<u64> {
        let mut operations: Vec<&Operation> = operations.iter().rev().collect();
        let mut operations_count: Vec<u64> = Vec::new();

        
        let mut posible_operation = operations.pop();
        while let Some(operation) = posible_operation {
            if *operation != Operation::Damage {
                posible_operation = operations.pop();
                continue;
            }

            posible_operation = None;
            let mut count: u64 = 1;

            while let Some(operation_next) = operations.pop() {
                if operation == operation_next {
                    count += 1;
                } else {
                    posible_operation = Some(operation_next);
                    break;
                }
            } 

            operations_count.push(count);
        }

        operations_count
    }

    fn get_operations_count(operations: &Vec<Operation>) -> Vec<(Operation, u64)> {
        let mut operations: Vec<&Operation> = operations.iter().rev().collect();

        let mut operations_count: Vec<(Operation, u64)> = Vec::new();
        let mut posible_operation = operations.pop();

        while let Some(operation) = posible_operation {
            posible_operation = None;
            let mut count: u64 = 1;

            while let Some(operation_next) = operations.pop() {
                if operation == operation_next {
                    count += 1;
                } else {
                    posible_operation = Some(operation_next);
                    break;
                }
            } 

            operations_count.push((*operation, count));
        }

        operations_count
    }
}

impl<'t> TryFrom<&'t str> for Line {
    type Error = ();

    fn try_from(line: &'t str) -> Result<Self, Self::Error> {
        let (operations, values) = line.trim().split_once(' ').ok_or(())?;
            
        let operations = operations.chars()
            .filter_map(|c| Operation::try_from(c).ok())
            .collect::<Vec<Operation>>();

        let values = values.split(',')
            .filter_map(|value| value.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        
        Ok(Self {
            operations,
            values,
        })
    }
}

#[cfg(test)]
mod test_line {
    use super::*;

    #[test]
    fn operation_count_correctly() {
        let operations = vec![
            Operation::Unknwon,
            Operation::Unknwon,
            Operation::Unknwon,
            Operation::Operational,
            Operation::Damage,
            Operation::Damage,
            Operation::Damage,
        ];

        let expected_operations_count = vec![
            (Operation::Unknwon, 3),
            (Operation::Operational, 1),
            (Operation::Damage, 3),
        ];
        let operations_count = Line::get_operations_count(&mut operations.clone());

        assert_eq!(operations_count, expected_operations_count);
    }

    #[test] 
    fn arrangment_is_valid() {
        let operations = vec![
            Operation::Damage,
            Operation::Operational,
            Operation::Damage,
            Operation::Operational,
            Operation::Damage,
            Operation::Damage,
            Operation::Damage,
        ];
        let values = vec![1, 1, 3];

        assert!(Line::valid_arrangment(&operations, &values).unwrap());
    }

    #[test] 
    fn arrangment_is_invalid() {
        let operations = vec![
            Operation::Damage,
            Operation::Operational,
            Operation::Operational,
            Operation::Damage,
            Operation::Operational,
            Operation::Damage,
            Operation::Damage,
            Operation::Damage,
        ];
        let values = vec![2, 1, 3];

        assert!(!Line::valid_arrangment(&operations, &values).unwrap());
    }

    #[test] 
    fn arrangment_is_partially_valid() {
        let operations = vec![
            Operation::Damage,
            Operation::Damage,
            Operation::Operational,
            Operation::Operational,
            Operation::Damage,
            Operation::Operational
        ];
        let values = vec![2, 1, 3];

        assert!(!Line::valid_arrangment(&operations, &values).unwrap());
    }

    #[test] 
    fn arrangment_is_incomplete() {
        let operations = vec![
            Operation::Unknwon,
            Operation::Unknwon,
            Operation::Unknwon,
            Operation::Operational,
            Operation::Damage,
            Operation::Damage,
            Operation::Damage,
        ];
        let values = vec![1, 1, 3];

        let valid_arrangment = Line::valid_arrangment(&operations, &values);

        assert_eq!(valid_arrangment, None);
    }

    #[test]
    fn one_possible_arrangment() {
        let line = Line::try_from("???.### 1,1,3").unwrap();
        assert_eq!(line.get_possible_arrangements(), 1);
    }

    #[test]
    fn ten_possible_arrangment() {
        let line = Line::try_from("?###???????? 3,2,1").unwrap();
        assert_eq!(line.get_possible_arrangements(), 10);
    }
}