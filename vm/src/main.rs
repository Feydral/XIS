use crate::{vm::VirtualMachine, window::Window};

mod parser;
mod vm;
mod window;
mod component;

pub fn main() {
    let mut window = Window::new();
    let mut vm = Box::new(VirtualMachine::new("../xis/examples/out.c16").unwrap());

    while window.is_open() {
        if vm.execute_next_instruction(&mut window) {
            return;
        }
        window.update_buffer();
    }
}