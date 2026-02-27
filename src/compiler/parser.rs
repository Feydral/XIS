use std::error::Error;

use crate::{errors::{CompileError, SyntaxError}, hardware, instruction::Instruction};

pub fn parse_line(line: &str, ln: usize) -> Result<Instruction, Box<dyn Error>> {
	let line = line
        .split("//")
        .next()
        .unwrap_or("")
        .trim();

	if line.is_empty() {
		return Err(Box::new(CompileError::new("Empty line", ln)));
	}

	let mut split = Vec::with_capacity(4);
    split.extend(line.split_whitespace().take(4));
    split.resize(4, "");

    check_syntax(&split, ln)?;

    let instruction = match split[0].to_uppercase().as_str() {
        "NOP"  => Instruction::NoOperation,
        "HLT"  => Instruction::Halt,
        "ADD"  => Instruction::Add                   { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? },
        "SUB"  => Instruction::Subtract              { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? },
        "MUL"  => Instruction::Multiply              { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? },
        "DIV"  => Instruction::Divide                { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? },
        "REM"  => Instruction::Modulo                { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? },
        "AND"  => Instruction::BitwiseAnd            { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? },
        "NAND" => Instruction::BitwiseNand           { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? },
        "OR"   => Instruction::BitwiseOr             { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? },
        "NOR"  => Instruction::BitwiseNor            { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? },
        "XOR"  => Instruction::BitwiseXor            { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? },
        "XNOR" => Instruction::BitwiseXnor           { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? },
        "NOT"  => Instruction::BitwiseNot            { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)? },
        "RSH"  => Instruction::RightShift            { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)? },
        "LSH"  => Instruction::LeftShift             { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)? },
        "ROL"  => Instruction::Roll                  { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, reg_c: to_reg(split[3], ln)? },
        "LDI"  => Instruction::LoadImmediate         { reg_a: to_reg(split[1], ln)?, immediate: to_immediate(split[2], ln)? },
        "ADDI" => Instruction::AddImmediate          { reg_a: to_reg(split[1], ln)?, immediate: to_immediate(split[2], ln)? },
        "SUBI" => Instruction::SubtractImmediate     { reg_a: to_reg(split[1], ln)?, immediate: to_immediate(split[2], ln)? },
        "MULI" => Instruction::MultiplyImmediate     { reg_a: to_reg(split[1], ln)?, immediate: to_immediate(split[2], ln)? },
        "DIVI" => Instruction::DivideImmediate       { reg_a: to_reg(split[1], ln)?, immediate: to_immediate(split[2], ln)? },
        "JMP"  => Instruction::Jump                  { address: to_instr_addr(split[1], ln)? },
        "BRH"  => Instruction::Branch                { address: to_instr_addr(split[1], ln)? },
        "CALL" => Instruction::Call                  { condition_flag: to_flag(split[1], ln)?, address: to_instr_addr(split[2], ln)? },
        "RET"  => Instruction::Return,
        "MLD"  => Instruction::MemoryLoad            { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, offset: to_offset(split[3], ln)? },
        "MSTR" => Instruction::MemoryStore           { reg_a: to_reg(split[1], ln)?, reg_b: to_reg(split[2], ln)?, offset: to_offset(split[3], ln)? },
        "DRW"  => Instruction::Draw                  { reg_x: to_reg(split[1], ln)?, reg_y: to_reg(split[2], ln)?, reg_rgb: to_reg(split[3], ln)? },
        "PSHB" => Instruction::PushBuffer,
        "PAD"  => Instruction::ControllerPad         { reg_a: to_reg(split[1], ln)? },
        "RNG"  => Instruction::RandomNumberGenerator { reg_a: to_reg(split[1], ln)? },
        _ => return Err(Box::new(CompileError::new("Invalid mnemonic", ln))),
    };

    Ok(instruction)
}

fn to_reg(s: &str, ln: usize) -> Result<u8, CompileError> {
    let is_valid = match s {
        "r0" | "r1" | "r2" | "r3" | "r4" | "r5" | "r6" | "r7" => true,
        _ => false,
    };

    if is_valid {
        Ok(s[1..].parse::<u8>().unwrap())
    } else {
        Err(CompileError::new("Invalid register", ln))
    }
}

