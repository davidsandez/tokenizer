use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to input JSON or TOML file
    #[arg(short, long)]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.input {
        None => {
            // Caso simple
            println!("{}", generate_token());
        }
        Some(path) => {
            if let Err(e) = process_file(&path) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn generate_token() -> String {
    Uuid::new_v4().to_string()
}

fn process_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let extension = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let mut used_tokens = HashSet::new();

    match extension {
        "json" => {
            let mut value: serde_json::Value = serde_json::from_str(&content)?;
            replace_values_json(&mut value, &mut used_tokens);
            println!("{}", serde_json::to_string_pretty(&value)?);
        }
        "toml" => {
            let mut value: toml::Value = toml::from_str(&content)?;
            replace_values_toml(&mut value, &mut used_tokens);
            println!("{}", toml::to_string_pretty(&value)?);
        }
        _ => {
            return Err("Unsupported file type (only .json or .toml allowed)".into());
        }
    }

    Ok(())
}

fn unique_token(set: &mut HashSet<String>) -> String {
    loop {
        let token = generate_token();
        if set.insert(token.clone()) {
            return token;
        }
    }
}

fn replace_values_json(
    value: &mut serde_json::Value,
    used_tokens: &mut HashSet<String>,
) {
    match value {
        serde_json::Value::Object(map) => {
            for (_, v) in map.iter_mut() {
                replace_values_json(v, used_tokens);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr.iter_mut() {
                replace_values_json(v, used_tokens);
            }
        }
        _ => {
            *value = serde_json::Value::String(unique_token(used_tokens));
        }
    }
}

fn replace_values_toml(
    value: &mut toml::Value,
    used_tokens: &mut HashSet<String>,
) {
    match value {
        toml::Value::Table(table) => {
            for (_, v) in table.iter_mut() {
                replace_values_toml(v, used_tokens);
            }
        }
        toml::Value::Array(arr) => {
            for v in arr.iter_mut() {
                replace_values_toml(v, used_tokens);
            }
        }
        _ => {
            *value = toml::Value::String(unique_token(used_tokens));
        }
    }
}