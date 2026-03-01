use crate::{errors::RuntimeError, instruction::Instruction, vm::{virtual_program::VirtualProgram, window::Window}};

pub mod window;
pub mod virtual_program;
pub mod parser;

pub struct VirtualMachine {
    pub window: Window,
    pub header: Vec<String>,
    pub virtual_program: VirtualProgram,
}

impl VirtualMachine {
    pub fn new(header: Vec<String>, instructions: Vec<Instruction>) -> Result<Self, RuntimeError> {
        Ok(Self {
            window: Window::new(),
            header: header.clone(),
            virtual_program: VirtualProgram::new(header, instructions)?,
        })
    }

    pub fn new_from_program(path: impl Into<String>) -> Result<Self, RuntimeError> {
        todo!()
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        todo!()
    }
}