fn to_immediate(s: &str, ln: usize) -> Result<u16, CompileError> {
    let value = s.parse::<u16>().map_err(|_| CompileError::new("Immediate value must be a number between 0 and 65535", ln))?;
    Ok(value)
} 

fn to_offset(s: &str, ln: usize) -> Result<u8, CompileError> {
    let value = s.parse::<u8>().map_err(|_| CompileError::new(format!("Offset value must be a number between 0 and {}", hardware::MAX_MEMORY_OFFSET), ln))?;
    Ok(value)
}

fn to_instr_addr(s: &str, ln: usize) -> Result<u16, CompileError> {
    let value = s.parse::<u16>().map_err(|_| CompileError::new(format!("Address value must be a number between 0 and {}", hardware::INSTRUCTION_MEM_SIZE - 1), ln))?;
    
    if value >= hardware::INSTRUCTION_MEM_SIZE as u16 {
        return Err(CompileError::new(format!("Address value must be a number between 0 and {}", hardware::INSTRUCTION_MEM_SIZE - 1), ln));
    }

    Ok(value)
}

fn to_flag(s: &str, ln: usize) -> Result<u8, CompileError> {
    let value = match s {
        "CF" => hardware::CARRY_FLAG_BINARY as u8,
        "ZF" => hardware::ZERO_FLAG_BINARY as u8,
        "OF" => hardware::OVERFLOW_FLAG_BINARY as u8,
        _ => return Err(CompileError::new("Invalid condition flag. Must be one of 'CF', 'ZF', 'OF'", ln)),
    };

    Ok(value)
}


fn check_syntax(split: &[&str], ln: usize) -> Result<(), SyntaxError> {
    let mnemonic = split[0].to_uppercase();
    let (m, ok) = match mnemonic.as_str() {
        m @ "NOP"  => (m, split[1].is_empty() && split[2].is_empty() && split[3].is_empty()),
        m @ "HLT"  => (m, split[1].is_empty() && split[2].is_empty() && split[3].is_empty()),
        m @ "ADD"  => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "SUB"  => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "MUL"  => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "DIV"  => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "REM"  => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "AND"  => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "NAND" => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "OR"   => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "NOR"  => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "XOR"  => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "XNOR" => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "NOT"  => (m, !split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()),
        m @ "RSH"  => (m, !split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()),
        m @ "LSH"  => (m, !split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()),
        m @ "ROL"  => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "LDI"  => (m, !split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()),
        m @ "ADDI" => (m, !split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()),
        m @ "SUBI" => (m, !split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()),
        m @ "MULI" => (m, !split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()),
        m @ "DIVI" => (m, !split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()),
        m @ "JMP"  => (m, !split[1].is_empty() && split[2].is_empty() && split[3].is_empty()),
        m @ "BRH"  => (m, !split[1].is_empty() && !split[2].is_empty() && split[3].is_empty()),
        m @ "CALL" => (m, !split[1].is_empty() && split[2].is_empty() && split[3].is_empty()),
        m @ "RET"  => (m, split[1].is_empty() && split[2].is_empty() && split[3].is_empty()),
        m @ "MLD"  => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "MSTR" => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "DRW"  => (m, !split[1].is_empty() && !split[2].is_empty() && !split[3].is_empty()),
        m @ "PSHB" => (m, split[1].is_empty() && split[2].is_empty() && split[3].is_empty()),
        m @ "PAD"  => (m, !split[1].is_empty() && split[2].is_empty() && split[3].is_empty()),
        m @ "RNG"  => (m, !split[1].is_empty() && split[2].is_empty() && split[3].is_empty()),
        m @ _ => return Err(SyntaxError::new(format!("Unknown mnemonic '{}'", m), ln)),
    };

    if ok {
        return Ok(());
    }

    Err(SyntaxError::new(format!("Mnemonic '{}' has wrong operands", m), ln))
}