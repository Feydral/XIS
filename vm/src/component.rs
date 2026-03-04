use std::ops::{Index, IndexMut};

use common::hardware;
use common::instruction::Instruction;

pub struct InstructionMemory {
    instructions: Vec<Instruction>,
}

impl InstructionMemory {
    pub fn new(mut instructions: Vec<Instruction>) -> Self {
        if instructions.len() > hardware::INSTRUCTION_MEM_SIZE as usize {
            instructions.truncate(hardware::INSTRUCTION_MEM_SIZE as usize);
        }
        Self { instructions }
    }
}

impl Index<usize> for InstructionMemory {
    type Output = Instruction;

    fn index(&self, index: usize) -> &Self::Output {
        self.instructions.get(index).unwrap_or(&Instruction::NoOperation)
    }
}


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


pub struct ArithmeticLogicUnit {
    zero_flag: bool,
    carry_flag: bool,
    overflow_flag: bool,
}

impl ArithmeticLogicUnit {
    pub fn new() -> Self {
        Self {
            zero_flag: false,
            carry_flag: false,
            overflow_flag: false,
        }
    }

    pub fn zero_flag(&self) -> bool {
        self.zero_flag
    }

    pub fn carry_flag(&self) -> bool {
        self.carry_flag
    }

    pub fn overflow_flag(&self) -> bool {
        self.overflow_flag
    }

    pub fn add(&mut self, a: u16, b: u16) -> u16 {
        let (result, carry) = a.overflowing_add(b);

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }
        if carry {
            self.carry_flag = true;
        }
        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }

    pub fn sub(&mut self, a: u16, b: u16) -> u16 {
        let (result, borrow) = a.overflowing_sub(b);

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }
        if borrow {
            self.carry_flag = true;
        }
        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }

    pub fn mul(&mut self, a: u16, b: u16) -> u16 {
        let (result, carry) = a.overflowing_mul(b);

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }
        if carry {
            self.carry_flag = true;
        }
        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }

    pub fn div(&mut self, a: u16, b: u16) -> u16 {
        let result = if b == 0 { 0 } else { a / b };

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }
        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }

    pub fn modulo(&mut self, a: u16, b: u16) -> u16 {
        let result = if b == 0 { 0 } else { a % b };

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }
        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }

    pub fn and(&mut self, a: u16, b: u16) -> u16 {
        let result = a & b;

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }
        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }

    pub fn nand(&mut self, a: u16, b: u16) -> u16 {
        let result = !(a & b);

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }
        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }

    pub fn or(&mut self, a: u16, b: u16) -> u16 {
        let result = a | b;

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }
        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }

    pub fn nor(&mut self, a: u16, b: u16) -> u16 {
        let result = !(a | b);

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }
        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }

    pub fn xor(&mut self, a: u16, b: u16) -> u16 {
        let result = a ^ b;

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }
        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }

    pub fn xnor(&mut self, a: u16, b: u16) -> u16 {
        let result = !(a ^ b);

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }
        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }

    pub fn not(&mut self, a: u16) -> u16 {
        let result = !a;

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }
        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }
    
    pub fn rsh(&mut self, a: u16) -> u16 {
        let result = a >> 1;

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }

        if (a & 0x0001) != 0 {
            self.carry_flag = true;
        }

        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }

    pub fn lsh(&mut self, a: u16) -> u16 {
        let result = a << 1;

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }

        if (a & 0x8000) != 0 {
            self.carry_flag = true;
        }

        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }

    pub fn rol(&mut self, a: u16, b: u16) -> u16 {
        let shift = (b & 0xF) as u32;
        let result = a.rotate_right(shift);

        self.zero_flag = false;
        self.carry_flag = false;
        self.overflow_flag = false;

        if result == 0 {
            self.zero_flag = true;
        }

        if shift > 0 {
            let shifted_out = (a >> (shift - 1)) & 1;
            if shifted_out == 1 {
                self.carry_flag = true;
            }
        }

        if (result & 0x8000) != 0 {
            self.overflow_flag = true;
        }

        result
    }
}


pub struct CallStack {
    stack: Vec<u16>,
}

impl CallStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(hardware::CALLSTACK_DEPTH as usize),
        }
    }

    pub fn push(&mut self, value: u16) {
        if self.stack.len() < hardware::CALLSTACK_DEPTH as usize {
            self.stack.push(value);
        }
    }

    pub fn pop(&mut self) -> u16 {
        match self.stack.pop() {
            Some(v) => v,
            None => 0,
        }
    }
}