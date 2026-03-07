use common::instruction::Instruction;
use common::hardware::*;
use common::math::mathi;

fn encode(instruction: &Instruction) -> u32 {
    let mut bits: u32 = 0;

    bits |= (get_opcode(instruction) as u32) << 19;

    match instruction {
        Instruction::Add { reg_a, reg_b, reg_c }
        | Instruction::Subtract { reg_a, reg_b, reg_c }
        | Instruction::Multiply { reg_a, reg_b, reg_c }
        | Instruction::Divide { reg_a, reg_b, reg_c }
        | Instruction::Modulo { reg_a, reg_b, reg_c }
        | Instruction::BitwiseAnd { reg_a, reg_b, reg_c }
        | Instruction::BitwiseNand { reg_a, reg_b, reg_c }
        | Instruction::BitwiseOr { reg_a, reg_b, reg_c }
        | Instruction::BitwiseNor { reg_a, reg_b, reg_c }
        | Instruction::BitwiseXor { reg_a, reg_b, reg_c }
        | Instruction::BitwiseXnor { reg_a, reg_b, reg_c }
        | Instruction::Roll { reg_a, reg_b, reg_c } => {
            bits |= (*reg_a as u32 & 0b111) << 16;
            bits |= (*reg_b as u32 & 0b111) << 13;
            bits |= (*reg_c as u32 & 0b111) << 10;
        }

        Instruction::BitwiseNot { reg_a, reg_c }
        | Instruction::RightShift { reg_a, reg_c }
        | Instruction::LeftShift { reg_a, reg_c } => {
            bits |= (*reg_a as u32 & 0b111) << 16;
            bits |= (*reg_c as u32 & 0b111) << 10;
        }

        Instruction::LoadImmediate { reg_a, immediate }
        | Instruction::AddImmediate { reg_a, immediate }
        | Instruction::SubtractImmediate { reg_a, immediate }
        | Instruction::MultiplyImmediate { reg_a, immediate }
        | Instruction::DivideImmediate { reg_a, immediate } => {
            bits |= (*reg_a as u32 & 0b111) << 16;
            bits |= *immediate as u32 & 0xFFFF;
        }

        Instruction::Jump { address }
        | Instruction::Call { address } => {
            bits |= (*address as u32 & 0xFFF) << 5;
        }

        Instruction::Branch { condition_flag, address } => {
            bits |= (*condition_flag as u32 & 0b11) << 17;
            bits |= (*address as u32 & 0xFFF) << 5;
        }

        Instruction::MemoryLoad { reg_a, reg_b, offset }
        | Instruction::MemoryStore { reg_a, reg_b, offset } => {
            bits |= (*reg_a as u32 & 0b111) << 16;
            bits |= (*reg_b as u32 & 0b111) << 13;
            bits |= *offset as u32 & 0xFF;
        }

        Instruction::Draw { reg_x, reg_y, reg_rgb } => {
            bits |= (*reg_x as u32 & 0b111) << 16;
            bits |= (*reg_y as u32 & 0b111) << 13;
            bits |= (*reg_rgb as u32 & 0b111) << 10;
        }

        Instruction::ControllerPad { reg_c }
        | Instruction::RandomNumberGenerator { reg_c } => {
            bits |= (*reg_c as u32 & 0b111) << 16;
        }

        Instruction::NoOperation
        | Instruction::Halt
        | Instruction::Return
        | Instruction::PushBuffer => {}
    }

    bits

}
    
fn get_opcode(instruction: &Instruction) -> u8 {
    match instruction {
        Instruction::NoOperation                  => OPCODE_NOP,
        Instruction::Halt                         => OPCODE_HLT,
        Instruction::Add { .. }                   => OPCODE_ADD,
        Instruction::Subtract { .. }              => OPCODE_SUB,
        Instruction::Multiply { .. }              => OPCODE_MUL,
        Instruction::Divide { .. }                => OPCODE_DIV,
        Instruction::Modulo { .. }                => OPCODE_REM,
        Instruction::BitwiseAnd { .. }            => OPCODE_AND,
        Instruction::BitwiseNand { .. }           => OPCODE_NAND,
        Instruction::BitwiseOr { .. }             => OPCODE_OR,
        Instruction::BitwiseNor { .. }            => OPCODE_NOR,
        Instruction::BitwiseXor { .. }            => OPCODE_XOR,
        Instruction::BitwiseXnor { .. }           => OPCODE_XNOR,
        Instruction::BitwiseNot { .. }            => OPCODE_NOT,
        Instruction::RightShift { .. }            => OPCODE_RSH,
        Instruction::LeftShift { .. }             => OPCODE_LSH,
        Instruction::Roll { .. }                  => OPCODE_ROL,
        Instruction::LoadImmediate { .. }         => OPCODE_LDI,
        Instruction::AddImmediate { .. }          => OPCODE_ADDI,
        Instruction::SubtractImmediate { .. }     => OPCODE_SUBI,
        Instruction::MultiplyImmediate { .. }     => OPCODE_MULI,
        Instruction::DivideImmediate { .. }       => OPCODE_DIVI,
        Instruction::Jump { .. }                  => OPCODE_JMP,
        Instruction::Branch { .. }                => OPCODE_BRH,
        Instruction::Call { .. }                  => OPCODE_CALL,
        Instruction::Return                       => OPCODE_RET,
        Instruction::MemoryLoad { .. }            => OPCODE_MLD,
        Instruction::MemoryStore { .. }           => OPCODE_MSTR,
        Instruction::Draw { .. }                  => OPCODE_DRW,
        Instruction::PushBuffer                   => OPCODE_PSHB,
        Instruction::ControllerPad { .. }         => OPCODE_PAD,
        Instruction::RandomNumberGenerator { .. } => OPCODE_RNG,
    }
}

