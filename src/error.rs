use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Encryption error: {0}")]
    Encryption(String),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("TOTP error: {0}")]
    Totp(String),
    
    #[error("Invalid master password")]
    InvalidMasterPassword,
    
    #[error("Database locked, master password required")]
    DatabaseLocked,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, AppError>;

