use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct CompileError {
    pub message: String,
    pub line: usize,
}

impl CompileError {
    pub fn new(message: impl Into<String>, line: usize) -> Self {
        Self { message: message.into(), line }
    }
}

impl Error for CompileError {
    
}
    
impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Compile error at line {}: {}", self.line, self.message)
    }
}