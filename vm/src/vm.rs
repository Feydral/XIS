use std::error::Error;

use common::errors::RuntimeError;

use crate::{virtual_program::VirtualProgram, window::Window};

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

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        todo!()
    }
}