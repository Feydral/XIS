use crate::instruction::Instruction;

pub struct VirtualProgram {
    pub overflow_flag: bool,
    pub carry_flag: bool,
    pub zero_flag: bool,

    pub instructions: Vec<Instruction>,
}