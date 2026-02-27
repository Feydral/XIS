use std::error::Error;

use crate::errors::CompileError;

pub mod parser;
pub mod assembler;


pub fn compile(source: &[String], format: OutputFormat) -> Result<Vec<String>, Box<dyn Error>> {
    let mut output = Vec::new();

    for (ln, line) in source.iter().enumerate() {
        if line.trim().is_empty() || line.trim_start().starts_with("//") {
            continue;
        }

        let instruction = parser::parse_line(line, ln)?;

        match format {
            OutputFormat::Binary => {
                output.push(assembler::generate_binary_code(instruction));
            }
            OutputFormat::Hex => {
                output.push(assembler::generate_hex_code(instruction));
            }
            OutputFormat::Assembly => {
                output.push(assembler::generate_assembly_code(instruction));
            }
        }
    }

    Ok(output)
}

pub enum OutputFormat {
    Binary,
    Hex,
    Assembly,
}