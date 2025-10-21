//! 2FA settings API handlers
//!
//! Author: steven
//!
//! Handles 2FA login protection setup, configuration, and management

use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::Mutex;

use crate::models::*;
use crate::auth;

/// Get 2FA login protection status
pub async fn get_2fa_enabled(
    req: HttpRequest,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = auth::extract_token_from_header(header_str) {
                let app_state = state.lock().unwrap();

                if auth::validate_session(&token, &app_state.sessions).is_none() {
                    return HttpResponse::Unauthorized().json(serde_json::json!({
                        "success": false,
                        "enabled": false
                    }));
                }

                return HttpResponse::Ok().json(serde_json::json!({
                    "success": true,
                    "enabled": app_state.totp_2fa_enabled
                }));
            }
        }
    }

    HttpResponse::Unauthorized().json(serde_json::json!({
        "success": false,
        "enabled": false
    }))
}

/// 获取2FA设置信息（生成新的独立密钥用于2FA Login Protection）
pub async fn get_2fa_setup(
    req: HttpRequest,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = auth::extract_token_from_header(header_str) {
                let mut app_state = state.lock().unwrap();

                if auth::validate_session(&token, &app_state.sessions).is_none() {
                    return HttpResponse::Unauthorized().json(Get2FASetupResponse {
                        success: false,
                        message: Some("Unauthorized".to_string()),
                        secret: None,
                        qr_code: None,
                    });
                }

                // 生成一个新的随机密钥用于2FA Login Protection
                let new_secret = crate::crypto::generate_random_secret();

                // 加密密钥以保存
                let master_key = crate::config::get_master_key();
                let secret_encrypted = match crate::crypto::encrypt(&new_secret, &master_key) {
                    Ok(encrypted) => encrypted,
                    Err(_) => {
                        return HttpResponse::InternalServerError().json(Get2FASetupResponse {
                            success: false,
                            message: Some("Failed to encrypt secret".to_string()),
                            secret: None,
                            qr_code: None,
                        });
                    }
                };

                // 临时保存加密的密钥（用于验证）
                app_state.totp_2fa_secret = Some(secret_encrypted);
                let _ = crate::storage::save_state(&app_state);

                // 生成otpauth://格式的URI
                let otpauth_uri = format!(
                    "otpauth://totp/WebTOTP:2FA%20Login?secret={}&issuer=WebTOTP",
                    new_secret
                );

                return HttpResponse::Ok().json(Get2FASetupResponse {
                    success: true,
                    message: None,
                    secret: Some(new_secret),
                    qr_code: Some(otpauth_uri),
                });
            }
        }
    }

    HttpResponse::Unauthorized().json(Get2FASetupResponse {
        success: false,
        message: Some("Unauthorized".to_string()),
        secret: None,
        qr_code: None,
    })
}

/// 启用2FA（需要验证代码）
pub async fn enable_2fa(
    req: HttpRequest,
    body: web::Json<Enable2FARequest>,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = auth::extract_token_from_header(header_str) {
                let mut app_state = state.lock().unwrap();

                if auth::validate_session(&token, &app_state.sessions).is_none() {
                    return HttpResponse::Unauthorized().json(Enable2FAResponse {
                        success: false,
                        message: "Unauthorized".to_string(),
                        secret: None,
                        qr_code: None,
                    });
                }

                // 检查是否有临时保存的2FA密钥
                if app_state.totp_2fa_secret.is_none() {
                    return HttpResponse::BadRequest().json(Enable2FAResponse {
                        success: false,
                        message: "No 2FA setup in progress. Please get 2FA setup first.".to_string(),
                        secret: None,
                        qr_code: None,
                    });
                }

                // 解密临时保存的密钥进行验证
                let secret_encrypted = app_state.totp_2fa_secret.clone().unwrap();
                let master_key = crate::config::get_master_key();
                let secret_decrypted = match crate::crypto::decrypt(&secret_encrypted, &master_key) {
                    Ok(decrypted) => decrypted,
                    Err(_) => {
                        return HttpResponse::InternalServerError().json(Enable2FAResponse {
                            success: false,
                            message: "Failed to decrypt secret".to_string(),
                            secret: None,
                            qr_code: None,
                        });
                    }
                };

                // 验证TOTP代码
                if !auth::verify_totp(&secret_decrypted, &body.totp_code) {
                    return HttpResponse::Unauthorized().json(Enable2FAResponse {
                        success: false,
                        message: "Invalid TOTP code".to_string(),
                        secret: None,
                        qr_code: None,
                    });
                }

                // 验证成功，启用2FA
                app_state.totp_2fa_enabled = true;
                let _ = crate::storage::save_state(&app_state);

                return HttpResponse::Ok().json(Enable2FAResponse {
                    success: true,
                    message: "2FA enabled successfully".to_string(),
                    secret: Some(secret_decrypted),
                    qr_code: None,
                });
            }
        }
    }

    HttpResponse::Unauthorized().json(Enable2FAResponse {
        success: false,
        message: "Unauthorized".to_string(),
        secret: None,
        qr_code: None,
    })
}

