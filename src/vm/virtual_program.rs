use crate::{errors::RuntimeError, hardware, instruction::{self, Instruction}};

pub struct VirtualProgram {
    pub carry_flag: bool,
    pub zero_flag: bool,
    pub overflow_flag: bool,

    pub memory: [u16; 1024],

    pub instructions: Vec<Instruction>,

    pub pc: u16,
}

impl VirtualProgram {
    pub fn new(mut instructions: Vec<Instruction>) -> Result<Self, RuntimeError> {
        if instructions.len() >= hardware::INSTRUCTION_MEM_SIZE as usize {
            instructions.truncate(hardware::INSTRUCTION_MEM_SIZE as usize);
        }

        Ok(Self {
            carry_flag: false,
            zero_flag: false,
            overflow_flag: false,

            memory: [0_u16; 1024],

            instructions,

            pc: 0_u16,
        })
    }

    pub fn advance_clock(&mut self) -> Result<(), RuntimeError> {
        if self.pc >= hardware::INSTRUCTION_MEM_SIZE as u16 {
            self.pc = 0;
        }

        let instruction = if (self.pc as usize) < self.instructions.len() {
            self.instructions[self.pc as usize]
        } else {
            Instruction::NoOperation
        };

        self.execute_instruction(instruction)?;
        self.pc += 1;

        Ok(())
}

    pub fn execute_instruction(&self, instruction: Instruction) -> Result<(), RuntimeError> {
        todo!()
    }
}