use std::fs::File;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;

pub struct VirtualFile {
    pub data: Vec<String>,
    marker: PhantomData<()>
}

impl VirtualFile {
    pub fn new(data: Vec<String>) -> Self {
        Self {
            data,
            marker: PhantomData,
        }
    }

    pub fn load_from_file(path: &str) -> Self {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                return Self::new(Vec::new());
            }
        };
    
        let reader = BufReader::new(file);
    
        let lines = match reader.lines().collect::<Result<Vec<String>, _>>() {
            Ok(v) => v,
            Err(_) => {
                return Self::new(Vec::new());
            }
        };

        Self::new(lines)
    }

    pub fn write_to_file(&self, path: &str) -> std::io::Result<()> {
        use std::io::Write;
        let mut file = File::create(path)?;
        for line in &self.data {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}

