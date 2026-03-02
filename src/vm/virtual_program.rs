use std::error::Error;

use crate::{errors::{BinaryDecodeError, RuntimeError}, hardware, instruction::{self, Instruction}, io_helper, vm::parser};

pub struct VirtualProgram {
    pub carry_flag: bool,
    pub zero_flag: bool,
    pub overflow_flag: bool,

    pub memory: [u16; 1024],

    pub instructions: Vec<Instruction>,

    pub pc: u16,
}

impl VirtualProgram {
    pub fn new(path: impl AsRef<str>) -> Result<Self, Box<dyn Error>> {
        let lines = io_helper::read_from_file(path.as_ref())?;

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
            carry_flag: false,
            zero_flag: false,
            overflow_flag: false,
            memory: [0u16; 1024],
            instructions,
            pc: 0,
        })
    }
 
    pub fn advance_clock(&mut self) -> Result<(), RuntimeError> {
        if self.pc >= hardware::INSTRUCTION_MEM_SIZE as u16 {
            self.pc = 0;
        }

        let instruction = if (self.pc as usize) < self.instructions.len() {
            self.instructions[self.pc as usize]
        } else {
            Instruction::NoOperation
        };

        self.execute_instruction(instruction)?;
        self.pc += 1;

        Ok(())
    }

    pub fn execute_instruction(&self, instruction: Instruction) -> Result<(), RuntimeError> {
        todo!()
    }
}