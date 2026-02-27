use crate::math::mathi;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    NoOperation,
    Halt,
    Add                   { reg_a: u8, reg_b: u8, reg_c: u8 },
    Subtract              { reg_a: u8, reg_b: u8, reg_c: u8 },
    Multiply              { reg_a: u8, reg_b: u8, reg_c: u8 },
    Divide                { reg_a: u8, reg_b: u8, reg_c: u8 },
    Modulo                { reg_a: u8, reg_b: u8, reg_c: u8 },
    BitwiseAnd            { reg_a: u8, reg_b: u8, reg_c: u8 },
    BitwiseNand           { reg_a: u8, reg_b: u8, reg_c: u8 },
    BitwiseOr             { reg_a: u8, reg_b: u8, reg_c: u8 },
    BitwiseNor            { reg_a: u8, reg_b: u8, reg_c: u8 },
    BitwiseXor            { reg_a: u8, reg_b: u8, reg_c: u8 },
    BitwiseXnor           { reg_a: u8, reg_b: u8, reg_c: u8 },
    BitwiseNot            { reg_a: u8, reg_b: u8 },
    RightShift            { reg_a: u8, reg_b: u8 },
    LeftShift             { reg_a: u8, reg_b: u8 },
    Roll                  { reg_a: u8, reg_b: u8, reg_c: u8 },
    LoadImmediate         { reg_a: u8, immediate: u16 },
    AddImmediate          { reg_a: u8, immediate: u16 },
    SubtractImmediate     { reg_a: u8, immediate: u16 },
    MultiplyImmediate     { reg_a: u8, immediate: u16 },
    DivideImmediate       { reg_a: u8, immediate: u16 },
    Jump                  { address: u16 },
    Branch                { address: u16 },
    Call                  { condition_flag: u8, address: u16 },
    Return,
    MemoryLoad            { reg_a: u8, reg_b: u8, offset: u8 },
    MemoryStore           { reg_a: u8, reg_b: u8, offset: u8 },
    Draw                  { reg_x: u8, reg_y: u8, reg_rgb: u8 },
    PushBuffer,
    ControllerPad         { reg_a: u8 },
    RandomNumberGenerator { reg_a: u8 },
}

