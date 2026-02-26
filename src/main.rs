use crate::compiler::OutputFormat;

mod vm;
mod compiler;
mod math;
mod io_helper;
mod errors;
mod instruction;

fn main() {
    let source = match io_helper::load_from_file("../xis/examples/program.xis16") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("IO error while loading file: {e}");
            return;
        }
    };

    let output = match compiler::compile(&source, OutputFormat::Assembly) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Compile error: {e}");
            return;
        }
    };

    let _ = io_helper::write_to_file("../xis/examples/output/output.c16", output);
}