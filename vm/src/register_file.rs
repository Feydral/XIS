use std::ops::{Index, IndexMut};

pub struct RegisterFile {
    registers: [u16; 8],
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
        }
    }
}

impl Index<usize> for RegisterFile {
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &0,
            1..=7 => &self.registers[index],
            _ => &0,
        }
    }
}

impl IndexMut<usize> for RegisterFile {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.registers[0],
            1..=7 => &mut self.registers[index],
            _ => &mut self.registers[0],
        }
    }
}