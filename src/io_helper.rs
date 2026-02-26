use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_from_file(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return Err(e);
        }
    };

    let reader = BufReader::new(file);

    let lines = match reader.lines().collect::<Result<Vec<String>, _>>() {
        Ok(v) => v,
        Err(e) => {
            return Err(e);
        }
    };

    Ok(lines)
}

pub fn write_to_file(path: &str, data: Vec<String>) -> std::io::Result<()> {
    use std::io::Write;
    let mut file = File::create(path)?;
    for line in &data {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}