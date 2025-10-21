//! Authentication API handlers
//!
//! Author: steven
//!
//! Handles login, logout, password change, and session management

use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::Mutex;

use crate::models::*;
use crate::auth;

/// Handle user login with optional 2FA verification
pub async fn login(
    req: web::Json<LoginRequest>,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    let mut app_state = state.lock().unwrap();

    // 验证用户名和密码
    if req.username != app_state.username {
        return HttpResponse::Unauthorized().json(LoginResponse {
            success: false,
            message: "Invalid username or password".to_string(),
            session_token: None,
            requires_2fa: false,
        });
    }

    if !bcrypt::verify(&req.password, &app_state.password_hash).unwrap_or(false) {
        return HttpResponse::Unauthorized().json(LoginResponse {
            success: false,
            message: "Invalid username or password".to_string(),
            session_token: None,
            requires_2fa: false,
        });
    }

    // 检查是否需要2FA
    if app_state.totp_2fa_enabled {
        if let Some(totp_code) = &req.totp_code {
            // 验证TOTP代码（使用2FA Login Protection的独立密钥）
            if let Some(secret_encrypted) = &app_state.totp_2fa_secret {
                let master_key = crate::config::get_master_key();
                match crate::crypto::decrypt(secret_encrypted, &master_key) {
                    Ok(secret_decrypted) => {
                        if !auth::verify_totp(&secret_decrypted, totp_code) {
                            return HttpResponse::Unauthorized().json(LoginResponse {
                                success: false,
                                message: "Invalid TOTP code".to_string(),
                                session_token: None,
                                requires_2fa: true,
                            });
                        }
                    }
                    Err(_) => {
                        return HttpResponse::InternalServerError().json(LoginResponse {
                            success: false,
                            message: "Failed to decrypt secret".to_string(),
                            session_token: None,
                            requires_2fa: false,
                        });
                    }
                }
            } else {
                return HttpResponse::InternalServerError().json(LoginResponse {
                    success: false,
                    message: "2FA is enabled but no secret found".to_string(),
                    session_token: None,
                    requires_2fa: false,
                });
            }
        } else {
            return HttpResponse::Unauthorized().json(LoginResponse {
                success: false,
                message: "2FA code required".to_string(),
                session_token: None,
                requires_2fa: true,
            });
        }
    }

    // 生成会话令牌
    let token = auth::generate_session_token();
    let username = app_state.username.clone();
    app_state.sessions.insert(
        token.clone(),
        crate::models::SessionInfo {
            username,
            created_at: chrono::Local::now().to_rfc3339(),
        },
    );

    // 保存会话到文件
    let _ = crate::storage::save_state(&app_state);

    HttpResponse::Ok().json(LoginResponse {
        success: true,
        message: "Login successful".to_string(),
        session_token: Some(token),
        requires_2fa: false,
    })
}

/// 登出处理
pub async fn logout(
    req: HttpRequest,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = auth::extract_token_from_header(header_str) {
                let mut app_state = state.lock().unwrap();
                app_state.sessions.remove(&token);
                // 保存会话到文件
                let _ = crate::storage::save_state(&app_state);
            }
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Logout successful"
    }))
}

/// 修改密码
pub async fn change_password(
    req: web::Json<ChangePasswordRequest>,
    http_req: HttpRequest,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    // 验证会话
    if let Some(auth_header) = http_req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = auth::extract_token_from_header(header_str) {
                let mut app_state = state.lock().unwrap();

                if auth::validate_session(&token, &app_state.sessions).is_none() {
                    return HttpResponse::Unauthorized().json(ChangePasswordResponse {
                        success: false,
                        message: "Unauthorized".to_string(),
                    });
                }

                // 验证旧密码
                if !bcrypt::verify(&req.old_password, &app_state.password_hash).unwrap_or(false) {
                    return HttpResponse::BadRequest().json(ChangePasswordResponse {
                        success: false,
                        message: "Old password is incorrect".to_string(),
                    });
                }

                // 更新密码
                match bcrypt::hash(&req.new_password, bcrypt::DEFAULT_COST) {
                    Ok(new_hash) => {
                        app_state.password_hash = new_hash;
                        let _ = crate::storage::save_state(&app_state);

                        return HttpResponse::Ok().json(ChangePasswordResponse {
                            success: true,
                            message: "Password changed successfully".to_string(),
                        });
                    }
                    Err(_) => {
                        return HttpResponse::InternalServerError().json(ChangePasswordResponse {
                            success: false,
                            message: "Failed to hash password".to_string(),
                        });
                    }
                }
            }
        }
    }

    HttpResponse::Unauthorized().json(ChangePasswordResponse {
        success: false,
        message: "Unauthorized".to_string(),
    })
}

/// 获取认证状态
pub async fn get_status(
    req: HttpRequest,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    let app_state = state.lock().unwrap();

    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(header_str) = auth_header.to_str() {
            if let Some(token) = auth::extract_token_from_header(header_str) {
                if let Some(username) = auth::validate_session(&token, &app_state.sessions) {
                    return HttpResponse::Ok().json(StatusResponse {
                        authenticated: true,
                        username: Some(username),
                        totp_2fa_enabled: app_state.totp_2fa_enabled,
                    });
                }
            }
        }
    }

    HttpResponse::Ok().json(StatusResponse {
        authenticated: false,
        username: None,
        totp_2fa_enabled: app_state.totp_2fa_enabled,
    })
}

