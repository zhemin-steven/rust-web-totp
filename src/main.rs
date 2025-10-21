//! WebTOTP - A secure TOTP (Time-based One-Time Password) manager
//!
//! Author: steven
//!
//! This application provides a web-based interface for managing TOTP secrets
//! with enterprise-grade security features including:
//! - Dynamic master key management
//! - AES-256-GCM encryption for secrets
//! - bcrypt password hashing
//! - 2FA login protection with independent secret key

mod models;
mod handlers;
mod crypto;
mod storage;
mod auth;
mod config;

use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use std::sync::Mutex;
use std::io::{self, Write};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化应用状态 - 从文件加载或创建新的
    let mut app_state = storage::load_state().unwrap_or_else(|_| models::AppState::new());

    // 处理主密钥
    let master_key = handle_master_key(&mut app_state);

    // 将主密钥存储在全局变量中
    config::set_master_key(master_key);

    let app_state = web::Data::new(Mutex::new(app_state));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            // 认证相关路由
            .route("/api/auth/login", web::post().to(handlers::auth::login))
            .route("/api/auth/logout", web::post().to(handlers::auth::logout))
            .route("/api/auth/change-password", web::post().to(handlers::auth::change_password))
            .route("/api/auth/status", web::get().to(handlers::auth::get_status))
            // 2FA相关路由
            .route("/api/totp/list", web::get().to(handlers::totp::list_totp))
            .route("/api/totp/add", web::post().to(handlers::totp::add_totp))
            .route("/api/totp/delete", web::post().to(handlers::totp::delete_totp))
            .route("/api/totp/get-code", web::post().to(handlers::totp::get_code))
            // 2FA登录设置
            .route("/api/settings/2fa-enabled", web::get().to(handlers::settings::get_2fa_enabled))
            .route("/api/settings/2fa-setup", web::get().to(handlers::settings::get_2fa_setup))
            .route("/api/settings/enable-2fa", web::post().to(handlers::settings::enable_2fa))
            .route("/api/settings/disable-2fa", web::post().to(handlers::settings::disable_2fa))
            .route("/api/settings/2fa-code", web::get().to(handlers::settings::get_2fa_code))
            // 静态文件
            .service(actix_files::Files::new("/", "./frontend/dist").index_file("index.html"))
    })
    .bind("127.0.0.1:18007")?
    .run()
    .await
}

/// 处理主密钥的输入和验证
fn handle_master_key(app_state: &mut models::AppState) -> String {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║           WebTOTP Master Key Configuration                ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // 首先检查环境变量
    if let Ok(env_key) = std::env::var("WEBTOTP_MASTER_KEY") {
        if !env_key.is_empty() {
            println!("✓ Master key loaded from WEBTOTP_MASTER_KEY environment variable");

            // 如果还没有设置过主密钥哈希，则设置
            if app_state.master_key_hash.is_none() {
                match crypto::hash_master_key(&env_key) {
                    Ok(hash) => {
                        app_state.master_key_hash = Some(hash);
                        let _ = storage::save_state(app_state);
                        println!("✓ Master key hash saved\n");
                    }
                    Err(e) => {
                        println!("✗ Error hashing master key: {}\n", e);
                    }
                }
            } else {
                // 验证环境变量中的密钥是否与保存的哈希匹配
                if let Some(ref key_hash) = app_state.master_key_hash {
                    match crypto::verify_master_key(&env_key, key_hash) {
                        Ok(true) => {
                            println!("✓ Master key verified\n");
                        }
                        Ok(false) => {
                            println!("✗ Master key from environment variable does not match!\n");
                            println!("Please provide the correct master key interactively.\n");
                            return interactive_master_key(app_state);
                        }
                        Err(e) => {
                            println!("✗ Error verifying master key: {}\n", e);
                            return interactive_master_key(app_state);
                        }
                    }
                }
            }

            return env_key;
        }
    }

    // 如果没有环境变量，使用交互式输入
    interactive_master_key(app_state)
}

/// 交互式主密钥输入
fn interactive_master_key(app_state: &mut models::AppState) -> String {
    // 检查是否已经设置过主密钥
    if let Some(ref key_hash) = app_state.master_key_hash {
        println!("✓ Master key already configured.");
        println!("Please enter the master key to decrypt your data:\n");

        loop {
            print!("Enter master key: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let master_key = input.trim().to_string();

            if master_key.is_empty() {
                println!("✗ Master key cannot be empty. Please try again.\n");
                continue;
            }

            // 验证主密钥
            match crypto::verify_master_key(&master_key, key_hash) {
                Ok(true) => {
                    println!("✓ Master key verified successfully!\n");
                    return master_key;
                }
                Ok(false) => {
                    println!("✗ Invalid master key. Please try again.\n");
                }
                Err(e) => {
                    println!("✗ Error verifying master key: {}\n", e);
                }
            }
        }
    } else {
        println!("⚠️  No master key configured yet.");
        println!("Please set a strong master key to encrypt your TOTP secrets.\n");
        println!("This key will be used to encrypt all sensitive data.");
        println!("You will need to enter it every time you start the server.\n");

        loop {
            print!("Enter new master key: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let master_key = input.trim().to_string();

            if master_key.is_empty() {
                println!("✗ Master key cannot be empty. Please try again.\n");
                continue;
            }

            if master_key.len() < 8 {
                println!("✗ Master key must be at least 8 characters long. Please try again.\n");
                continue;
            }

            print!("Confirm master key: ");
            io::stdout().flush().unwrap();

            let mut confirm_input = String::new();
            io::stdin().read_line(&mut confirm_input).unwrap();
            let confirm_key = confirm_input.trim().to_string();

            if master_key != confirm_key {
                println!("✗ Master keys do not match. Please try again.\n");
                continue;
            }

            // 哈希并保存主密钥
            match crypto::hash_master_key(&master_key) {
                Ok(hash) => {
                    app_state.master_key_hash = Some(hash);
                    let _ = storage::save_state(app_state);
                    println!("✓ Master key set successfully!\n");
                    return master_key;
                }
                Err(e) => {
                    println!("✗ Error setting master key: {}\n", e);
                }
            }
        }
    }
}
