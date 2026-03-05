use std::collections::HashMap;

pub fn process_program(input: Vec<String>) -> Vec<String> {
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut instructions: Vec<String> = Vec::new();

    for line in &input {
        let clean = if let Some(pos) = line.find("//") {
            line[..pos].trim().to_string()
        } else {
            line.trim().to_string()
        };

        if clean.is_empty() {
            continue;
        }

        if clean.starts_with('.') {
            let mut parts = clean.split_whitespace();
            let label_name = parts.next().unwrap_or(&clean).to_string();

            if parts.next().is_some() {
                continue;
            }

            labels.insert(label_name, instructions.len());
            continue;
        }

        instructions.push(clean);
    }

    instructions
        .into_iter()
        .map(|line| {
            let mut result = line.clone();
            for (label, addr) in &labels {
                result = result.replace(label.as_str(), &addr.to_string());
            }
            result
        })
        .collect()
}