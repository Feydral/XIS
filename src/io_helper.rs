use std::fs::File;
use std::io::{BufRead, BufReader, Write, stdin, stdout};
use std::path::Path;

pub fn load_from_file(path: &str) -> Result<Vec<String>, std::io::Error> {
    let resolved_path = Path::new(path);

    let file = File::open(&resolved_path)
        .map_err(|e| {
            std::io::Error::new(
                e.kind(),
                format!("Failed to open file '{}': {}", resolved_path.display(), e),
            )
        })?;

    let reader = BufReader::new(file);

    let lines = reader.lines()
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| {
            std::io::Error::new(
                e.kind(),
                format!("Failed to read from file '{}': {}", resolved_path.display(), e),
            )
        })?;

    Ok(lines)
}

pub fn write_to_file(path: &str, data: Vec<String>) -> Result<(), std::io::Error> {
    let resolved_path = Path::new(path);

    let mut file = File::create(&resolved_path)
        .map_err(|e| {
            std::io::Error::new(
                e.kind(),
                format!("Failed to create file '{}': {}", resolved_path.display(), e),
            )
        })?;

    for line in &data {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().expect("Console-flushing failed.");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Console input failed.");

    input
}