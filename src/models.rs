//! Data models for WebTOTP application
//!
//! Author: steven

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// TOTP entry with encrypted secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TOTPEntry {
    pub id: String,
    pub name: String,
    pub secret: String, // Encrypted secret key
    pub created_at: String,
}

/// Application state containing user credentials and TOTP data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub username: String,
    pub password_hash: String,
    pub totp_entries: Vec<TOTPEntry>,
    pub totp_2fa_enabled: bool,
    pub totp_2fa_secret: Option<String>, // Independent encrypted secret for 2FA login protection
    pub sessions: HashMap<String, SessionInfo>,
    pub master_key_hash: Option<String>, // Bcrypt hash of master key for verification
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub username: String,
    pub created_at: String,
}

impl AppState {
    /// Create a new AppState with default credentials (admin/admin)
    pub fn new() -> Self {
        let default_password_hash = bcrypt::hash("admin", bcrypt::DEFAULT_COST)
            .unwrap_or_else(|_| "error".to_string());

        AppState {
            username: "admin".to_string(),
            password_hash: default_password_hash,
            totp_entries: Vec::new(),
            totp_2fa_enabled: false,
            totp_2fa_secret: None,
            sessions: HashMap::new(),
            master_key_hash: None,
        }
    }
}

// 请求/响应模型
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub totp_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub session_token: Option<String>,
    pub requires_2fa: bool,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct ChangePasswordResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct AddTOTPRequest {
    pub name: String,
    pub secret: String,
}

#[derive(Debug, Serialize)]
pub struct AddTOTPResponse {
    pub success: bool,
    pub message: String,
    pub id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTOTPRequest {
    pub id: String,
    pub confirmed: bool,
}

#[derive(Debug, Serialize)]
pub struct DeleteTOTPResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct GetCodeRequest {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct GetCodeResponse {
    pub success: bool,
    pub code: Option<String>,
    pub remaining_seconds: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct TOTPListResponse {
    pub success: bool,
    pub entries: Vec<TOTPEntryResponse>,
}

#[derive(Debug, Serialize)]
pub struct TOTPEntryResponse {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub authenticated: bool,
    pub username: Option<String>,
    pub totp_2fa_enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct Toggle2FAResponse {
    pub success: bool,
    pub message: String,
    pub enabled: bool,
    pub secret: Option<String>,  // Base32编码的密钥（启用时返回）
    pub qr_code: Option<String>, // QR码数据URL（启用时返回）
}

#[derive(Debug, Deserialize)]
pub struct Enable2FARequest {
    pub totp_code: String,
}

#[derive(Debug, Serialize)]
pub struct Enable2FAResponse {
    pub success: bool,
    pub message: String,
    pub secret: Option<String>,
    pub qr_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Get2FASetupResponse {
    pub success: bool,
    pub message: Option<String>,
    pub secret: Option<String>,
    pub qr_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Get2FACodeResponse {
    pub success: bool,
    pub code: Option<String>,
    pub remaining_seconds: Option<u32>,
}

