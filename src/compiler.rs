use crate::errors::CompileError;

pub mod syntax_helper;
pub mod assembler;


pub fn compile(source: &str) -> Result<Vec<u8>, CompileError> {
    let mut bytecode = Vec::new();

    for (ln, line) in source.lines().enumerate() {
        let instruction = syntax_helper::parse_line(line, ln)?;
    }

    Ok(bytecode)
}