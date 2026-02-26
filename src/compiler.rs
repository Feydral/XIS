use crate::errors::CompileError;

pub mod syntax_helper;
pub mod assembler;


pub fn compile(source: &[String], format: OutputFormat) -> Result<Vec<String>, CompileError> {
    let mut output = Vec::new();

    for (ln, line) in source.iter().enumerate() {
        if line.trim().is_empty() || line.trim_start().starts_with("//") {
            continue;
        }

        let instruction = syntax_helper::parse_line(line, ln)?;

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
            OutputFormat::Instructions => {
                output.push(assembler::generate_instruction_code(instruction));
            }
        }
    }

    Ok(output)
}

pub enum OutputFormat {
    Binary,
    Hex,
    Assembly,
    Instructions,
}