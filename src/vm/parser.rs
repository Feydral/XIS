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

    let value = u32::from_str_radix(line, 2)
        .map_err(|_| BinaryDecodeError::new("Failed to parse binary line", ln))?;

    let opcode = ((value >> 19) & 0b11111) as u8;

    let instruction = match opcode {
        OPCODE_NOP => {
            // NOP
            todo!()
        }
        OPCODE_HLT => {
            // HLT
            todo!()
        }
        OPCODE_ADD => {
            // ADD
            todo!()
        }
        OPCODE_SUB => {
            // SUB
            todo!()
        }
        OPCODE_MUL => {
            // MUL
            todo!()
        }
        OPCODE_DIV => {
            // DIV
            todo!()
        }
        OPCODE_REM => {
            // REM
            todo!()
        }
        OPCODE_AND => {
            // AND
            todo!()
        }
        OPCODE_NAND => {
            // NAND
            todo!()
        }
        OPCODE_OR => {
            // OR
            todo!()
        }
        OPCODE_NOR => {
            // NOR
            todo!()
        }
        OPCODE_XOR => {
            // XOR
            todo!()
        }
        OPCODE_XNOR => {
            // XNOR
            todo!()
        }
        OPCODE_NOT => {
            // NOT
            todo!()
        }
        OPCODE_RSH => {
            // RSH
            todo!()
        }
        OPCODE_LSH => {
            // LSH
            todo!()
        }
        OPCODE_ROL => {
            // ROL
            todo!()
        }
        OPCODE_LDI => {
            // LDI
            todo!()
        }
        OPCODE_ADDI => {
            // ADDI
            todo!()
        }
        OPCODE_SUBI => {
            // SUBI
            todo!()
        }
        OPCODE_MULI => {
            // MULI
            todo!()
        }
        OPCODE_DIVI => {
            // DIVI
            todo!()
        }
        OPCODE_JMP => {
            // JMP
            todo!()
        }
        OPCODE_BRH => {
            // BRH
            todo!()
        }
        OPCODE_CALL => {
            // CALL
            todo!()
        }
        OPCODE_RET => {
            // RET
            todo!()
        }
        OPCODE_MLD => {
            // MLD
            todo!()
        }
        OPCODE_MSTR => {
            // MSTR
            todo!()
        }
        OPCODE_DRW => {
            // DRW
            todo!()
        }
        OPCODE_PSHB => {
            // PSHB
            todo!()
        }
        OPCODE_PAD => {
            // PAD
            todo!()
        }
        OPCODE_RNG => {
            // RNG
            todo!()
        }
        _ => unreachable!()
    };

    Ok(instruction)
}