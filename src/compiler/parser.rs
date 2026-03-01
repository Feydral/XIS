use std::error::Error;

use crate::{errors::{ParseError, SyntaxError}, hardware, instruction::Instruction};

pub fn parse_line(line: &str, ln: usize) -> Result<Instruction, Box<dyn Error>> {
	let line = line
        .split("//")
        .next()
        .unwrap_or("")
        .trim();

	let tokens: Vec<&str> = line.split_whitespace().collect();

    if tokens.len() > 4 {
        return Err(Box::new(ParseError::new(format!("Too many operands. Help: consider removing unnecessary operand: '{}'.", tokens[4]), ln)));
    }

    let mut split = tokens;
    split.resize(4, "");

    let instruction = match split[0].to_uppercase().as_str() {
        "NOP" => { 
            if !(split[1].is_empty() && split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: consider removing all operands.", split[0]), ln)));
            }
            Instruction::NoOperation 
        },
        "HLT" => { 
            if !(split[1].is_empty() && split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: consider removing all operands.", split[0]), ln)));
            }
            Instruction::Halt 
        }
        "ADD" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 3 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::Add { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? } 
        },
        "SUB" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 3 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::Subtract { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? } 
        },
        "MUL" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 3 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::Multiply { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? } 
        },
        "DIV" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 3 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::Divide { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? } 
        },
        "REM" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 3 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::Modulo { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? } 
        },
        "AND" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 3 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::BitwiseAnd { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? } 
        },
        "NAND" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 3 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::BitwiseNand { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? } 
        },
        "OR" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 3 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::BitwiseOr { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? } 
        },
        "NOR" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 3 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::BitwiseNor { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? } 
        },
        "XOR" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 3 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::BitwiseXor { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? } 
        },
        "XNOR" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 3 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::BitwiseXnor { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? } 
        },
        "NOT" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes only 2 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::BitwiseNot { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)? } 
        },
        "RSH" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes only 2 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::RightShift { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)? } 
        },
        "LSH" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes only 2 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::LeftShift { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)? } 
        },
        "ROL" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 3 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::Roll { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? } 
        },
        "LDI" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 1 register and an immediate value as operands.", split[0], split[0]), ln)));
            }
            Instruction::LoadImmediate { reg_a: to_reg(split[1], ln)?, immediate: to_immediate(split[2], ln)? } 
        },
        "ADDI" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 1 register and an immediate value as operands.", split[0], split[0]), ln)));
            }
            Instruction::AddImmediate { reg_a: to_reg(split[1], ln)?, immediate: to_immediate(split[2], ln)? } 
        },
        "SUBI" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 1 register and an immediate value as operands.", split[0], split[0]), ln)));
            }
            Instruction::SubtractImmediate { reg_a: to_reg(split[1], ln)?, immediate: to_immediate(split[2], ln)? } 
        },
        "MULI" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 1 register and an immediate value as operands.", split[0], split[0]), ln)));
            }
            Instruction::MultiplyImmediate { reg_a: to_reg(split[1], ln)?, immediate: to_immediate(split[2], ln)? } 
        },
        "DIVI" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 1 register and an immediate value as operands.", split[0], split[0]), ln)));
            }
            Instruction::DivideImmediate { reg_a: to_reg(split[1], ln)?, immediate: to_immediate(split[2], ln)? } 
        },
        "JMP" => { 
            if !(!split[1].is_empty() && split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes an instruction memory address as operand.", split[0], split[0]), ln)));
            }
            Instruction::Jump { address: to_instr_addr(split[1], ln)? } 
        },
        "BRH" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes a flag (CF, ZF or OF) and an instruction memory address as operands.", split[0], split[0]), ln)));
            }
            Instruction::Branch { condition_flag: to_flag(split[1], ln)?, address: to_instr_addr(split[2], ln)? } 
        },
        "CALL" => { 
            if !(!split[1].is_empty() && split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes an instruction memory address as operand.", split[0], split[0]), ln)));
            }
            Instruction::Call { address: to_instr_addr(split[2], ln)? } 
        },
        "RET" => { 
            if !(split[1].is_empty() && split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: consider removing all operands.", split[0]), ln)));
            }
            Instruction::Return 
        },
        "MLD" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 2 registers and a offset value as operands.", split[0], split[0]), ln)));
            }
            Instruction::MemoryLoad { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, offset: to_offset(split[3], ln)? } 
        },
        "MSTR" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 2 registers and a offset value as operands.", split[0], split[0]), ln)));
            }
            Instruction::MemoryStore { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, offset: to_offset(split[3], ln)? } 
        },
        "DRW" => { 
            if !(!split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 3 registers as operands.", split[0], split[0]), ln)));
            }
            Instruction::Draw { reg_x: to_reg(split[1], ln)?, reg_y: to_reg(split[2], ln)?, reg_rgb: to_reg(split[3], ln)? } 
        },
        "PSHB" => { 
            if !(split[1].is_empty() && split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: consider removing all operands.", split[0]), ln)));
            }
            Instruction::PushBuffer 
        },
        "PAD" => { 
            if !(!split[1].is_empty() && split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 1 register as operand.", split[0], split[0]), ln)));
            }
            Instruction::ControllerPad { reg_a: to_reg(split[1], ln)? } 
        },
        "RNG" => { 
            if !(!split[1].is_empty() && split[2].is_empty() && split[3].is_empty()) {
                return Err(Box::new(SyntaxError::new(format!("Mnemonic '{}' has wrong operands. Help: '{}' takes 1 register as operand.", split[0], split[0]), ln)));
            }
            Instruction::RandomNumberGenerator { reg_a: to_reg(split[1], ln)? } 
        },
        _ => return Err(Box::new(ParseError::new(format!("Invalid mnemonic: '{}'. Help: consider checking the xis16 spreadsheet for valid mnemonics.", split[0]), ln))),
    };

    Ok(instruction)
}

