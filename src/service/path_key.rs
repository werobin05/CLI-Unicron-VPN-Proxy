use std::env;
use dirs::home_dir;
use std::path::PathBuf;

pub fn get_connection_path() -> PathBuf {
    let mut path = home_dir().expect("Cannot find home directory");
    path.push(".unicron_connection.json");
    path
}

pub fn get_encryption_key() -> [u8; 32] {
    dotenv::from_filename(".env").ok();
    
    let key_str = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set in .env");
    let key_bytes = key_str.as_bytes();
    if key_bytes.len() != 32 {
        panic!("ENCRYPTION_KEY must be exactly 32 bytes");
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(key_bytes);
    key
}