use std::error::Error;

use common::{errors::RuntimeError, hardware, instruction::Instruction, io_helper};

use crate::{component::{ArithmeticLogicUnit, CallStack, InstructionMemory, RegisterFile}, parser, window::Window};

pub struct VirtualMachine {
    pub window: Window,

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
            window: Window::new(),

            instruction_mem: InstructionMemory::new(instructions),
            alu: ArithmeticLogicUnit::new(),
            register_file: RegisterFile::new(),
            call_stack: CallStack::new(),
            memory: [0_16; 1024],
            pc: 0,
        })
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        todo!()
    }

pub fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), RuntimeError> {
    match instruction {
        Instruction::NoOperation => { }

        Instruction::Halt => {
            return Err(RuntimeError::new("Program stopped"));
        }

        Instruction::Add { reg_a, reg_b, reg_c } => {
            let a = self.register_file[reg_a.into()];
            let b = self.register_file[reg_a.into()];
            self.register_file[reg_c.into()] = self.alu.add(a, b)
        }

        Instruction::Subtract { reg_a, reg_b, reg_c } => {
            let a = self.register_file[reg_a.into()];
            let b = self.register_file[reg_a.into()];
            self.register_file[reg_c.into()] = self.alu.sub(a, b)
        }

        Instruction::Multiply { reg_a, reg_b, reg_c } => {
            let a = self.register_file[reg_a.into()];
            let b = self.register_file[reg_a.into()];
            self.register_file[reg_c.into()] = self.alu.mul(a, b)
        }

        Instruction::Divide { reg_a, reg_b, reg_c } => {
            let a = self.register_file[reg_a.into()];
            let b = self.register_file[reg_a.into()];
            self.register_file[reg_c.into()] = self.alu.div(a, b)
        }

        Instruction::Modulo { reg_a, reg_b, reg_c } => {
            let a = self.register_file[reg_a.into()];
            let b = self.register_file[reg_a.into()];
            self.register_file[reg_c.into()] = self.alu.modulo(a, b)
        }

        Instruction::BitwiseAnd { reg_a, reg_b, reg_c } => {
            let a = self.register_file[reg_a.into()];
            let b = self.register_file[reg_a.into()];
            self.register_file[reg_c.into()] = self.alu.and(a, b)
        }

        Instruction::BitwiseNand { reg_a, reg_b, reg_c } => {
            let a = self.register_file[reg_a.into()];
            let b = self.register_file[reg_a.into()];
            self.register_file[reg_c.into()] = self.alu.nand(a, b)
        }

        Instruction::BitwiseOr { reg_a, reg_b, reg_c } => {
            let a = self.register_file[reg_a.into()];
            let b = self.register_file[reg_a.into()];
            self.register_file[reg_c.into()] = self.alu.or(a, b)
        }

        Instruction::BitwiseNor { reg_a, reg_b, reg_c } => {
            let a = self.register_file[reg_a.into()];
            let b = self.register_file[reg_a.into()];
            self.register_file[reg_c.into()] = self.alu.nor(a, b)
        }

        Instruction::BitwiseXor { reg_a, reg_b, reg_c } => {
            let a = self.register_file[reg_a.into()];
            let b = self.register_file[reg_a.into()];
            self.register_file[reg_c.into()] = self.alu.xor(a, b)
        }

        Instruction::BitwiseXnor { reg_a, reg_b, reg_c } => {
            let a = self.register_file[reg_a.into()];
            let b = self.register_file[reg_a.into()];
            self.register_file[reg_c.into()] = self.alu.xnor(a, b)
        }

        Instruction::BitwiseNot { reg_a, reg_c } => {
            let a = self.register_file[reg_a.into()];
            self.register_file[reg_c.into()] = self.alu.not(a)
        }

        Instruction::RightShift { reg_a, reg_c } => {
            let a = self.register_file[reg_a.into()];
            self.register_file[reg_c.into()] = self.alu.rsh(a)
        }

        Instruction::LeftShift { reg_a, reg_c } => {
        
        }

        Instruction::Roll { reg_a, reg_b, reg_c } => {
        
        }

        Instruction::LoadImmediate { reg_a, immediate } => {
        
        }

        Instruction::AddImmediate { reg_a, immediate } => {
        
        }

        Instruction::SubtractImmediate { reg_a, immediate } => {
        
        }

        Instruction::MultiplyImmediate { reg_a, immediate } => {
        
        }

        Instruction::DivideImmediate { reg_a, immediate } => {
        
        }

        Instruction::Jump { address } => {
        
        }

        Instruction::Branch { condition_flag, address } => {
        
        }

        Instruction::Call { address } => {
        
        }

        Instruction::Return => {
        
        }

        Instruction::MemoryLoad { reg_a, reg_b, offset } => {
        
        }

        Instruction::MemoryStore { reg_a, reg_b, offset } => {
        
        }

        Instruction::Draw { reg_x, reg_y, reg_rgb } => {
        
        }

        Instruction::PushBuffer => {
        
        }

        Instruction::ControllerPad { reg_a } => {
        
        }

        Instruction::RandomNumberGenerator { reg_a } => {
        
        }
    }

    Ok(())
}
}