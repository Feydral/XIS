use crate::compiler::OutputFormat;

mod vm;
mod compiler;
mod math;
mod io_helper;
mod errors;
mod instruction;
mod hardware;

fn main() {
    repl();
}

fn repl() {
    loop {
        let input = io_helper::read_line("xis>");
        let line = input.trim();

        if line.is_empty() {
            continue;
        }

        if line == "exit" || line == "quit" {
            break;
        }

        if let Err(e) = handle_command(&line) {
            eprintln!("Error: {e}");
        }
    }
}

fn handle_command(input: &str) -> Result<(), String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Ok(());
    }

    match parts[0] {
        "compile" => handle_compile(&parts[1..]),
        _ => Err(format!("Unknown command: {}", parts[0])),
    }
}

fn handle_compile(args: &[&str]) -> Result<(), String> {
    if args.len() != 5 {
        return Err(
            "Usage: compile <input> to <output> as <binary|hex|assembly>".into()
        );
    }

    let input_path = args[0];

    if args[1] != "to" {
        return Err("Expected 'to' after input".into());
    }

    let output_path = args[2];

    if args[3] != "as" {
        return Err("Expected 'as' after output".into());
    }

    let format = match args[4] {
        "binary" => OutputFormat::Binary,
        "hex" => OutputFormat::Hex,
        "assembly" => OutputFormat::Assembly,
        other => return Err(format!("Unknown format: {other}")),
    };

    let source = io_helper::load_from_file(input_path)
        .map_err(|e| format!("IO error while loading file: {e}"))?;

    let output = compiler::compile(&source, format)
        .map_err(|e| format!("{e}"))?;

    io_helper::write_to_file(output_path, output)
        .map_err(|e| format!("IO error while writing file: {e}"))?;

    println!("Compilation successful.");

    Ok(())
}