use crate::compiler::parsing::{Mnemonic, Operand};

fn opcode(m: &Mnemonic) -> u32 {
    match m {
        Mnemonic::NOP  => 0b00000,
        Mnemonic::HLT  => 0b00001,
        Mnemonic::ADD  => 0b00010,
        Mnemonic::SUB  => 0b00011,
        Mnemonic::MUL  => 0b00100,
        Mnemonic::DIV  => 0b00101,
        Mnemonic::REM  => 0b00110,
        Mnemonic::AND  => 0b00111,
        Mnemonic::NAND => 0b01000,
        Mnemonic::OR   => 0b01001,
        Mnemonic::NOR  => 0b01010,
        Mnemonic::XOR  => 0b01011,
        Mnemonic::XNOR => 0b01100,
        Mnemonic::NOT  => 0b01101,
        Mnemonic::RSH  => 0b01110,
        Mnemonic::LSH  => 0b01111,
        Mnemonic::LDI  => 0b10000,
        Mnemonic::ADDI => 0b10001,
        Mnemonic::SUBI => 0b10011,
        Mnemonic::MULI => 0b10100,
        Mnemonic::DIVI => 0b10101,
        Mnemonic::JMP  => 0b10110,
        Mnemonic::BRH  => 0b10111,
        Mnemonic::CALL => 0b11000,
        Mnemonic::RET  => 0b11001,
        Mnemonic::MLD  => 0b11010,
        Mnemonic::MSTR => 0b11011,
        Mnemonic::DRW  => 0b11100,
        Mnemonic::PSHB => 0b11101,
        Mnemonic::PAD  => 0b11110,
        Mnemonic::RNG  => 0b11111,
    }
}

struct BitWriter {
    bits: u32,
    used: u8,
}

impl BitWriter {
    fn new() -> Self {
        Self { bits: 0, used: 0 }
    }

    fn push(&mut self, value: u32, count: u8) {
        self.bits <<= count;
        self.bits |= value & ((1 << count) - 1);
        self.used += count;
    }

    fn finish(self) -> [u8; 3] {
        let v = self.bits << (24 - self.used);
        [
            ((v >> 16) & 0xFF) as u8,
            ((v >> 8) & 0xFF) as u8,
            (v & 0xFF) as u8,
        ]
    }
}

fn reg(op: &Operand) -> u32 {
    match op {
        Operand::Register(r) => (*r & 0b111) as u32,
        _ => unreachable!("Expected register"),
    }
}

fn imm16(op: &Operand) -> u32 {
    match op {
        Operand::Immediate(v) => (*v as u32) & 0xFFFF,
        Operand::InstructionAddress(v) => (*v as u32) & 0xFFFF,
        _ => unreachable!("Expected 16-bit immediate"),
    }
}

fn imm8(op: &Operand) -> u32 {
    match op {
        Operand::Offset(v) => (*v as u32) & 0xFF,
        _ => unreachable!("Expected 8-bit offset"),
    }
}

fn flag(op: &Operand) -> u32 {
    match op {
        Operand::Flag(f) => (*f & 0b11) as u32,
        _ => unreachable!("Expected flag"),
    }
}

pub fn assemble(program: &Vec<(Mnemonic, Vec<Operand>)>) -> Vec<[u8; 3]> {
    let mut output = Vec::new();

    for (mnemonic, operands) in program {
        let mut w = BitWriter::new();

        // ---- Opcode (5 Bit) ----
        w.push(opcode(mnemonic), 5);

        match mnemonic {

            // ===== R R R =====
            Mnemonic::ADD | Mnemonic::SUB | Mnemonic::MUL | Mnemonic::DIV
            | Mnemonic::REM | Mnemonic::AND | Mnemonic::NAND
            | Mnemonic::OR | Mnemonic::NOR | Mnemonic::XOR | Mnemonic::XNOR => {
                w.push(reg(&operands[0]), 3);
                w.push(reg(&operands[1]), 3);
                w.push(reg(&operands[2]), 3);
            }

            // ===== R R =====
            Mnemonic::NOT | Mnemonic::RSH | Mnemonic::LSH => {
                w.push(reg(&operands[0]), 3);
                w.push(reg(&operands[1]), 3);
            }

            // ===== R IMM =====
            Mnemonic::LDI | Mnemonic::ADDI | Mnemonic::SUBI
            | Mnemonic::MULI | Mnemonic::DIVI => {
                w.push(reg(&operands[0]), 3);
                w.push(imm16(&operands[1]), 16);
            }

            // ===== JMP / CALL =====
            Mnemonic::JMP | Mnemonic::CALL => {
                w.push(imm16(&operands[0]), 16);
            }

            // ===== BRH =====
            Mnemonic::BRH => {
                w.push(flag(&operands[0]), 2);
                w.push(imm16(&operands[1]), 16);
            }

            // ===== MLD / MSTR =====
            Mnemonic::MLD | Mnemonic::MSTR => {
                w.push(reg(&operands[0]), 3);
                w.push(reg(&operands[1]), 3);
                w.push(imm8(&operands[2]), 8);
            }

            // ===== DRW =====
            Mnemonic::DRW => {
                w.push(reg(&operands[0]), 3);
                w.push(reg(&operands[1]), 3);
                w.push(reg(&operands[2]), 3);
            }

            // ===== PAD / RNG =====
            Mnemonic::PAD | Mnemonic::RNG => {
                w.push(reg(&operands[0]), 3);
            }

            // ===== NO OPERANDS =====
            Mnemonic::NOP | Mnemonic::HLT | Mnemonic::RET | Mnemonic::PSHB => {}
        }

        output.push(w.finish());
    }

    output
}