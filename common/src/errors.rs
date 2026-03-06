use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ParseError {
    message: String,
    line: usize,
}

impl ParseError {
    pub fn new(message: impl Into<String>, line: usize) -> Self {
        Self { message: message.into(), line }
    }
}

impl Error for ParseError {
    
}
    
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at line {}: {}", self.line, self.message)
    }
}

#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
    pub line: usize,
}

impl SyntaxError {
    pub fn new(message: impl Into<String>, line: usize) -> Self {
        Self { message: message.into(), line }
    }
}

impl Error for SyntaxError {
    
}
    
impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid syntax at line {}: {}", self.line, self.message)
    }
}



#[derive(Debug)]
pub struct PreprocessError {
    pub message: String,
    pub line: usize,
}

impl PreprocessError {
    pub fn new(message: impl Into<String>, line: usize) -> Self {
        Self { message: message.into(), line }
    }
}

impl Error for PreprocessError {
    
}
    
impl Display for PreprocessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Preprocess error at line {}: {}", self.line, self.message)
    }
}



#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl Error for RuntimeError {
    
}
    
impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error appeared: {}", self.message)
    }
}


#[derive(Debug)]
pub struct BinaryDecodeError {
    pub message: String,
    pub line: usize,
}

impl BinaryDecodeError {
    pub fn new(message: impl Into<String>, line: usize) -> Self {
        Self { message: message.into(), line }
    }
}

impl Error for BinaryDecodeError {
    
}
    
impl Display for BinaryDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Decode error at line {}: {}", self.line, self.message)
    }
}