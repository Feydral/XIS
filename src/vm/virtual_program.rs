use crate::instruction::Instruction;

pub struct VirtualProgram {
    pub carry_flag: bool,
    pub zero_flag: bool,

    pub memory: [u16; 1024],

    pub instructions: Vec<Instruction>,
}