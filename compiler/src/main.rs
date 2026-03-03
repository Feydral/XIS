use common::io_helper;

pub mod code_gen;
pub mod compiler;
pub mod parser;

fn main() {
    let default = io_helper::read_line("Use default? ");
    if default.trim() == "y" {
        let input_file_trimmed = "../xis/examples/program.xis16";
        let output_file_trimmed = "../xis/examples/out.c16";
        let format_trimmed = "binary";

        // Read file content
        let file_content = match io_helper::read_from_file(input_file_trimmed) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", input_file_trimmed, e);
                return;
            }
        };

        let compiled_result = match format_trimmed.to_string().as_str() {
            "binary" => compiler::compile(&file_content, compiler::OutputFormat::Binary),
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

        return;
    }

    let input_file = io_helper::read_line("File to compile: ");
    let input_file_trimmed = input_file.trim();

    let output_file = io_helper::read_line("Output directory: ");
    let output_file_trimmed = output_file.trim();

    let format = io_helper::read_line("Format (binary, hexadecimal or assembly): ");
    let format_trimmed = format.trim();

    // Read file content
    let file_content = match io_helper::read_from_file(&input_file_trimmed) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", input_file_trimmed, e);
            return;
        }
    };

    // Compile
    let compiled_result = match format_trimmed.to_string().as_str() {
        "binary" => compiler::compile(&file_content, compiler::OutputFormat::Binary),
        "hexadecimal" | "hex" => compiler::compile(&file_content, compiler::OutputFormat::Hexadecimal),
        "assembly" | "asm" => compiler::compile(&file_content, compiler::OutputFormat::Assembly),
        _ => {
            eprintln!("Unknown format: '{}'", format);
            return;
        }
    };

    // Collect errors and successful lines
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

    // Write output only if no errors
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