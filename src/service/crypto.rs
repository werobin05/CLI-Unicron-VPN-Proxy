use base64::{engine::general_purpose, Engine};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::{Aead, OsRng, rand_core::RngCore};

pub fn encrypt(data: &str, key: &[u8;32]) -> String {
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();

    let mut nonce_bytes = [0u8;12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, data.as_bytes()).expect("encryption failed");

    let mut result = Vec::new();
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    general_purpose::STANDARD.encode(&result)
}

pub fn decrypt(data: &str, key: &[u8;32]) -> String {
    let bytes = general_purpose::STANDARD.decode(data).expect("base64 decode failed");
    let (nonce_bytes, ciphertext) = bytes.split_at(12);

    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext).expect("decryption failed");
    String::from_utf8(plaintext).expect("invalid UTF-8")
}