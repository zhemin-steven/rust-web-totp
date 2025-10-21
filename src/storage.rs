//! Data persistence layer for WebTOTP
//!
//! Author: steven
//!
//! Handles loading and saving application state to JSON file

use crate::models::AppState;
use std::fs;
use std::path::Path;

const DATA_FILE: &str = "data.json";

/// Save application state to JSON file
pub fn save_state(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(state)?;
    fs::write(DATA_FILE, json)?;
    Ok(())
}

/// Load application state from JSON file
pub fn load_state() -> Result<AppState, Box<dyn std::error::Error>> {
    if Path::new(DATA_FILE).exists() {
        let json = fs::read_to_string(DATA_FILE)?;
        let state = serde_json::from_str(&json)?;
        Ok(state)
    } else {
        Ok(AppState::new())
    }
}

