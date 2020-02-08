use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Union {
    from: u8,
    to: u8,
}

#[derive(Serialize, Deserialize)]
struct AppConfig {
    size: u8,
    unions: Vec<Union>,
}

fn main() {
    println!("Quick Union Example");

    let raw_config = fs::read_to_string("input.json").unwrap();
    let config: AppConfig = serde_json::from_str(&raw_config).unwrap();

    println!("Size: {}", config.size);
}
