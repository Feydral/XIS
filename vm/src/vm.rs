use std::error::Error;

use common::{errors::RuntimeError, hardware, instruction::Instruction, io_helper};

use crate::{parser, window::Window};

pub struct VirtualMachine {
    pub window: Window,

    pub instructions: Vec<Instruction>,
    pub carry_flag: bool,
    pub zero_flag: bool,
    pub overflow_flag: bool,
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

            instructions,
            carry_flag: false,
            zero_flag: false,
            overflow_flag: false,
            memory: [0_16; 1024],
            pc: 0,
        })
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        todo!()
    }
}