impl Instruction {
    fn encode(&self) -> u32 {
        let mut bits: u32 = 0;

        bits |= (self.get_opcode() as u32) << 19;

        match self {
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

            Instruction::BitwiseNot { reg_a, reg_b }
            | Instruction::RightShift { reg_a, reg_b }
            | Instruction::LeftShift { reg_a, reg_b } => {
                bits |= (*reg_a as u32 & 0b111) << 16;
                bits |= (*reg_b as u32 & 0b111) << 13;
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
            | Instruction::Branch { address } => {
                bits |= *address as u32 & 0xFFFF;
            }

            Instruction::Call { condition_flag, address } => {
                bits |= (*condition_flag as u32 & 0b111) << 16;
                bits |= *address as u32 & 0xFFFF;
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

            Instruction::ControllerPad { reg_a }
            | Instruction::RandomNumberGenerator { reg_a } => {
                bits |= (*reg_a as u32 & 0b111) << 16;
            }

            Instruction::NoOperation
            | Instruction::Halt
            | Instruction::Return
            | Instruction::PushBuffer => {}
        }

        bits
    }
    
    fn get_opcode(&self) -> u8 {
        match self {
            Instruction::NoOperation                  => 0b00000,
            Instruction::Halt                         => 0b00001,
            Instruction::Add { .. }                   => 0b00010,
            Instruction::Subtract { .. }              => 0b00011,
            Instruction::Multiply { .. }              => 0b00100,
            Instruction::Divide { .. }                => 0b00101,
            Instruction::Modulo { .. }                => 0b00110,
            Instruction::BitwiseAnd { .. }            => 0b00111,
            Instruction::BitwiseNand { .. }           => 0b01000,
            Instruction::BitwiseOr { .. }             => 0b01001,
            Instruction::BitwiseNor { .. }            => 0b01010,
            Instruction::BitwiseXor { .. }            => 0b01011,
            Instruction::BitwiseXnor { .. }           => 0b01100,
            Instruction::BitwiseNot { .. }            => 0b01101,
            Instruction::RightShift { .. }            => 0b01110,
            Instruction::LeftShift { .. }             => 0b01111,
            Instruction::Roll { .. }                  => 0b10000,
            Instruction::LoadImmediate { .. }         => 0b10001,
            Instruction::AddImmediate { .. }          => 0b10010,
            Instruction::SubtractImmediate { .. }     => 0b10011,
            Instruction::MultiplyImmediate { .. }     => 0b10100,
            Instruction::DivideImmediate { .. }       => 0b10101,
            Instruction::Jump { .. }                  => 0b10110,
            Instruction::Branch { .. }                => 0b10111,
            Instruction::Call { .. }                  => 0b11000,
            Instruction::Return                       => 0b11001,
            Instruction::MemoryLoad { .. }            => 0b11010,
            Instruction::MemoryStore { .. }           => 0b11011,
            Instruction::Draw { .. }                  => 0b11100,
            Instruction::PushBuffer                   => 0b11101,
            Instruction::ControllerPad { .. }         => 0b11110,
            Instruction::RandomNumberGenerator { .. } => 0b11111,
        }
    }

    pub fn to_binary_string(&self) -> String {
        mathi::int_to_binary_string(self.encode() as u64, 24)
    }

    pub fn to_hex_string(&self) -> String {
        mathi::int_to_hex_string(self.encode() as u64, 6)
    }

    pub fn to_assembly_string(&self) -> String {
        match self {
            Instruction::NoOperation { } => "NOP".to_string(),
            Instruction::Halt { } => "HLT".to_string(),
            Instruction::Add { reg_a, reg_b, reg_c } => format!("ADD r{} r{} r{}", reg_a, reg_b, reg_c),
            Instruction::Subtract { reg_a, reg_b, reg_c } => format!("SUB r{} r{} r{}", reg_a, reg_b, reg_c),
            Instruction::Multiply { reg_a, reg_b, reg_c } => format!("MUL r{} r{} r{}", reg_a, reg_b, reg_c),
            Instruction::Divide { reg_a, reg_b, reg_c } => format!("DIV r{} r{} r{}", reg_a, reg_b, reg_c),
            Instruction::Modulo { reg_a, reg_b, reg_c } => format!("MOD r{} r{} r{}", reg_a, reg_b, reg_c),
            Instruction::BitwiseAnd { reg_a, reg_b, reg_c } => format!("AND r{} r{} r{}", reg_a, reg_b, reg_c),
            Instruction::BitwiseNand { reg_a, reg_b, reg_c } => format!("NAND r{} r{} r{}", reg_a, reg_b, reg_c),
            Instruction::BitwiseOr { reg_a, reg_b, reg_c } => format!("OR r{} r{} r{}", reg_a, reg_b, reg_c),
            Instruction::BitwiseNor { reg_a, reg_b, reg_c } => format!("NOR r{} r{} r{}", reg_a, reg_b, reg_c),
            Instruction::BitwiseXor { reg_a, reg_b, reg_c } => format!("XOR r{} r{} r{}", reg_a, reg_b, reg_c),
            Instruction::BitwiseXnor { reg_a, reg_b, reg_c } => format!("XNOR r{} r{} r{}", reg_a, reg_b, reg_c),
            Instruction::BitwiseNot { reg_a, reg_b } => format!("NOT r{} r{}", reg_a, reg_b),
            Instruction::RightShift { reg_a, reg_b } => format!("RSH r{} r{}", reg_a, reg_b),
            Instruction::LeftShift { reg_a, reg_b } => format!("LSH r{}, r{}", reg_a, reg_b),
            Instruction::Roll { reg_a, reg_b, reg_c } => format!("ROL r{} r{} r{}", reg_a, reg_b, reg_c),
            Instruction::LoadImmediate { reg_a, immediate } => format!("LDI r{} {}", reg_a, immediate),
            Instruction::AddImmediate { reg_a, immediate } => format!("ADDI r{} {}", reg_a, immediate),
            Instruction::SubtractImmediate { reg_a, immediate } => format!("SUBI r{} {}", reg_a, immediate),
            Instruction::MultiplyImmediate { reg_a, immediate } => format!("MULI r{} {}", reg_a, immediate),
            Instruction::DivideImmediate { reg_a, immediate } => format!("DIVI r{} {}", reg_a, immediate),
            Instruction::Jump { address } => format!("JMP {}", address),
            Instruction::Branch { address } => format!("BRH {}", address),
            Instruction::Call { condition_flag, address } => format!("CALL {} {}", condition_flag, address),
            Instruction::Return => "RET".to_string(),
            Instruction::MemoryLoad { reg_a, reg_b, offset } => format!("MLD r{} r{} {}", reg_a, reg_b, offset),
            Instruction::MemoryStore { reg_a, reg_b, offset } => format!("MSTR r{} r{} {}", reg_a, reg_b, offset),
            Instruction::Draw { reg_x, reg_y, reg_rgb } => format!("DRW r{} r{} r{}", reg_x, reg_y, reg_rgb),
            Instruction::PushBuffer => "PSHB".to_string(),
            Instruction::ControllerPad { reg_a } => format!("PAD r{}", reg_a),
            Instruction::RandomNumberGenerator { reg_a } => format!("RNG r{}", reg_a),
        }
    }   
}