use crate::{instruction::{self, Instruction}, math::numerics::byte3::Byte3};

pub fn generate_instruction_code(instruction: Instruction) -> String {
    todo!()
}

pub fn generate_assembly_code(instruction: Instruction) -> String {
    todo!()
}

pub fn generate_hex_code(instruction: Instruction) -> String {
    todo!()
}

pub fn generate_binary_code(instruction: Instruction) -> String {
    todo!()
}

pub fn get_opcode(instruction: &Instruction) -> u8 {
    match instruction {
        Instruction::NoOperation                  => 0b00000,
        Instruction::Halt                         => 0b00001,
        Instruction::Add { .. }                   => 0b00010,
        Instruction::Subtract { .. }              => 0b00011,
        Instruction::Multiply { .. }              => 0b00100,
        Instruction::Divide { .. }                => 0b00101,
        Instruction::Modulo { .. }                => 0b00110,
        Instruction::BitwiseAnd { .. }            => 0b00111,
        Instruction::BitwiseNand { .. }           => 0b01000,
        Instruction::BitwiseOr { .. }             => 0b01001,
        Instruction::BitwiseNor { .. }            => 0b01010,
        Instruction::BitwiseXor { .. }            => 0b01011,
        Instruction::BitwiseXnor { .. }           => 0b01100,
        Instruction::BitwiseNot { .. }            => 0b01101,
        Instruction::RightShift { .. }            => 0b01110,
        Instruction::LeftShift { .. }             => 0b01111,
        Instruction::Shift { .. }                 => 0b10000,
        Instruction::LoadImmediate { .. }         => 0b10001,
        Instruction::AddImmediate { .. }          => 0b10010,
        Instruction::SubtractImmediate { .. }     => 0b10011,
        Instruction::MultiplyImmediate { .. }     => 0b10100,
        Instruction::DivideImmediate { .. }       => 0b10101,
        Instruction::Jump { .. }                  => 0b10110,
        Instruction::Branch { .. }                => 0b10111,
        Instruction::Call { .. }                  => 0b11000,
        Instruction::Return                       => 0b11001,
        Instruction::MemoryLoad { .. }            => 0b11010,
        Instruction::MemoryStore { .. }           => 0b11011,
        Instruction::Draw { .. }                  => 0b11100,
        Instruction::PushBuffer                   => 0b11101,
        Instruction::ControllerPad { .. }         => 0b11110,
        Instruction::RandomNumberGenerator { .. } => 0b11111,
    }
}