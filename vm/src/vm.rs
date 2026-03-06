use std::error::Error;

use common::{errors::RuntimeError, hardware::{self, CARRY_FLAG_BINARY, INSTRUCTION_MEM_SIZE, OVERFLOW_FLAG_BINARY, SCREEN_HEIGHT, SCREEN_WIDTH, ZERO_FLAG_BINARY}, instruction::Instruction, io_helper, math::mathi::{self}};

use crate::{component::{ArithmeticLogicUnit, CallStack, InstructionMemory, RegisterFile}, parser, window::Window};

pub struct VirtualMachine {
    pub buffer: Vec<u16>,

    pub instruction_mem: InstructionMemory,
    pub alu: ArithmeticLogicUnit,
    pub register_file: RegisterFile,
    pub call_stack: CallStack,
    pub memory: [u16; 1024],
    pub pc: u16,
}

impl VirtualMachine {
    pub fn new(path: impl Into<String>) -> Result<Self, Box<dyn Error>> {
        let lines = io_helper::read_from_file(path.into().as_str())?;

        let mut format = "";

        for raw_line in &lines {
            let line = raw_line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with('!') {
                match line {
                    "!notrunable" => {
                        return Err(Box::new(RuntimeError::new(
                            "Tried running a program marked with !notrunable",
                        )));
                    }
                    "!format [c16 binary]" => {
                        format = "b";
                        break;
                    }
                    "!format [c16 hexadecimal]" => {
                        format = "h";
                        break;
                    }
                    _ => {
                        return Err(Box::new(RuntimeError::new(
                            "Unknown format directive",
                        )));
                    }
                }
            }
        }

        let mut instructions = Vec::new();

        for (ln, raw_line) in lines.iter().enumerate() {
            let line = raw_line.trim();

            if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
                continue;
            }

            let instruction = match format {
                "b" => parser::parse_binary_line(line, ln)?,
                "h" => parser::parse_hexadecimal_line(line, ln)?,
                _ => unreachable!(),
            };

            instructions.push(instruction);

            if instructions.len() == hardware::INSTRUCTION_MEM_SIZE as usize {
                break;
            }
        }

