use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub two_fa_enabled: bool,
    pub two_fa_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpEntry {
    pub id: String,
    pub name: String,
    pub issuer: String,
    pub secret: String,
    pub created_at: String,
}

impl TotpEntry {
    pub fn new(name: String, issuer: String, secret: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            issuer,
            secret,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppData {
    pub user: User,
    pub totp_entries: Vec<TotpEntry>,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            user: User {
                username: "admin".to_string(),
                password_hash: hash_password("admin"),
                two_fa_enabled: false,
                two_fa_secret: None,
            },
            totp_entries: Vec::new(),
        }
    }
}

pub fn hash_password(password: &str) -> String {
    use sha2::{Sha256, Digest};
    
    const SALT: &[u8] = b"web-totp-salt"; // In production, use random salt per user
    
    // 简单的迭代哈希（用于登录密码）
    let mut hasher = Sha256::new();
    for _ in 0..100_000 {
        hasher.update(password.as_bytes());
        hasher.update(SALT);
    }
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn verify_password(password: &str, hash_str: &str) -> bool {
    let computed_hash = hash_password(password);
    computed_hash == hash_str
}

// Request/Response structs
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub totp_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CheckUser2FARequest {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct UnlockRequest {
    pub master_password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub requires_2fa: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct TwoFaSetupResponse {
    pub secret: String,
    pub qr_code: String,
    pub otpauth_url: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyTwoFaRequest {
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct DisableTwoFaRequest {
    pub password: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct AddTotpRequest {
    pub name: String,
    pub issuer: String,
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTotpRequest {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct TotpCodeResponse {
    pub code: String,
    pub remaining_seconds: u64,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

