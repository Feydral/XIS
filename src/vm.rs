use crate::vm::{virtual_program::VirtualProgram, window::Window};

pub mod window;
pub mod virtual_program;
pub mod parser;

pub struct VirtualMachine {
    pub window: Window,
    pub virtual_program: VirtualProgram,
}