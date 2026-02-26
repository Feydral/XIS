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
        write!(f, "Parse error at line {}: {}", self.line, self.message)
    }
}



#[derive(Debug)]
pub struct FileError {
    pub message: String,
    pub file_path: usize,
}

impl FileError {
    pub fn new(message: impl Into<String>, file_path: usize) -> Self {
        Self { message: message.into(), file_path }
    }
}

impl Error for FileError {
    
}
    
impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File error (path: {}): {}", self.file_path, self.message)
    }
}