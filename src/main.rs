use crate::compiler::OutputFormat;

mod vm;
mod compiler;
mod math;
mod io_helper;
mod errors;
mod instruction;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = io_helper::load_from_file("../xis/examples/program.xis16")?;
    let output = compiler::compile(&source, OutputFormat::Binary)?;

    for line in output {
        println!("{}", line);
    }

    Ok(())
}
