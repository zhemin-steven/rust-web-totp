//! Authentication utilities for WebTOTP
//!
//! Author: steven
//!
//! Handles session management and token validation

use crate::models::SessionInfo;
use std::collections::HashMap;

/// Generate a new session token (UUID v4)
pub fn generate_session_token() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}

/// Validate a session token and return the associated username
pub fn validate_session(
    token: &str,
    sessions: &HashMap<String, SessionInfo>,
) -> Option<String> {
    sessions.get(token).map(|info| info.username.clone())
}

/// Extract session token from Authorization header (Bearer token format)
pub fn extract_token_from_header(auth_header: &str) -> Option<String> {
    if auth_header.starts_with("Bearer ") {
        Some(auth_header[7..].to_string())
    } else {
        None
    }
}

/// Verify TOTP code against the secret (with time tolerance)
pub fn verify_totp(secret: &str, code: &str) -> bool {
    use totp_lite::{totp_custom, Sha1};

    // Decode base32-encoded secret
    if let Ok(decoded) = base32_decode(secret) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Check current and adjacent time steps to allow for time drift
        // Time tolerance: -60 to +60 seconds (2 time steps)
        for time_offset in &[-60, -30, 0, 30, 60] {
            let time_to_check = (now as i64 + time_offset) as u64;
            let generated_code = totp_custom::<Sha1>(
                30,  // 30-second time step
                6,   // 6-digit code
                &decoded,
                time_to_check,
            );
            if generated_code == code {
                return true;
            }
        }
    }

    false
}

/// Decode base32-encoded string to bytes
pub fn base32_decode(input: &str) -> Result<Vec<u8>, String> {
    use data_encoding::BASE32;

    let input = input.to_uppercase();
    BASE32.decode(input.as_bytes())
        .map_err(|e| format!("Base32 decode error: {}", e))
}
