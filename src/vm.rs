use crate::{errors::RuntimeError, instruction::Instruction, vm::{virtual_program::VirtualProgram, window::Window}};

pub mod window;
pub mod virtual_program;
pub mod parser;

pub struct VirtualMachine {
    pub window: Window,
    pub virtual_program: VirtualProgram,
}

impl VirtualMachine {
    pub fn new(instructions: Vec<Instruction>) -> Result<Self, RuntimeError> {
        Ok(Self {
            window: Window::new(),
            virtual_program: VirtualProgram::new(instructions)?,
        })
    }
}