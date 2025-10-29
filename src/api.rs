use actix_web::{get, post, web, HttpResponse, Responder};
use actix_session::Session;
use crate::{auth, models::*, storage::Storage, totp_manager};
use log::{info, warn, error};

// 辅助宏：处理数据库锁定错误
macro_rules! handle_storage_result {
    ($result:expr) => {
        match $result {
            Ok(val) => val,
            Err(crate::error::AppError::DatabaseLocked) => {
                return HttpResponse::ServiceUnavailable().json(ApiResponse {
                    success: false,
                    message: "Database is locked. Please unlock first.".to_string(),
                });
            }
            Err(e) => {
                error!("Storage error: {}", e);
                return HttpResponse::InternalServerError().json(ApiResponse {
                    success: false,
                    message: format!("Internal error: {}", e),
                });
            }
        }
    };
}

#[post("/unlock")]
async fn unlock_database(
    data: web::Json<UnlockRequest>,
    storage: web::Data<Storage>,
) -> impl Responder {
    info!("Database unlock requested");
    
    match storage.unlock(&data.master_password).await {
        Ok(true) => {
            info!("Database unlocked successfully");
            HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: "Database unlocked successfully".to_string(),
            })
        }
        Ok(false) => {
            warn!("Invalid master password attempt");
            HttpResponse::Ok().json(ApiResponse {
                success: false,
                message: "Invalid master password".to_string(),
            })
        }
        Err(e) => {
            error!("Unlock error: {}", e);
            HttpResponse::Ok().json(ApiResponse {
                success: false,
                message: format!("Error: {}", e),
            })
        }
    }
}

#[get("/lock-status")]
async fn get_lock_status(storage: web::Data<Storage>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "locked": !storage.is_unlocked()
    }))
}

#[post("/check-user-2fa")]
async fn check_user_2fa(
    data: web::Json<CheckUser2FARequest>,
    storage: web::Data<Storage>,
) -> impl Responder {
    let user = handle_storage_result!(storage.get_user());
    
    if data.username == user.username {
        return HttpResponse::Ok().json(serde_json::json!({
            "requires_2fa": user.two_fa_enabled
        }));
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "requires_2fa": false
    }))
}

#[post("/login")]
async fn login(
    session: Session,
    data: web::Json<LoginRequest>,
    storage: web::Data<Storage>,
) -> impl Responder {
    let user = handle_storage_result!(storage.get_user());
    
    if data.username != user.username {
        return HttpResponse::Ok().json(LoginResponse {
            success: false,
            message: "Invalid username or password".to_string(),
            requires_2fa: None,
        });
    }
    
    if !verify_password(&data.password, &user.password_hash) {
        return HttpResponse::Ok().json(LoginResponse {
            success: false,
            message: "Invalid username or password".to_string(),
            requires_2fa: None,
        });
    }
    
    if user.two_fa_enabled {
        if let Some(ref totp_code) = data.totp_code {
            if let Some(ref secret) = user.two_fa_secret {
                match totp_manager::verify_totp_code(secret, totp_code) {
                    Ok(true) => {
                        if auth::set_session(&session, &user.username).is_ok() {
                            return HttpResponse::Ok().json(LoginResponse {
                                success: true,
                                message: "Login successful".to_string(),
                                requires_2fa: None,
                            });
                        }
                    }
                    _ => {
                        return HttpResponse::Ok().json(LoginResponse {
                            success: false,
                            message: "Invalid 2FA code".to_string(),
                            requires_2fa: Some(true),
                        });
                    }
                }
            }
        } else {
            return HttpResponse::Ok().json(LoginResponse {
                success: false,
                message: "2FA code required".to_string(),
                requires_2fa: Some(true),
            });
        }
    }
    
    if auth::set_session(&session, &user.username).is_ok() {
        HttpResponse::Ok().json(LoginResponse {
            success: true,
            message: "Login successful".to_string(),
            requires_2fa: None,
        })
    } else {
        HttpResponse::InternalServerError().json(LoginResponse {
            success: false,
            message: "Session error".to_string(),
            requires_2fa: None,
        })
    }
}

#[post("/logout")]
async fn logout(session: Session) -> impl Responder {
    auth::clear_session(&session);
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: "Logged out successfully".to_string(),
    })
}

#[get("/check-session")]
async fn check_session(session: Session) -> impl Responder {
    let authenticated = auth::check_auth(&session);
    HttpResponse::Ok().json(ApiResponse {
        success: authenticated,
        message: if authenticated { "Authenticated" } else { "Not authenticated" }.to_string(),
    })
}