/// 禁用2FA
pub async fn disable_2fa(
    req: HttpRequest,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = auth::extract_token_from_header(header_str) {
                let mut app_state = state.lock().unwrap();

                if auth::validate_session(&token, &app_state.sessions).is_none() {
                    return HttpResponse::Unauthorized().json(Toggle2FAResponse {
                        success: false,
                        message: "Unauthorized".to_string(),
                        enabled: app_state.totp_2fa_enabled,
                        secret: None,
                        qr_code: None,
                    });
                }

                app_state.totp_2fa_enabled = false;
                let _ = crate::storage::save_state(&app_state);

                return HttpResponse::Ok().json(Toggle2FAResponse {
                    success: true,
                    message: "2FA disabled".to_string(),
                    enabled: false,
                    secret: None,
                    qr_code: None,
                });
            }
        }
    }

    HttpResponse::Unauthorized().json(Toggle2FAResponse {
        success: false,
        message: "Unauthorized".to_string(),
        enabled: false,
        secret: None,
        qr_code: None,
    })
}

/// 获取2FA代码（用于测试）
pub async fn get_2fa_code(
    req: HttpRequest,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = auth::extract_token_from_header(header_str) {
                let app_state = state.lock().unwrap();

                if auth::validate_session(&token, &app_state.sessions).is_none() {
                    return HttpResponse::Unauthorized().json(Get2FACodeResponse {
                        success: false,
                        code: None,
                        remaining_seconds: None,
                    });
                }

                // 检查是否有2FA密钥
                if let Some(secret_encrypted) = &app_state.totp_2fa_secret {
                    let master_key = crate::config::get_master_key();
                    match crate::crypto::decrypt(secret_encrypted, &master_key) {
                        Ok(secret_decrypted) => {
                            // 生成TOTP代码
                            use totp_lite::{totp_custom, Sha1};

                            let now = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs();

                            // 解码base32密钥
                            if let Ok(decoded) = crate::auth::base32_decode(&secret_decrypted) {
                                let code = totp_custom::<Sha1>(30, 6, &decoded, now);
                                let remaining = 30 - (now % 30);

                                return HttpResponse::Ok().json(Get2FACodeResponse {
                                    success: true,
                                    code: Some(code),
                                    remaining_seconds: Some(remaining as u32),
                                });
                            }
                        }
                        Err(_) => {
                            return HttpResponse::InternalServerError().json(Get2FACodeResponse {
                                success: false,
                                code: None,
                                remaining_seconds: None,
                            });
                        }
                    }
                }

                return HttpResponse::BadRequest().json(Get2FACodeResponse {
                    success: false,
                    code: None,
                    remaining_seconds: None,
                });
            }
        }
    }

    HttpResponse::Unauthorized().json(Get2FACodeResponse {
        success: false,
        code: None,
        remaining_seconds: None,
    })
}


