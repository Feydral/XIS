#[derive(Debug)]
pub enum Error {
    ParseErrorInvalidRegister(String),
    ParseErrorInvalidImmediate(String),
    ParseErrorInvalidInstructionAddress(String),
    ParseErrorInvalidOffset(String),
    ParseErrorInvalidFlag(String),
    ParseErrorWrongOperandCount { mnemonic: String, expected: usize, given: usize },
    // ParseErrorWrongOperandType { mnemonic: String, expected: OperandType, given: String },
    ParseErrorUnknownMnemonic(String),
    
    // FileReadError(String),
}

pub fn kill(error: Error) {
    match error {
        Error::ParseErrorInvalidRegister(s) => {
            println!("parse error: invalid register '{}'.", s);
        }
        Error::ParseErrorInvalidImmediate(s) => {
            println!("parse error: invalid immediate value '{}'.", s);
        }
        Error::ParseErrorInvalidInstructionAddress(s) => {
            println!("parse error: invalid instruction address '{}'.", s);
        }
        Error::ParseErrorInvalidOffset(s) => {
            println!("parse error: invalid offset value '{}'.", s);
        }
        Error::ParseErrorInvalidFlag(s) => {
            println!("parse error: invalid flag value '{}'.", s);
        }
        Error::ParseErrorWrongOperandCount { mnemonic, expected, given } => {
            println!("parse error: wrong operand count for '{}'. expected: {}, given: {}.", mnemonic, expected, given);
        }
        // Error::ParseErrorWrongOperandType { mnemonic, expected, given } => {
        //     println!("parse error: wrong operand type for '{}'. expected: {:?}, given: '{}'.", mnemonic, expected, given);
        // }
        Error::ParseErrorUnknownMnemonic(s) => {
            println!("parse error: unknown mnemonic '{}'.", s);
        }
        // Error::FileReadError(s) => {
        //     println!("file read error: {}", s);
        // }
    }
    std::process::exit(0);
}