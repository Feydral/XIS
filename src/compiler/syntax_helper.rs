use crate::{errors::*, instruction::Instruction};

pub fn parse_line(line: &str, line_number: usize) -> Result<Instruction, ParseError> {
	let line = line
        .split("//")
        .next()
        .unwrap_or("")
        .trim();

	if line.is_empty() {
		return Err(ParseError::new("Empty line", line_number));
	}

	let split = line.split_whitespace().collect::<Vec<&str>>();

    let instruction = match split[0].to_uppercase().as_str() {
        "NOP"  => Instruction::NoOperation,
        "HLT"  => Instruction::Halt,
        "ADD"  => Instruction::Add                   { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)?, reg_c: to_register(split[3], line_number)? },
        "SUB"  => Instruction::Subtract              { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)?, reg_c: to_register(split[3], line_number)? },
        "MUL"  => Instruction::Multiply              { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)?, reg_c: to_register(split[3], line_number)? },
        "DIV"  => Instruction::Divide                { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)?, reg_c: to_register(split[3], line_number)? },
        "REM"  => Instruction::Modulo                { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)?, reg_c: to_register(split[3], line_number)? },
        "AND"  => Instruction::BitwiseAnd            { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)?, reg_c: to_register(split[3], line_number)? },
        "NAND" => Instruction::BitwiseNand           { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)?, reg_c: to_register(split[3], line_number)? },
        "OR"   => Instruction::BitwiseOr             { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)?, reg_c: to_register(split[3], line_number)? },
        "NOR"  => Instruction::BitwiseNor            { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)?, reg_c: to_register(split[3], line_number)? },
        "XOR"  => Instruction::BitwiseXor            { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)?, reg_c: to_register(split[3], line_number)? },
        "XNOR" => Instruction::BitwiseXnor           { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)?, reg_c: to_register(split[3], line_number)? },
        "NOT"  => Instruction::BitwiseNot            { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)? },
        "RSH"  => Instruction::RightShift            { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)? },
        "LSH"  => Instruction::LeftShift             { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)? },
        "LDI"  => Instruction::LoadImmediate         { reg_a: to_register(split[1], line_number)?, immediate: to_immediate(split[2], line_number)? },
        "ADDI" => Instruction::AddImmediate          { reg_a: to_register(split[1], line_number)?, immediate: to_immediate(split[2], line_number)? },
        "SUBI" => Instruction::SubtractImmediate     { reg_a: to_register(split[1], line_number)?, immediate: to_immediate(split[2], line_number)? },
        "MULI" => Instruction::MultiplyImmediate     { reg_a: to_register(split[1], line_number)?, immediate: to_immediate(split[2], line_number)? },
        "DIVI" => Instruction::DivideImmediate       { reg_a: to_register(split[1], line_number)?, immediate: to_immediate(split[2], line_number)? },
        "JMP"  => Instruction::Jump                  { address: to_address(split[1], line_number)? },
        "BRH"  => Instruction::Branch                { address: to_address(split[1], line_number)? },
        "CALL" => Instruction::Call                  { condition_flag: to_flag(split[1], line_number)?, address: to_address(split[2], line_number)? },
        "RET"  => Instruction::Return,
        "MLD"  => Instruction::MemoryLoad            { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)?, offset: to_offset(split[3], line_number)? },
        "MSTR" => Instruction::MemoryStore           { reg_a: to_register(split[1], line_number)?, reg_b: to_register(split[2], line_number)?, offset: to_offset(split[3], line_number)? },
        "DRW"  => Instruction::Draw                  { reg_x: to_register(split[1], line_number)?, reg_y: to_register(split[2], line_number)?, reg_r: to_register(split[3], line_number)?, reg_g: to_register(split[4], line_number)?, reg_b: to_register(split[5], line_number)? },
        "PSHB" => Instruction::PushBuffer,
        "PAD"  => Instruction::ControllerPad         { reg_a: to_register(split[1], line_number)? },
        "RNG"  => Instruction::RandomNumberGenerator { reg_a: to_register(split[1], line_number)? },
        _ => return Err(ParseError::new("Invalid mnemonic", line_number)),
    };

    Ok(instruction)
}

fn to_mnemonic(s: &str, line_number: usize) -> Result<String, ParseError> {
    let is_valid = match s.to_uppercase().as_str() {
        "NOP"  | "HLT"  | "ADD"  | "SUB"  | 
        "MUL"  | "DIV"  | "REM"  | "AND"  | 
        "NAND" | "OR"   | "NOR"  | "XOR"  | 
        "XNOR" | "NOT"  | "RSH"  | "LSH"  | 
        "LDI"  | "ADDI" | "SUBI" | "MULI" |
        "DIVI" | "JMP"  | "BRH"  | "CALL" |
        "RET"  | "MLD"  | "MSTR" | "DRW"  | 
        "PSHB" | "PAD"  | "RNG" => true,
        _ => false,
    };

    if is_valid {
        Ok(s.to_uppercase())
    } else {
        Err(ParseError::new("Invalid mnemonic", line_number))
    }
}

fn to_register(s: &str, line_number: usize) -> Result<u8, ParseError> {
    let is_valid = match s.to_uppercase().as_str() {
        "r0" | "r1" | "r2" | "r3" | "r4" | "r5" | "r6" | "r7" => true,
        _ => false,
    };

    if is_valid {
        Ok(s[1..].parse::<u8>().unwrap())
    } else {
        Err(ParseError::new("Invalid register", line_number))
    }
}

fn to_immediate(s: &str, line_number: usize) -> Result<u16, ParseError> {
    let value = s.parse::<u16>();
    if value.is_err() {
        return Err(ParseError::new("Immediate value must be a number between 0 and 65535", line_number));
    }

    Ok(value.unwrap())
}

fn to_offset(s: &str, line_number: usize) -> Result<u8, ParseError> {
    let value = s.parse::<u8>();
    if value.is_err() {
        return Err(ParseError::new("Offset value must be a number between 0 and 255", line_number));
    }

    Ok(value.unwrap())
}

fn to_address(s: &str, line_number: usize) -> Result<u16, ParseError> {
    let value = s.parse::<u16>();
    if value.is_err() {
        return Err(ParseError::new("Address value must be a number between 0 and 4095", line_number));
    } 
    
    let address = value.unwrap();
    if address > 4095 {
        return Err(ParseError::new("Address value must be a number between 0 and 4095", line_number));
    }

    Ok(address)
}

fn to_flag(s: &str, line_number: usize) -> Result<u8, ParseError> {
    let value = match s.to_uppercase().as_str() {
        "Carry" => 0b010,
        "Zero" => 0b100,
        _ => return Err(ParseError::new("Invalid condition flag. Must be one of 'Carry', 'Zero'", line_number)),
    };

    Ok(value)
}