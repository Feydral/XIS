use crate::{errors::CompileError, instruction::Instruction};

pub fn parse_line(line: &str, ln: usize) -> Result<Instruction, CompileError> {
	let line = line
        .split("//")
        .next()
        .unwrap_or("")
        .trim();

	if line.is_empty() {
		return Err(CompileError::new("Empty line", ln));
	}

	let mut split = Vec::with_capacity(5);
    split.extend(line.split_whitespace().take(5));
    split.resize(5, "");

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
        "DRW"  => Instruction::Draw                  { reg_x: to_reg(split[1], ln)?, reg_y: to_reg(split[2], ln)?, reg_r: to_reg(split[3], ln)?, reg_g: to_reg(split[4], ln)?, reg_b: to_reg(split[5], ln)? },
        "PSHB" => Instruction::PushBuffer,
        "PAD"  => Instruction::ControllerPad         { reg_a: to_reg(split[1], ln)? },
        "RNG"  => Instruction::RandomNumberGenerator { reg_a: to_reg(split[1], ln)? },
        _ => return Err(CompileError::new("Invalid mnemonic", ln)),
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
    let value = s.parse::<u8>().map_err(|_| CompileError::new("Offset value must be a number between 0 and 255", ln))?;
    Ok(value)
}

fn to_instr_addr(s: &str, ln: usize) -> Result<u16, CompileError> {
    let value = s.parse::<u16>().map_err(|_| CompileError::new("Address value must be a number between 0 and 4095", ln))?;
    
    if value > 4095 {
        return Err(CompileError::new("Address value must be a number between 0 and 4095", ln));
    }

    Ok(value)
}

fn to_flag(s: &str, ln: usize) -> Result<u8, CompileError> {
    let value = match s {
        "Carry" => 0b001,
        "Zero" => 0b010,
        _ => return Err(CompileError::new("Invalid condition flag. Must be one of 'Carry', 'Zero'", ln)),
    };

    Ok(value)
}