pub fn to_binary_string(instruction: &Instruction) -> String {
    mathi::int_to_binary_string(encode(instruction) as u64, 24)
}

pub fn to_hexadecimal_string(instruction: &Instruction) -> String {
    mathi::int_to_hexadecimal_string(encode(instruction) as u64, 6)
}

pub fn to_assembly_string(instruction: &Instruction) -> String {
    match instruction {
        Instruction::NoOperation { } => "NOP".to_string(),
        Instruction::Halt { } => "HLT".to_string(),
        Instruction::Add { reg_a, reg_b, reg_c } => format!("ADD r{} r{} r{}", reg_a, reg_b, reg_c),
        Instruction::Subtract { reg_a, reg_b, reg_c } => format!("SUB r{} r{} r{}", reg_a, reg_b, reg_c),
        Instruction::Multiply { reg_a, reg_b, reg_c } => format!("MUL r{} r{} r{}", reg_a, reg_b, reg_c),
        Instruction::Divide { reg_a, reg_b, reg_c } => format!("DIV r{} r{} r{}", reg_a, reg_b, reg_c),
        Instruction::Modulo { reg_a, reg_b, reg_c } => format!("REM r{} r{} r{}", reg_a, reg_b, reg_c),
        Instruction::BitwiseAnd { reg_a, reg_b, reg_c } => format!("AND r{} r{} r{}", reg_a, reg_b, reg_c),
        Instruction::BitwiseNand { reg_a, reg_b, reg_c } => format!("NAND r{} r{} r{}", reg_a, reg_b, reg_c),
        Instruction::BitwiseOr { reg_a, reg_b, reg_c } => format!("OR r{} r{} r{}", reg_a, reg_b, reg_c),
        Instruction::BitwiseNor { reg_a, reg_b, reg_c } => format!("NOR r{} r{} r{}", reg_a, reg_b, reg_c),
        Instruction::BitwiseXor { reg_a, reg_b, reg_c } => format!("XOR r{} r{} r{}", reg_a, reg_b, reg_c),
        Instruction::BitwiseXnor { reg_a, reg_b, reg_c } => format!("XNOR r{} r{} r{}", reg_a, reg_b, reg_c),
        Instruction::BitwiseNot { reg_a, reg_c } => format!("NOT r{} r{}", reg_a, reg_c),
        Instruction::RightShift { reg_a, reg_c } => format!("RSH r{} r{}", reg_a, reg_c),
        Instruction::LeftShift { reg_a, reg_c } => format!("LSH r{} r{}", reg_a, reg_c),
        Instruction::Roll { reg_a, reg_b, reg_c } => format!("ROL r{} r{} r{}", reg_a, reg_b, reg_c),
        Instruction::LoadImmediate { reg_a, immediate } => format!("LDI r{} {}", reg_a, immediate),
        Instruction::AddImmediate { reg_a, immediate } => format!("ADDI r{} {}", reg_a, immediate),
        Instruction::SubtractImmediate { reg_a, immediate } => format!("SUBI r{} {}", reg_a, immediate),
        Instruction::MultiplyImmediate { reg_a, immediate } => format!("MULI r{} {}", reg_a, immediate),
        Instruction::DivideImmediate { reg_a, immediate } => format!("DIVI r{} {}", reg_a, immediate),
        Instruction::Jump { address } => format!("JMP {}", address),
        Instruction::Branch { condition_flag, address } => format!("BRH {} {}", condition_flag, address),
        Instruction::Call { address } => format!("CALL {}", address),
        Instruction::Return => "RET".to_string(),
        Instruction::MemoryLoad { reg_a, reg_b, offset } => format!("MLD r{} r{} {}", reg_a, reg_b, offset),
        Instruction::MemoryStore { reg_a, reg_b, offset } => format!("MSTR r{} r{} {}", reg_a, reg_b, offset),
        Instruction::Draw { reg_x, reg_y, reg_rgb } => format!("DRW r{} r{} r{}", reg_x, reg_y, reg_rgb),
        Instruction::PushBuffer => "PSHB".to_string(),
        Instruction::ControllerPad { reg_c } => format!("PAD r{}", reg_c),
        Instruction::RandomNumberGenerator { reg_c } => format!("RNG r{}", reg_c),
    }
}   