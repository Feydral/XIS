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

    pub fn advance_clock() -> Result<(), RuntimeError> {
        Ok(())
    }
}