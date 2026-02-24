use std::io;

mod assembling;
mod parsing;
mod virtual_file;
mod killer;

use crate::virtual_file::VirtualFile;

fn main() {
    // C:\\Users\\lianh\\Development\\program.xis16

    let mut input = String::new();

    println!("File to compile:");

    let _ = io::stdin().read_line(&mut input);
    let path = input.trim();

    let instructions = parsing::parse_file(path);
    let machine_code = assembling::assemble(&instructions);

    let output: Vec<String> = machine_code
    .iter()
    .map(|arr| {
        arr.iter()
            .map(|b| format!("{:08b}", b))
            .collect::<Vec<_>>()
            .join(" ")
    })
    .collect();

    let vfile_out = VirtualFile::new(output);
    let _ = vfile_out.write_to_file("C:\\Users\\lianh\\Development\\output.c16");
}
