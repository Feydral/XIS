use common::io_helper;

pub mod code_gen;
pub mod compiler;
pub mod parser;
pub mod preprocessor;

fn main() {
    let default = io_helper::read_line("Use default paths and settings? (Y|N) ");

    let mut input_file_trimmed = "../xis/examples/program.xis16".to_string();
    let mut output_file_trimmed = "../xis/examples/out.c16".to_string();
    let mut format_trimmed = "binary".to_string();

    if default.trim().to_lowercase() == "n" {
        let input_file = io_helper::read_line("File to compile: ");
        input_file_trimmed = input_file.trim().to_string();

        let output_file = io_helper::read_line("Output directory: ");
        output_file_trimmed = output_file.trim().to_string();

        let format = io_helper::read_line("Format (binary, hexadecimal or assembly): ");
        format_trimmed = format.trim().to_string();
    } 

    let file_content = match io_helper::read_from_file(&input_file_trimmed) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file_trimmed, e);
            return;
        }
    };

    let compiled_result = match format_trimmed.to_string().as_str() {
        "binary" | "bin" => compiler::compile(&file_content, compiler::OutputFormat::Binary),
        "hexadecimal" | "hex" => compiler::compile(&file_content, compiler::OutputFormat::Hexadecimal),
        "assembly" | "asm" => compiler::compile(&file_content, compiler::OutputFormat::Assembly),
        _ => {
            eprintln!("Unknown format: '{}'", format_trimmed);
            return;
        }
    };

    let mut output = Vec::new();
    let mut has_errors = false;

    for line in compiled_result {
        match line {
            Ok(code_line) => output.push(code_line),
            Err(err_msg) => {
                eprintln!("{}", err_msg);
                has_errors = true;
            }
        }
    }

    if !has_errors {
        if let Err(e) = io_helper::write_to_file(&output_file_trimmed, output) {
            eprintln!("Error writing output file '{}': {}", output_file_trimmed, e);
        } else {
            println!("Compilation successful");
        }
    } else {
        eprintln!("Compilation aborted due to errors.");
    }
}