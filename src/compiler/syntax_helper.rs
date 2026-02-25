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


    let mnemonic = split[0].to_uppercase().as_str();
    let mnemonic_token = Token::Mnemonic(mnemonic.to_string());

	if !mnemonic_token.is_valid() {
		return Err(ParseError::new("Invalid mnemonic", line_number));
	}

    let instruction = match mnemonic {
        "NOP"  => Instruction::NoOperation,
        "HLT"  => Instruction::Halt,
        "ADD"  => Instruction::Add                   { reg_a: to_register(split[1])?, reg_b: to_register(split[2])?, reg_c: to_register(split[3])? },
        "SUB"  => Instruction::Subtract              { reg_a: to_register(split[1])?, reg_b: to_register(split[2])?, reg_c: to_register(split[3])? },
        "MUL"  => Instruction::Multiply              { reg_a: to_register(split[1])?, reg_b: to_register(split[2])?, reg_c: to_register(split[3])? },
        "DIV"  => Instruction::Divide                { reg_a: to_register(split[1])?, reg_b: to_register(split[2])?, reg_c: to_register(split[3])? },
        "REM"  => Instruction::Modulo                { reg_a: to_register(split[1])?, reg_b: to_register(split[2])?, reg_c: to_register(split[3])? },
        "AND"  => Instruction::BitwiseAnd            { reg_a: to_register(split[1])?, reg_b: to_register(split[2])?, reg_c: to_register(split[3])? },
        "NAND" => Instruction::BitwiseNand           { reg_a: to_register(split[1])?, reg_b: to_register(split[2])?, reg_c: to_register(split[3])? },
        "OR"   => Instruction::BitwiseOr             { reg_a: to_register(split[1])?, reg_b: to_register(split[2])?, reg_c: to_register(split[3])? },
        "NOR"  => Instruction::BitwiseNor            { reg_a: to_register(split[1])?, reg_b: to_register(split[2])?, reg_c: to_register(split[3])? },
        "XOR"  => Instruction::BitwiseXor            { reg_a: to_register(split[1])?, reg_b: to_register(split[2])?, reg_c: to_register(split[3])? },
        "XNOR" => Instruction::BitwiseXnor           { reg_a: to_register(split[1])?, reg_b: to_register(split[2])?, reg_c: to_register(split[3])? },
        "NOT"  => Instruction::BitwiseNot            { reg_a: to_register(split[1])?, reg_b: to_register(split[2])? },
        "RSH"  => Instruction::RightShift            { reg_a: to_register(split[1])?, reg_b: to_register(split[2])? },
        "LSH"  => Instruction::LeftShift             { reg_a: to_register(split[1])?, reg_b: to_register(split[2])? },
        "LDI"  => Instruction::LoadImmediate         { reg_a: to_register(split[1])?, immediate: to_immediate(split[2])? },
        "ADDI" => Instruction::AddImmediate          { reg_a: to_register(split[1])?, immediate: to_immediate(split[2])? },
        "SUBI" => Instruction::SubtractImmediate     { reg_a: to_register(split[1])?, immediate: to_immediate(split[2])? },
        "MULI" => Instruction::MultiplyImmediate     { reg_a: to_register(split[1])?, immediate: to_immediate(split[2])? },
        "DIVI" => Instruction::DivideImmediate       { reg_a: to_register(split[1])?, immediate: to_immediate(split[2])? },
        "JMP"  => Instruction::Jump                  { address: to_address(split[1])? },
        "BRH"  => Instruction::Branch                { address: to_address(split[1])? },
        "CALL" => Instruction::Call                  { condition_flag: to_flag(split[1])?, address: to_address(split[2])? },
        "RET"  => Instruction::Return,
        "MLD"  => Instruction::MemoryLoad            { reg_a: to_register(split[1])?, reg_b: to_register(split[2])?, offset: to_offset(split[3])? },
        "MSTR" => Instruction::MemoryStore           { reg_a: to_register(split[1])?, reg_b: to_register(split[2])?, offset: to_offset(split[3])? },
        "DRW"  => Instruction::Draw                  { reg_x: to_register(split[1])?, reg_y: to_register(split[2])?, reg_r: to_register(split[3])?, reg_g: to_register(split[4])?, reg_b: to_register(split[5])? },
        "PSHB" => Instruction::PushBuffer,
        "PAD"  => Instruction::ControllerPad         { reg_a: to_register(split[1])? },
        "RNG"  => Instruction::RandomNumberGenerator { reg_a: to_register(split[1])? },
    }
}

fn to_register(s: &str) -> Result<u8, ParseError> {
    let token = Token::Register(s.to_string());
    if token.is_valid() {
        Ok(s[1..].parse::<u8>().unwrap())
    } else {
        Err(ParseError::new("Invalid register", 0))
    }
}

fn to_immediate(s: &str) -> Result<u16, ParseError> {
    let token = Token::Immediate(s.parse::<u16>().unwrap());
    if token.is_valid() {
        Ok(s.parse::<u16>().unwrap())
    } else {
        Err(ParseError::new("Invalid immediate value", 0))
    }
}

fn to_offset(s: &str) -> Result<u8, ParseError> {
    let token = Token::Offset(s.parse::<u8>().unwrap());
    if token.is_valid() {
        Ok(s.parse::<u8>().unwrap())
    } else {
        Err(ParseError::new("Invalid memory offset", 0))
    }
}

fn to_address(s: &str) -> Result<u16, ParseError> {
    let token = Token::InstructionAddress(s.parse::<u16>().unwrap());
    if token.is_valid() {
        Ok(s.parse::<u16>().unwrap())
    } else {
        Err(ParseError::new("Invalid instruction address", 0))
    }
}

fn to_flag(s: &str) -> Result<u8, ParseError> {
    let token = Token::ConditionFlag(s.to_string());
    if token.is_valid() {
        Ok(match s.to_uppercase().as_str() {
            "OF" => 0,
            "CF" => 1,
            "ZF" => 2,
            _ => unreachable!(),
        })
    } else {
        Err(ParseError::new("Invalid condition flag", 0))
    }
}