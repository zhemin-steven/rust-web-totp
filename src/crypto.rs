//! Cryptographic utilities for WebTOTP
//!
//! Author: steven
//!
//! Provides encryption/decryption using AES-256-GCM and password hashing using bcrypt

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::Rng;
use std::error::Error;
use base64::engine::general_purpose;
use base64::Engine;

const NONCE_SIZE: usize = 12;
const KEY_SIZE: usize = 32;

/// Derive encryption key from master key using hash function
pub fn derive_key(master_key: &str) -> [u8; KEY_SIZE] {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    master_key.hash(&mut hasher);
    let hash = hasher.finish();

    let mut key = [0u8; KEY_SIZE];
    for i in 0..KEY_SIZE {
        key[i] = ((hash >> (i % 8 * 8)) & 0xFF) as u8;
    }
    key
}

/// Encrypt plaintext using AES-256-GCM with master key
pub fn encrypt(plaintext: &str, master_key: &str) -> Result<String, Box<dyn Error>> {
    let key = derive_key(master_key);
    let cipher = Aes256Gcm::new(&key.into());

    let mut rng = rand::rng();
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    rng.fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| format!("Encryption failed: {}", e))?;

    // Combine nonce + ciphertext and encode as base64
    let mut encrypted = nonce_bytes.to_vec();
    encrypted.extend_from_slice(&ciphertext);

    Ok(general_purpose::STANDARD.encode(&encrypted))
}

/// Decrypt ciphertext using AES-256-GCM with master key
pub fn decrypt(encrypted: &str, master_key: &str) -> Result<String, Box<dyn Error>> {
    let key = derive_key(master_key);
    let cipher = Aes256Gcm::new(&key.into());

    let encrypted_bytes = general_purpose::STANDARD.decode(encrypted)?;

    if encrypted_bytes.len() < NONCE_SIZE {
        return Err("Invalid encrypted data".into());
    }

    let (nonce_bytes, ciphertext) = encrypted_bytes.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;

    Ok(String::from_utf8(plaintext)?)
}

/// Generate a random base32-encoded secret for TOTP
pub fn generate_random_secret() -> String {
    use data_encoding::BASE32;

    let mut rng = rand::rng();
    let mut random_bytes = [0u8; 20]; // 20 bytes = 160 bits (sufficient security)
    rng.fill(&mut random_bytes);

    BASE32.encode(&random_bytes)
}

/// Hash master key using bcrypt
pub fn hash_master_key(master_key: &str) -> Result<String, Box<dyn Error>> {
    use bcrypt::{hash, DEFAULT_COST};

    hash(master_key, DEFAULT_COST)
        .map_err(|e| format!("Failed to hash master key: {}", e).into())
}

/// Verify master key against bcrypt hash
pub fn verify_master_key(master_key: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
    use bcrypt::verify;

    verify(master_key, hash)
        .map_err(|e| format!("Failed to verify master key: {}", e).into())
}