#[post("/change-password")]
async fn change_password(
    session: Session,
    data: web::Json<ChangePasswordRequest>,
    storage: web::Data<Storage>,
) -> impl Responder {
    if !auth::check_auth(&session) {
        return HttpResponse::Unauthorized().json(ApiResponse {
            success: false,
            message: "Not authenticated".to_string(),
        });
    }
    
    let user = handle_storage_result!(storage.get_user());
    
    if !verify_password(&data.old_password, &user.password_hash) {
        return HttpResponse::Ok().json(ApiResponse {
            success: false,
            message: "Invalid old password".to_string(),
        });
    }
    
    let new_hash = hash_password(&data.new_password);
    if let Err(e) = storage.update_user(|u| {
        u.password_hash = new_hash;
    }) {
        error!("Failed to update password: {}", e);
        return HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: "Failed to update password".to_string(),
        });
    }
    
    if let Err(e) = storage.save().await {
        error!("Failed to save: {}", e);
        return HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: "Failed to save changes".to_string(),
        });
    }
    
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: "Password changed successfully".to_string(),
    })
}

#[post("/enable-2fa")]
async fn enable_2fa(
    session: Session,
    storage: web::Data<Storage>,
) -> impl Responder {
    if !auth::check_auth(&session) {
        return HttpResponse::Unauthorized().json(ApiResponse {
            success: false,
            message: "Not authenticated".to_string(),
        });
    }
    
    let secret = totp_manager::generate_secret();
    let qr_code = match totp_manager::generate_qr_code(&secret, "admin", "WebTOTP") {
        Ok(qr) => qr,
        Err(e) => {
            error!("Failed to generate QR code: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                message: "Failed to generate QR code".to_string(),
            });
        }
    };
    
    let otpauth_url = format!(
        "otpauth://totp/WebTOTP:admin?secret={}&issuer=WebTOTP",
        secret
    );
    
    if let Err(e) = storage.update_user(|u| {
        u.two_fa_secret = Some(secret.clone());
    }) {
        error!("Failed to update 2FA secret: {}", e);
        return HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: "Failed to enable 2FA".to_string(),
        });
    }
    
    if let Err(e) = storage.save().await {
        error!("Failed to save: {}", e);
    }
    
    HttpResponse::Ok().json(TwoFaSetupResponse {
        secret,
        qr_code,
        otpauth_url,
    })
}

#[post("/verify-2fa")]
async fn verify_2fa(
    session: Session,
    data: web::Json<VerifyTwoFaRequest>,
    storage: web::Data<Storage>,
) -> impl Responder {
    if !auth::check_auth(&session) {
        return HttpResponse::Unauthorized().json(ApiResponse {
            success: false,
            message: "Not authenticated".to_string(),
        });
    }
    
    let user = handle_storage_result!(storage.get_user());
    
    if let Some(ref secret) = user.two_fa_secret {
        match totp_manager::verify_totp_code(secret, &data.code) {
            Ok(true) => {
                if let Err(e) = storage.update_user(|u| {
                    u.two_fa_enabled = true;
                }) {
                    error!("Failed to enable 2FA: {}", e);
                    return HttpResponse::InternalServerError().json(ApiResponse {
                        success: false,
                        message: "Failed to enable 2FA".to_string(),
                    });
                }
                
                if let Err(e) = storage.save().await {
                    error!("Failed to save: {}", e);
                }
                
                return HttpResponse::Ok().json(ApiResponse {
                    success: true,
                    message: "2FA enabled successfully".to_string(),
                });
            }
            _ => {
                return HttpResponse::Ok().json(ApiResponse {
                    success: false,
                    message: "Invalid 2FA code".to_string(),
                });
            }
        }
    }
    
    HttpResponse::Ok().json(ApiResponse {
        success: false,
        message: "2FA not set up".to_string(),
    })
}

