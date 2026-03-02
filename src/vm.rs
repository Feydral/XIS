use std::error::Error;

use crate::{errors::RuntimeError, instruction::Instruction, vm::{virtual_program::VirtualProgram, window::Window}};

pub mod window;
pub mod virtual_program;
pub mod parser;

pub struct VirtualMachine {
    pub window: Window,
    pub virtual_program: VirtualProgram,
}

impl VirtualMachine {
    pub fn new(path: impl Into<String>) -> Result<Self, Box<dyn Error>> {
        let program = VirtualProgram::new(path.into())?;

        Ok(Self {
            window: Window::new(),
            virtual_program: program,
        })
    }

    pub fn new_from_program(path: impl Into<String>) -> Result<Self, RuntimeError> {
        todo!()
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        todo!()
    }
}