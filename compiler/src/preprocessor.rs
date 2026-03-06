use std::collections::HashMap;

use common::errors::PreprocessError;

pub fn process_program(input: Vec<String>) -> Result<Vec<(usize, String)>, PreprocessError> {
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut defines: HashMap<String, String> = HashMap::new();
    let mut instruction_count: usize = 0;

    for (line_idx, line) in input.iter().enumerate() {
        let line_num = line_idx + 1;
        let clean = strip_comment(line);

        if clean.is_empty() {
            continue;
        }

        if clean.starts_with('#') {
            let parts: Vec<&str> = clean.splitn(4, char::is_whitespace)
                .filter(|s| !s.is_empty())
                .collect();

            match parts.as_slice() {
                ["#define", name, value] => {
                    validate_define_name(name, line_num)?;
                    defines.insert(name.to_string(), value.to_string());
                }
                [directive, ..] if *directive == "#define" => {
                    return Err(PreprocessError::new(
                        format!("Invalid #define syntax '{}': expected '#define <name> <value>'", clean),
                        line_num,
                    ));
                }
                [directive, ..] => {
                    return Err(PreprocessError::new(
                        format!("Unknown directive '{}'", directive),
                        line_num,
                    ));
                }
                [] => unreachable!(),
            }
            continue;
        }

        if clean.starts_with('.') {
            let mut parts = clean.split_whitespace();
            let label_name = parts.next().unwrap_or(&clean).to_string();
            if parts.next().is_some() {
                return Err(PreprocessError::new(
                    format!("Invalid label '{}': labels must be a single word without whitespace", clean),
                    line_num,
                ));
            }
            labels.insert(label_name, instruction_count);
            continue;
        }

        instruction_count += 1;
    }

    let mut output: Vec<(usize, String)> = Vec::new();

    for (line_idx, line) in input.iter().enumerate() {
        let line_num = line_idx + 1;
        let clean = strip_comment(line);

        if clean.is_empty() || clean.starts_with('#') || clean.starts_with('.') {
            continue;
        }

        let mut result = clean;

        for (name, value) in &defines {
            result = replace_whole_word(&result, name, value);
        }
        for (label, addr) in &labels {
            result = replace_whole_word(&result, label, &addr.to_string());
        }

        for token in result.split_whitespace() {
            if token.starts_with('.') {
                return Err(PreprocessError::new(
                    format!("Undefined label '{}': label was never declared", token),
                    line_num,
                ));
            }
        }

        output.push((line_num, result));
    }

    Ok(output)
}

fn strip_comment(line: &str) -> String {
    if let Some(pos) = line.find("//") {
        line[..pos].trim().to_string()
    } else {
        line.trim().to_string()
    }
}

fn validate_define_name(name: &str, line_num: usize) -> Result<(), PreprocessError> {
    let mut chars = name.chars();
    let first_ok = chars.next()
        .map(|c| c.is_alphabetic() || c == '_')
        .unwrap_or(false);

    if !first_ok {
        return Err(PreprocessError::new(
            format!("Invalid #define name '{}': must start with a letter or '_'", name),
            line_num,
        ));
    }
    if !chars.all(|c| c.is_alphanumeric() || c == '_') {
        return Err(PreprocessError::new(
            format!("Invalid #define name '{}': only alphanumeric characters and '_' allowed", name),
            line_num,
        ));
    }
    Ok(())
}

fn replace_whole_word(source: &str, word: &str, replacement: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let mut rest = source;

    while let Some(pos) = rest.find(word) {
        let before = pos.checked_sub(1)
            .and_then(|i| rest.as_bytes().get(i))
            .map(|&b| b as char);
        let after = rest.as_bytes().get(pos + word.len())
            .map(|&b| b as char);

        let boundary_before = before.map(|c| !c.is_alphanumeric() && c != '_').unwrap_or(true);
        let boundary_after  = after .map(|c| !c.is_alphanumeric() && c != '_').unwrap_or(true);

        if boundary_before && boundary_after {
            result.push_str(&rest[..pos]);
            result.push_str(replacement);
        } else {
            result.push_str(&rest[..pos + word.len()]);
        }
        rest = &rest[pos + word.len()..];
    }
    result.push_str(rest);
    result
}