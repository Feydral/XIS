use std::error::Error;

use crate::errors::BinaryDecodeError;
use crate::instruction::Instruction;
use crate::hardware::*;

pub fn parse_binary_line(line: &str, ln: usize) -> Result<Instruction, Box<dyn Error>> {
    if line.len() != 24 || !line.chars().all(|c| c == '0' || c == '1') {
        return Err(Box::new(BinaryDecodeError::new(
            "Binary line must be exactly 24 characters of '0' or '1'",
            ln,
        )));
    }

    let value = u32::from_str_radix(line, 2).map_err(|_| BinaryDecodeError::new("Failed to parse binary line", ln))?;

    let opcode = ((value >> 19) & 0b11111) as u8;
    
    let reg_a     = ((value >> 16) & 0b111) as u8;
    let reg_b     = ((value >> 13) & 0b111) as u8;
    let reg_c     = ((value >> 10) & 0b111) as u8;

    let immediate = (value & 0xFFFF) as u16;
    let address = ((value >> 5) & 0x0FFF) as u16;
    let condition_flag = ((value >> 17) & 0b11) as u8;
    let offset    = (value & 0xFF) as u8;

    let instruction = match opcode {
        OPCODE_NOP  => { Instruction::NoOperation { } }
        OPCODE_HLT  => { Instruction::Halt { } }
        OPCODE_ADD  => { Instruction::Add { reg_a, reg_b, reg_c } }
        OPCODE_SUB  => { Instruction::Subtract { reg_a, reg_b, reg_c } }
        OPCODE_MUL  => { Instruction::Multiply { reg_a, reg_b, reg_c } }
        OPCODE_DIV  => { Instruction::Divide { reg_a, reg_b, reg_c } }
        OPCODE_REM  => { Instruction::Modulo { reg_a, reg_b, reg_c } }
        OPCODE_AND  => { Instruction::BitwiseAnd { reg_a, reg_b, reg_c } }
        OPCODE_NAND => { Instruction::BitwiseNand { reg_a, reg_b, reg_c } }
        OPCODE_OR   => { Instruction::BitwiseOr { reg_a, reg_b, reg_c } }
        OPCODE_NOR  => { Instruction::BitwiseNor { reg_a, reg_b, reg_c } }
        OPCODE_XOR  => { Instruction::BitwiseXor { reg_a, reg_b, reg_c } }
        OPCODE_XNOR => { Instruction::BitwiseXnor { reg_a, reg_b, reg_c } }
        OPCODE_NOT  => { Instruction::BitwiseNot { reg_a, reg_b } }
        OPCODE_RSH  => { Instruction::RightShift { reg_a, reg_b } }
        OPCODE_LSH  => { Instruction::LeftShift { reg_a, reg_b } }
        OPCODE_ROL  => { Instruction::Roll { reg_a, reg_b, reg_c } }
        OPCODE_LDI  => { Instruction::LoadImmediate { reg_a, immediate } }
        OPCODE_ADDI => { Instruction::AddImmediate { reg_a, immediate }  }
        OPCODE_SUBI => { Instruction::SubtractImmediate { reg_a, immediate }  }
        OPCODE_MULI => { Instruction::MultiplyImmediate { reg_a, immediate }  }
        OPCODE_DIVI => { Instruction::DivideImmediate { reg_a, immediate }  }
        OPCODE_JMP  => { Instruction::Jump { address } }
        OPCODE_BRH  => { Instruction::Branch { condition_flag, address } }
        OPCODE_CALL => { Instruction::Call { address } }
        OPCODE_RET  => { Instruction::Return { } }
        OPCODE_MLD  => { Instruction::MemoryLoad { reg_a, reg_b, offset } }
        OPCODE_MSTR => { Instruction::MemoryStore { reg_a, reg_b, offset } }
        OPCODE_DRW  => { Instruction::Draw { reg_x: reg_a, reg_y: reg_b, reg_rgb: reg_c } }
        OPCODE_PSHB => { Instruction::PushBuffer { } }
        OPCODE_PAD  => { Instruction::ControllerPad { reg_a } }
        OPCODE_RNG  => { Instruction::RandomNumberGenerator { reg_a } }
        _ => unreachable!()
    };

    Ok(instruction)
}