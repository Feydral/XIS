use crate::compiler::{killer::{self, Error}, virtual_file::VirtualFile};

#[derive(Debug, Clone, Copy)]
pub enum Mnemonic { NOP, HLT, ADD, SUB, MUL, DIV, REM, AND, NAND, OR, NOR, XOR, XNOR, NOT, RSH, LSH, LDI, ADDI, SUBI, MULI, DIVI, JMP, BRH, CALL, RET, MLD, MSTR, DRW, PSHB, PAD, RNG, }

impl Mnemonic {
    fn from_str(s: &str) -> Option<Mnemonic> {
        use Mnemonic::*;
        match s {
            "NOP" => Some(NOP),
            "HLT" => Some(HLT),
            "ADD" => Some(ADD),
            "SUB" => Some(SUB),
            "MUL" => Some(MUL),
            "DIV" => Some(DIV),
            "REM" => Some(REM),
            "AND" => Some(AND),
            "NAND" => Some(NAND),
            "OR" => Some(OR),
            "NOR" => Some(NOR),
            "XOR" => Some(XOR),
            "XNOR" => Some(XNOR),
            "NOT" => Some(NOT),
            "RSH" => Some(RSH),
            "LSH" => Some(LSH),
            "LDI" => Some(LDI),
            "ADDI" => Some(ADDI),
            "SUBI" => Some(SUBI),
            "MULI" => Some(MULI),
            "DIVI" => Some(DIVI),
            "JMP" => Some(JMP),
            "BRH" => Some(BRH),
            "CALL" => Some(CALL),
            "RET" => Some(RET),
            "MLD" => Some(MLD),
            "MSTR" => Some(MSTR),
            "DRW" => Some(DRW),
            "PSHB" => Some(PSHB),
            "PAD" => Some(PAD),
            "RNG" => Some(RNG),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OperandType {
    Register,            // r0..r7
    Immediate,           // 0..65535
    InstructionAddress,  // 0..4095
    Offset,              // 0..255
    Flag,                // 0..3
}

#[derive(Debug)]
pub enum Operand {
    Register(u8),
    Immediate(i32),
    InstructionAddress(u16),
    Offset(u8),
    Flag(u8),
}

/// Parse register
fn parse_register(s: &str) -> Result<Operand, Error> {
    if s.len() == 2 && &s[0..1] == "r" {
        if let Ok(num) = s[1..2].parse::<u8>() {
            if num <= 7 {
                return Ok(Operand::Register(num));
            }
        }
    }
    Err(Error::ParseErrorInvalidRegister(s.to_string()))
}

/// Parse an operand according to its expected type
fn parse_operand(s: &str, expected_type: OperandType) -> Result<Operand, Error> {
    match expected_type {
        OperandType::Register => parse_register(s),
        OperandType::Immediate => s.parse::<i32>().map(Operand::Immediate).map_err(|_| Error::ParseErrorInvalidImmediate(s.to_string())),
        OperandType::InstructionAddress => s.parse::<u16>().map_err(|_| Error::ParseErrorInvalidInstructionAddress(s.to_string())).and_then(|v| if v <= 4095 { Ok(Operand::InstructionAddress(v)) } else { Err(Error::ParseErrorInvalidInstructionAddress(s.to_string())) }),
        OperandType::Offset => s.parse::<u8>().map(Operand::Offset).map_err(|_| Error::ParseErrorInvalidOffset(s.to_string())),
        OperandType::Flag => s.parse::<u8>().map_err(|_| Error::ParseErrorInvalidFlag(s.to_string())).and_then(|v| if v <= 3 { Ok(Operand::Flag(v)) } else { Err(Error::ParseErrorInvalidFlag(s.to_string())) }),
    }
}

/// Define expected operands per mnemonic
fn expected_operands(m: Mnemonic) -> &'static [OperandType] {
    use Mnemonic::*;
    use OperandType::*;
    match m {
        ADD | SUB | MUL | DIV | REM | AND | NAND | OR | NOR | XOR | XNOR => &[Register, Register, Register],
        NOT | RSH | LSH => &[Register, Register],
        LDI | ADDI | SUBI | MULI | DIVI => &[Register, Immediate],
        JMP | CALL => &[InstructionAddress],
        BRH => &[Flag, InstructionAddress],
        RET | NOP | HLT | PSHB => &[],
        MLD | MSTR => &[Register, Register, Offset],
        DRW => &[Register, Register, Register],
        PAD | RNG => &[Register],
    }
}

/// Parse a line into a mnemonic and operands
fn parse_line(line: &str) -> Result<(Mnemonic, Vec<Operand>), Error> {
    let mut parts = line.split_whitespace();
    let mnemonic_str = parts.next().ok_or(Error::ParseErrorUnknownMnemonic(line.to_string()))?;
    let mnemonic = Mnemonic::from_str(mnemonic_str).ok_or(Error::ParseErrorUnknownMnemonic(mnemonic_str.to_string()))?;

    let expected = expected_operands(mnemonic);

    let operands: Result<Vec<Operand>, Error> = parts
        .zip(expected.iter())
        .map(|(s, &ty)| parse_operand(s, ty))
        .collect();

    let operands = operands?;

    if operands.len() != expected.len() {
        return Err(Error::ParseErrorWrongOperandCount { mnemonic: mnemonic_str.to_string(), expected: expected.len(), given: operands.len() });
    }

    Ok((mnemonic, operands))
}


pub fn parse_file(path: &str) -> Vec<(Mnemonic, Vec<Operand>)> {
    let mut instructions: Vec<(Mnemonic, Vec<Operand>)> = Vec::new();
    let vfile = VirtualFile::load_from_file(path);

    for line in &vfile.data {
        let line = line.trim();
        if line.is_empty() || line.starts_with('/') {
            continue;
        }

        match parse_line(line) {
            Ok((mnemonic, operands)) => {
                instructions.push((mnemonic, operands));
            }
            Err(e) => {
                killer::kill(e);
            }
        }
    }

    instructions
}