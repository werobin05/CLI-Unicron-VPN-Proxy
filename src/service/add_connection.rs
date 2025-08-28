use std::fs;
use std::io::{self};

use crate::service::parse::connect::{parse_connection_url};
use crate::{
    service::{
        crypto::encrypt,
        path_key::{get_connection_path, get_encryption_key},
    },
};

pub fn add_connection() {
    let mut uri = String::new();
    println!("\nEnter connection URI (vless:// or trojan://)");
    io::stdin().read_line(&mut uri).unwrap();

    match parse_connection_url(uri.trim()) {
        Some(connection) => {
            let json = serde_json::to_string(&connection).unwrap();
            let key = get_encryption_key();
            let encrypted = encrypt(&json, &key);

            let path = get_connection_path();
            let mut list: Vec<String> = if path.exists() {
                let content = fs::read_to_string(&path).unwrap();
                serde_json::from_str(&content).unwrap_or_default()
            } else {
                Vec::new()
            };
            
            list.push(encrypted);
            let updated = serde_json::to_string_pretty(&list).unwrap();
            fs::write(&path, updated).unwrap();

            println!("Connection saved: {}", connection.name);
        }
        None => {
            println!("Invalid URI format!");
        }
    }
}