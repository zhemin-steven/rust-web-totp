//! Global configuration for WebTOTP
//!
//! Author: steven
//!
//! Manages global state including the master key for encryption/decryption

use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Global master key storage (thread-safe)
pub static MASTER_KEY: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

/// Get the master key from global storage
pub fn get_master_key() -> String {
    MASTER_KEY.lock().unwrap().clone()
}

/// Set the master key in global storage
pub fn set_master_key(key: String) {
    let mut master_key = MASTER_KEY.lock().unwrap();
    *master_key = key;
}

