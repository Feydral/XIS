use crate::{errors::VirtualProgramError, instruction::{self, Instruction}};

pub struct VirtualProgram {
    pub carry_flag: bool,
    pub zero_flag: bool,
    pub overflow_flag: bool,

    pub memory: [u16; 1024],

    pub instructions: Vec<Instruction>,
}

impl VirtualProgram {
    pub fn new(mut instructions: Vec<Instruction>) -> Result<Self, VirtualProgramError> {
        if instructions.len() >= 4096 {
            instructions.truncate(4096);
        }

        Ok(Self {
            carry_flag: false,
            zero_flag: false,
            overflow_flag: false,

            memory: [0_u16; 1024],

            instructions,
        })
    }
}