#[post("/disable-2fa")]
async fn disable_2fa(
    session: Session,
    data: web::Json<DisableTwoFaRequest>,
    storage: web::Data<Storage>,
) -> impl Responder {
    if !auth::check_auth(&session) {
        return HttpResponse::Unauthorized().json(ApiResponse {
            success: false,
            message: "Not authenticated".to_string(),
        });
    }
    
    let user = handle_storage_result!(storage.get_user());
    
    if !verify_password(&data.password, &user.password_hash) {
        return HttpResponse::Ok().json(ApiResponse {
            success: false,
            message: "Invalid password".to_string(),
        });
    }
    
    if user.two_fa_enabled {
        if let Some(ref secret) = user.two_fa_secret {
            match totp_manager::verify_totp_code(secret, &data.code) {
                Ok(true) => {}
                _ => {
                    return HttpResponse::Ok().json(ApiResponse {
                        success: false,
                        message: "Invalid 2FA code".to_string(),
                    });
                }
            }
        }
    }
    
    if let Err(e) = storage.update_user(|u| {
        u.two_fa_enabled = false;
        u.two_fa_secret = None;
    }) {
        error!("Failed to disable 2FA: {}", e);
        return HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: "Failed to disable 2FA".to_string(),
        });
    }
    
    if let Err(e) = storage.save().await {
        error!("Failed to save: {}", e);
    }
    
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        message: "2FA disabled successfully".to_string(),
    })
}

#[get("/2fa-status")]
async fn get_2fa_status(
    session: Session,
    storage: web::Data<Storage>,
) -> impl Responder {
    if !auth::check_auth(&session) {
        return HttpResponse::Unauthorized().json(ApiResponse {
            success: false,
            message: "Not authenticated".to_string(),
        });
    }
    
    let user = handle_storage_result!(storage.get_user());
    HttpResponse::Ok().json(serde_json::json!({
        "enabled": user.two_fa_enabled
    }))
}

#[post("/totp/add")]
async fn add_totp_entry(
    session: Session,
    data: web::Json<AddTotpRequest>,
    storage: web::Data<Storage>,
) -> impl Responder {
    if !auth::check_auth(&session) {
        return HttpResponse::Unauthorized().json(ApiResponse {
            success: false,
            message: "Not authenticated".to_string(),
        });
    }
    
    let entry = TotpEntry::new(
        data.name.clone(),
        data.issuer.clone(),
        data.secret.clone(),
    );
    
    if let Err(e) = storage.add_totp_entry(entry.clone()) {
        error!("Failed to add TOTP entry: {}", e);
        return HttpResponse::InternalServerError().json(ApiResponse {
            success: false,
            message: "Failed to add entry".to_string(),
        });
    }
    
    if let Err(e) = storage.save().await {
        error!("Failed to save: {}", e);
    }
    
    HttpResponse::Ok().json(entry)
}

#[get("/totp/list")]
async fn list_totp_entries(
    session: Session,
    storage: web::Data<Storage>,
) -> impl Responder {
    if !auth::check_auth(&session) {
        return HttpResponse::Unauthorized().json(ApiResponse {
            success: false,
            message: "Not authenticated".to_string(),
        });
    }
    
    let entries = handle_storage_result!(storage.get_totp_entries());
    HttpResponse::Ok().json(entries)
}

#[post("/totp/delete")]
async fn delete_totp_entry(
    session: Session,
    data: web::Json<DeleteTotpRequest>,
    storage: web::Data<Storage>,
) -> impl Responder {
    if !auth::check_auth(&session) {
        return HttpResponse::Unauthorized().json(ApiResponse {
            success: false,
            message: "Not authenticated".to_string(),
        });
    }
    
    let deleted = handle_storage_result!(storage.delete_totp_entry(&data.id));
    if deleted {
        if let Err(e) = storage.save().await {
            error!("Failed to save: {}", e);
        }
        HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: "Entry deleted successfully".to_string(),
        })
    } else {
        HttpResponse::Ok().json(ApiResponse {
            success: false,
            message: "Entry not found".to_string(),
        })
    }
}

#[get("/totp/generate/{id}")]
async fn generate_totp_code(
    session: Session,
    id: web::Path<String>,
    storage: web::Data<Storage>,
) -> impl Responder {
    if !auth::check_auth(&session) {
        return HttpResponse::Unauthorized().json(ApiResponse {
            success: false,
            message: "Not authenticated".to_string(),
        });
    }
    
    match handle_storage_result!(storage.get_totp_entry(&id)) {
        Some(entry) => {
            match totp_manager::generate_totp_code(&entry.secret) {
                Ok((code, remaining)) => {
                    HttpResponse::Ok().json(TotpCodeResponse {
                        code,
                        remaining_seconds: remaining,
                    })
                }
                Err(e) => {
                    error!("Failed to generate TOTP code: {}", e);
                    HttpResponse::InternalServerError().json(ApiResponse {
                        success: false,
                        message: "Failed to generate code".to_string(),
                    })
                }
            }
        }
        None => {
            HttpResponse::NotFound().json(ApiResponse {
                success: false,
                message: "Entry not found".to_string(),
            })
        }
    }
}

