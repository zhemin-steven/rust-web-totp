//! TOTP management API handlers
//!
//! Author: steven
//!
//! Handles TOTP entry management including list, add, delete, and code generation

use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::Mutex;

use crate::models::*;
use crate::auth;
use crate::crypto;

/// List all TOTP entries for the authenticated user
pub async fn list_totp(
    req: HttpRequest,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = auth::extract_token_from_header(header_str) {
                let app_state = state.lock().unwrap();

                if auth::validate_session(&token, &app_state.sessions).is_none() {
                    return HttpResponse::Unauthorized().json(TOTPListResponse {
                        success: false,
                        entries: vec![],
                    });
                }

                let entries = app_state
                    .totp_entries
                    .iter()
                    .map(|e| TOTPEntryResponse {
                        id: e.id.clone(),
                        name: e.name.clone(),
                        created_at: e.created_at.clone(),
                    })
                    .collect();

                return HttpResponse::Ok().json(TOTPListResponse {
                    success: true,
                    entries,
                });
            }
        }
    }

    HttpResponse::Unauthorized().json(TOTPListResponse {
        success: false,
        entries: vec![],
    })
}

/// 添加TOTP
pub async fn add_totp(
    req: web::Json<AddTOTPRequest>,
    http_req: HttpRequest,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    if let Some(auth_header) = http_req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = auth::extract_token_from_header(header_str) {
                let mut app_state = state.lock().unwrap();

                if auth::validate_session(&token, &app_state.sessions).is_none() {
                    return HttpResponse::Unauthorized().json(AddTOTPResponse {
                        success: false,
                        message: "Unauthorized".to_string(),
                        id: None,
                    });
                }

                // 加密密钥
                let master_key = crate::config::get_master_key();
                let encrypted_secret = match crypto::encrypt(&req.secret, &master_key) {
                    Ok(encrypted) => encrypted,
                    Err(_) => {
                        return HttpResponse::InternalServerError().json(AddTOTPResponse {
                            success: false,
                            message: "Failed to encrypt secret".to_string(),
                            id: None,
                        });
                    }
                };

                let id = {
                    use uuid::Uuid;
                    Uuid::new_v4().to_string()
                };
                let entry = TOTPEntry {
                    id: id.clone(),
                    name: req.name.clone(),
                    secret: encrypted_secret,
                    created_at: chrono::Local::now().to_rfc3339(),
                };

                app_state.totp_entries.push(entry);
                let _ = crate::storage::save_state(&app_state);

                return HttpResponse::Ok().json(AddTOTPResponse {
                    success: true,
                    message: "TOTP added successfully".to_string(),
                    id: Some(id),
                });
            }
        }
    }

    HttpResponse::Unauthorized().json(AddTOTPResponse {
        success: false,
        message: "Unauthorized".to_string(),
        id: None,
    })
}

/// 删除TOTP
pub async fn delete_totp(
    req: web::Json<DeleteTOTPRequest>,
    http_req: HttpRequest,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    if let Some(auth_header) = http_req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = auth::extract_token_from_header(header_str) {
                let mut app_state = state.lock().unwrap();

                if auth::validate_session(&token, &app_state.sessions).is_none() {
                    return HttpResponse::Unauthorized().json(DeleteTOTPResponse {
                        success: false,
                        message: "Unauthorized".to_string(),
                    });
                }

                if !req.confirmed {
                    return HttpResponse::BadRequest().json(DeleteTOTPResponse {
                        success: false,
                        message: "Deletion not confirmed".to_string(),
                    });
                }

                app_state.totp_entries.retain(|e| e.id != req.id);
                let _ = crate::storage::save_state(&app_state);

                return HttpResponse::Ok().json(DeleteTOTPResponse {
                    success: true,
                    message: "TOTP deleted successfully".to_string(),
                });
            }
        }
    }

    HttpResponse::Unauthorized().json(DeleteTOTPResponse {
        success: false,
        message: "Unauthorized".to_string(),
    })
}

/// 获取TOTP代码
pub async fn get_code(
    req: web::Json<GetCodeRequest>,
    http_req: HttpRequest,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    if let Some(auth_header) = http_req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = auth::extract_token_from_header(header_str) {
                let app_state = state.lock().unwrap();

                if auth::validate_session(&token, &app_state.sessions).is_none() {
                    return HttpResponse::Unauthorized().json(GetCodeResponse {
                        success: false,
                        code: None,
                        remaining_seconds: None,
                    });
                }

                if let Some(entry) = app_state.totp_entries.iter().find(|e| e.id == req.id) {
                    // 解密密钥
                    let master_key = crate::config::get_master_key();
                    match crypto::decrypt(&entry.secret, &master_key) {
                        Ok(decrypted_secret) => {
                            // 生成TOTP代码
                            use totp_lite::{totp_custom, Sha1};

                            if let Ok(decoded) = base32_decode(&decrypted_secret) {
                                let now = std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs();

                                let remaining = 30 - (now % 30) as u32;

                                // 使用 totp_custom 生成6位数字的TOTP代码
                                // 参数顺序: step (30秒), digits (6位), secret, time (当前Unix时间戳)
                                let code = totp_custom::<Sha1>(
                                    30,  // 30秒时间步
                                    6,   // 6位数字
                                    &decoded,
                                    now, // 当前Unix时间戳
                                );

                                return HttpResponse::Ok().json(GetCodeResponse {
                                    success: true,
                                    code: Some(code),
                                    remaining_seconds: Some(remaining),
                                });
                            }
                        }
                        Err(_) => {
                            return HttpResponse::InternalServerError().json(GetCodeResponse {
                                success: false,
                                code: None,
                                remaining_seconds: None,
                            });
                        }
                    }
                }

                return HttpResponse::NotFound().json(GetCodeResponse {
                    success: false,
                    code: None,
                    remaining_seconds: None,
                });
            }
        }
    }

    HttpResponse::Unauthorized().json(GetCodeResponse {
        success: false,
        code: None,
        remaining_seconds: None,
    })
}

fn base32_decode(input: &str) -> Result<Vec<u8>, String> {
    use data_encoding::BASE32;

    // 移除填充字符并转换为大写
    let input = input.to_uppercase();

    // 使用 data-encoding 库进行 Base32 解码
    BASE32.decode(input.as_bytes())
        .map_err(|e| format!("Base32 decode error: {}", e))
}