fn to_reg(s: &str, ln: usize) -> Result<u8, ParseError> {
    let is_valid = match s {
        "r0" | "r1" | "r2" | "r3" | "r4" | "r5" | "r6" | "r7" => true,
        _ => false,
    };

    if is_valid {
        Ok(s[1..].parse::<u8>().unwrap())
    } else {
        Err(ParseError::new(format!("Invalid register: '{}'. Help: consider using one of 'r0', 'r1', 'r2', 'r3', 'r4', 'r5', 'r6', 'r7'.", s), ln))
    }
}

fn to_immediate(s: &str, ln: usize) -> Result<u16, ParseError> {
    let value = s.parse::<u16>().map_err(|_| ParseError::new(format!("Immediate value must be a number between 0 and 65535. {} is not in range 0..65535", s), ln))?;
    Ok(value)
} 

fn to_offset(s: &str, ln: usize) -> Result<u8, ParseError> {
    let value = s.parse::<u8>().map_err(|_| ParseError::new(format!("Offset value must be a number between 0 and {}.", hardware::MAX_MEMORY_OFFSET), ln))?;
    Ok(value)
}

fn to_instr_addr(s: &str, ln: usize) -> Result<u16, ParseError> {
    let value = s.parse::<u16>().map_err(|_| ParseError::new(format!("Address value must be a number between 0 and {}.", hardware::INSTRUCTION_MEM_SIZE - 1), ln))?;
    
    if value >= hardware::INSTRUCTION_MEM_SIZE as u16 {
        return Err(ParseError::new(format!("Address value must be a number between 0 and {}.", hardware::INSTRUCTION_MEM_SIZE - 1), ln));
    }

    Ok(value)
}

fn to_flag(s: &str, ln: usize) -> Result<u8, ParseError> {
    let value = match s {
        "CF" => hardware::CARRY_FLAG_BINARY as u8,
        "ZF" => hardware::ZERO_FLAG_BINARY as u8,
        "OF" => hardware::OVERFLOW_FLAG_BINARY as u8,
        _ => return Err(ParseError::new(format!("Invalid condition flag: {}. Must be one of 'CF', 'ZF', 'OF'.", s), ln)),
    };

    Ok(value)
}