use crate::models::AppData;
use crate::error::{AppError, Result};
use std::sync::Mutex;
use tokio::fs;
use aes_gcm::{
    aead::{Aead, NewAead},
    Aes256Gcm, Nonce, Key,
};
use rand::rngs::OsRng;
use log::{info, warn, debug};

const NONCE_SIZE: usize = 12;
const SALT_SIZE: usize = 16;

pub struct Storage {
    file_path: String,
    data: Mutex<Option<AppData>>,
    master_password_hash: Mutex<Option<String>>,
}

// 从主密码派生加密密钥
fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    use argon2::Argon2;
    
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| AppError::Encryption(format!("Key derivation failed: {}", e)))?;
    
    Ok(key)
}

impl Storage {
    pub async fn new(file_path: &str) -> Result<Self> {
        info!("Initializing storage at: {}", file_path);
        
        // 检查文件是否存在
        let file_exists = fs::metadata(file_path).await.is_ok();
        
        if !file_exists {
            info!("Data file not found, will create new one");
        }
        
        Ok(Self {
            file_path: file_path.to_string(),
            data: Mutex::new(None),
            master_password_hash: Mutex::new(None),
        })
    }

    // 解锁数据库（使用主密码）
    pub async fn unlock(&self, master_password: &str) -> Result<bool> {
        info!("Attempting to unlock database");
        
        match fs::read(&self.file_path).await {
            Ok(encrypted_data) => {
                // 文件存在，尝试解密
                match Self::decrypt_data(&encrypted_data, master_password) {
                    Ok(data) => {
                        info!("Database unlocked successfully");
                        *self.data.lock().unwrap() = Some(data);
                        *self.master_password_hash.lock().unwrap() = Some(master_password.to_string());
                        Ok(true)
                    }
                    Err(e) => {
                        warn!("Failed to unlock database: {}", e);
                        Err(AppError::InvalidMasterPassword)
                    }
                }
            }
            Err(_) => {
                // 文件不存在，创建新数据库
                info!("Creating new database with master password");
                let default_data = AppData::default();
                self.save_with_password(&default_data, master_password).await?;
                *self.data.lock().unwrap() = Some(default_data);
                *self.master_password_hash.lock().unwrap() = Some(master_password.to_string());
                Ok(true)
            }
        }
    }

    // 检查是否已解锁
    pub fn is_unlocked(&self) -> bool {
        self.data.lock().unwrap().is_some()
    }

    async fn save_with_password(
        &self,
        data: &AppData,
        password: &str,
    ) -> Result<()> {
        debug!("Encrypting and saving data");
        let encrypted_data = Self::encrypt_data(data, password)?;
        fs::write(&self.file_path, encrypted_data).await
            .map_err(|e| AppError::Storage(format!("Failed to write file: {}", e)))?;
        info!("Data saved successfully");
        Ok(())
    }

    fn encrypt_data(data: &AppData, password: &str) -> Result<Vec<u8>> {
        // 生成随机盐值
        let mut salt_bytes = [0u8; SALT_SIZE];
        rand::RngCore::fill_bytes(&mut OsRng, &mut salt_bytes);
        
        // 从密码派生密钥
        let key = derive_key(password, &salt_bytes)?;
        
        // 生成随机 nonce
        let nonce_bytes: [u8; NONCE_SIZE] = rand::random();
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // 序列化数据
        let json = serde_json::to_string(data)
            .map_err(|e| AppError::Storage(format!("Serialization failed: {}", e)))?;
        
        // 使用 AES-256-GCM 加密
        let cipher = Aes256Gcm::new(Key::from_slice(&key));
        let ciphertext = cipher
            .encrypt(nonce, json.as_bytes())
            .map_err(|e| AppError::Encryption(format!("Encryption failed: {}", e)))?;
        
        // 格式: salt (16 bytes) + nonce (12 bytes) + ciphertext
        let mut result = Vec::new();
        result.extend_from_slice(&salt_bytes);
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        
        debug!("Data encrypted successfully");
        Ok(result)
    }

    fn decrypt_data(encrypted_data: &[u8], password: &str) -> Result<AppData> {
        if encrypted_data.len() < SALT_SIZE + NONCE_SIZE {
            return Err(AppError::Encryption("Invalid encrypted data format".to_string()));
        }
        
        // 提取 salt, nonce 和 ciphertext
        let salt = &encrypted_data[..SALT_SIZE];
        let nonce_bytes = &encrypted_data[SALT_SIZE..SALT_SIZE + NONCE_SIZE];
        let ciphertext = &encrypted_data[SALT_SIZE + NONCE_SIZE..];
        
        // 从密码派生密钥
        let key = derive_key(password, salt)?;
        
        // 解密
        let cipher = Aes256Gcm::new(Key::from_slice(&key));
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| AppError::InvalidMasterPassword)?;
        
        // 反序列化
        let json = String::from_utf8(plaintext)
            .map_err(|e| AppError::Encryption(format!("UTF-8 decode failed: {}", e)))?;
        let data: AppData = serde_json::from_str(&json)
            .map_err(|e| AppError::Storage(format!("Deserialization failed: {}", e)))?;
        
        debug!("Data decrypted successfully");
        Ok(data)
    }

    pub async fn save(&self) -> Result<()> {
        let data_lock = self.data.lock().unwrap();
        let password_lock = self.master_password_hash.lock().unwrap();
        
        match (data_lock.as_ref(), password_lock.as_ref()) {
            (Some(data), Some(password)) => {
                self.save_with_password(data, password).await
            }
            _ => Err(AppError::DatabaseLocked),
        }
    }

    pub fn get_user(&self) -> Result<crate::models::User> {
        let data = self.data.lock().unwrap();
        data.as_ref()
            .map(|d| d.user.clone())
            .ok_or(AppError::DatabaseLocked)
    }

    pub fn update_user<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(&mut crate::models::User),
    {
        let mut data = self.data.lock().unwrap();
        match data.as_mut() {
            Some(d) => {
                f(&mut d.user);
                Ok(())
            }
            None => Err(AppError::DatabaseLocked),
        }
    }

    pub fn get_totp_entries(&self) -> Result<Vec<crate::models::TotpEntry>> {
        let data = self.data.lock().unwrap();
        data.as_ref()
            .map(|d| d.totp_entries.clone())
            .ok_or(AppError::DatabaseLocked)
    }

    pub fn add_totp_entry(&self, entry: crate::models::TotpEntry) -> Result<()> {
        let mut data = self.data.lock().unwrap();
        match data.as_mut() {
            Some(d) => {
                d.totp_entries.push(entry);
                Ok(())
            }
            None => Err(AppError::DatabaseLocked),
        }
    }

    pub fn delete_totp_entry(&self, id: &str) -> Result<bool> {
        let mut data = self.data.lock().unwrap();
        match data.as_mut() {
            Some(d) => {
                let len_before = d.totp_entries.len();
                d.totp_entries.retain(|e| e.id != id);
                Ok(d.totp_entries.len() < len_before)
            }
            None => Err(AppError::DatabaseLocked),
        }
    }

    pub fn get_totp_entry(&self, id: &str) -> Result<Option<crate::models::TotpEntry>> {
        let data = self.data.lock().unwrap();
        match data.as_ref() {
            Some(d) => Ok(d.totp_entries.iter().find(|e| e.id == id).cloned()),
            None => Err(AppError::DatabaseLocked),
        }
    }
}