        Ok(Self {
            buffer: vec![0_u16; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],

            instruction_mem: InstructionMemory::new(instructions),
            alu: ArithmeticLogicUnit::new(),
            register_file: RegisterFile::new(),
            call_stack: CallStack::new(),
            memory: [0_16; 1024],
            pc: 0,
        })
    }

    pub fn execute_next_instruction(&mut self, window: &mut Window) -> bool {
        let instruction = self.instruction_mem[self.pc as usize];
        if self.pc == (INSTRUCTION_MEM_SIZE - 1) as u16 {
            self.pc = 0
        } else {
            self.pc += 1;
        }

        match instruction {
            Instruction::NoOperation => { }

            Instruction::Halt => { return true; }

            Instruction::Add { reg_a, reg_b, reg_c } => {
                let a = self.register_file[reg_a.into()];
                let b = self.register_file[reg_b.into()];
                self.register_file[reg_c.into()] = self.alu.add(a, b)
            }

            Instruction::Subtract { reg_a, reg_b, reg_c } => {
                let a = self.register_file[reg_a.into()];
                let b = self.register_file[reg_b.into()];
                self.register_file[reg_c.into()] = self.alu.sub(a, b)
            }

            Instruction::Multiply { reg_a, reg_b, reg_c } => {
                let a = self.register_file[reg_a.into()];
                let b = self.register_file[reg_b.into()];
                self.register_file[reg_c.into()] = self.alu.mul(a, b)
            }

            Instruction::Divide { reg_a, reg_b, reg_c } => {
                let a = self.register_file[reg_a.into()];
                let b = self.register_file[reg_b.into()];
                self.register_file[reg_c.into()] = self.alu.div(a, b)
            }

            Instruction::Modulo { reg_a, reg_b, reg_c } => {
                let a = self.register_file[reg_a.into()];
                let b = self.register_file[reg_b.into()];
                self.register_file[reg_c.into()] = self.alu.modulo(a, b)
            }

            Instruction::BitwiseAnd { reg_a, reg_b, reg_c } => {
                let a = self.register_file[reg_a.into()];
                let b = self.register_file[reg_b.into()];
                self.register_file[reg_c.into()] = self.alu.and(a, b)
            }

            Instruction::BitwiseNand { reg_a, reg_b, reg_c } => {
                let a = self.register_file[reg_a.into()];
                let b = self.register_file[reg_b.into()];
                self.register_file[reg_c.into()] = self.alu.nand(a, b)
            }

            Instruction::BitwiseOr { reg_a, reg_b, reg_c } => {
                let a = self.register_file[reg_a.into()];
                let b = self.register_file[reg_b.into()];
                self.register_file[reg_c.into()] = self.alu.or(a, b)
            }

            Instruction::BitwiseNor { reg_a, reg_b, reg_c } => {
                let a = self.register_file[reg_a.into()];
                let b = self.register_file[reg_b.into()];
                self.register_file[reg_c.into()] = self.alu.nor(a, b)
            }

            Instruction::BitwiseXor { reg_a, reg_b, reg_c } => {
                let a = self.register_file[reg_a.into()];
                let b = self.register_file[reg_b.into()];
                self.register_file[reg_c.into()] = self.alu.xor(a, b)
            }

            Instruction::BitwiseXnor { reg_a, reg_b, reg_c } => {
                let a = self.register_file[reg_a.into()];
                let b = self.register_file[reg_b.into()];
                self.register_file[reg_c.into()] = self.alu.xnor(a, b)
            }

            Instruction::BitwiseNot { reg_a, reg_c } => {
                let a = self.register_file[reg_a.into()];
                self.register_file[reg_c.into()] = self.alu.not(a)
            }

            Instruction::RightShift { reg_a, reg_c } => {
                let a = self.register_file[reg_a.into()];
                self.register_file[reg_c.into()] = self.alu.rsh(a);
            }
        
            Instruction::LeftShift { reg_a, reg_c } => {
                let a = self.register_file[reg_a.into()];
                self.register_file[reg_c.into()] = self.alu.lsh(a);
            }
        
            Instruction::Roll { reg_a, reg_b, reg_c } => {
                let a = self.register_file[reg_a.into()];
                let b = self.register_file[reg_b.into()];
                self.register_file[reg_c.into()] = self.alu.rol(a, b);
            }

            Instruction::LoadImmediate { reg_a, immediate } => {
                self.register_file[reg_a.into()] = immediate;
            }
        
            Instruction::AddImmediate { reg_a, immediate } => {
                let a = self.register_file[reg_a.into()];
                self.register_file[reg_a.into()] = self.alu.add(a, immediate);
            }
        
            Instruction::SubtractImmediate { reg_a, immediate } => {
                let a = self.register_file[reg_a.into()];
                self.register_file[reg_a.into()] = self.alu.sub(a, immediate);
            }
        
            Instruction::MultiplyImmediate { reg_a, immediate } => {
                let a = self.register_file[reg_a.into()];
                self.register_file[reg_a.into()] = self.alu.mul(a, immediate);
            }
        
            Instruction::DivideImmediate { reg_a, immediate } => {
                let a = self.register_file[reg_a.into()];
                self.register_file[reg_a.into()] = self.alu.div(a, immediate);
            }

            Instruction::Jump { address } => {
                self.pc = address;
            }

            Instruction::Branch { condition_flag, address } => {
                match condition_flag as u32 {
                    ZERO_FLAG_BINARY => if self.alu.zero_flag() { self.pc = address; },
                    CARRY_FLAG_BINARY => if self.alu.carry_flag() { self.pc = address; },
                    OVERFLOW_FLAG_BINARY => if self.alu.overflow_flag() { self.pc = address; },
                    _ => unreachable!()
                }
            }

            Instruction::Call { address } => {
                self.pc = address;
                self.call_stack.push(address + 1);
            }

            Instruction::Return => {
                self.pc = self.call_stack.pop();
            }

            Instruction::MemoryLoad { reg_a, reg_b, offset } => {
                let address = self.register_file[reg_a.into()].wrapping_add(offset as u16) as usize;
                self.register_file[reg_b.into()] = self.memory[address];
            }

            Instruction::MemoryStore { reg_a, reg_b, offset } => {
                let address = self.register_file[reg_a.into()].wrapping_add(offset as u16) as usize;
                self.memory[address] = self.register_file[reg_b.into()];
            }

            Instruction::Draw { reg_x, reg_y, reg_rgb } => {
                let x = self.register_file[reg_x.into()] as u32;
                let y = self.register_file[reg_y.into()] as u32;
                let index = mathi::xy_to_index(x, y, SCREEN_WIDTH, SCREEN_HEIGHT) as usize;
                let color = self.register_file[reg_rgb.into()];
                self.buffer[index] = color;
            }

            Instruction::PushBuffer => {
                for (index, color) in self.buffer.iter().enumerate() {
                    let pos = mathi::index_to_xy(index as u32, SCREEN_WIDTH, SCREEN_HEIGHT);
                    window.set_pixel(pos.x, pos.y, *color);
                }
                self.buffer.fill(0);
            }

            Instruction::ControllerPad { reg_a } => {
                let _ = reg_a;
                todo!();
            }

            Instruction::RandomNumberGenerator { reg_a } => {
                let mut x = (self.pc as u32) ^ ((reg_a as u32) << 16) ^ (self.register_file[reg_a.into()] as u32);

                x = x.wrapping_add(0x9E3779B9);
                x ^= x >> 16;
                x = x.wrapping_mul(0x85EBCA6B);
                x ^= x >> 13;
                x = x.wrapping_mul(0xC2B2AE35);
                x ^= x >> 16;

                self.register_file[reg_a.into()] = (x & 0xFFFF) as u16;
            }
        }

        false
    }
}