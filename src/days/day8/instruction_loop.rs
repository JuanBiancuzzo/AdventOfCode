use std::iter::Iterator;

use super::rl::RL;

pub struct InstructionLoop {
    pub instructions: Vec<RL>,
    current_position: usize,
}

impl InstructionLoop {
    pub fn new(instructions: Vec<RL>) -> InstructionLoop {
        InstructionLoop {
            instructions,
            current_position: 0
        }
    }
}

impl Iterator for InstructionLoop {
    type Item = RL;

    fn next(&mut self) -> Option<Self::Item> {
        match self.instructions.get(self.current_position) {
            Some(instruction) => {
                    self.current_position += 1;
                    Some(*instruction)
                },
            None => {
                self.current_position = 1;
                self.instructions.first().cloned()
            }
        }

    }
}