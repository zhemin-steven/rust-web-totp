// Web TOTP - Production-Grade 2FA Management Tool
// Author: Steven
// License: MIT
// Version: 1.0.0

mod models;
mod storage;
mod auth;
mod totp_manager;
mod api;
mod error;

use actix_web::{web, App, HttpServer, middleware};
use actix_files as fs;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    // 加载环境变量
    dotenv::dotenv().ok();
    
    info!("Starting Web TOTP Server...");
    
    // Initialize storage (unlocked later via API)
    let storage = storage::Storage::new("data.enc").await
        .expect("Failed to initialize storage");
    let app_data = web::Data::new(storage);

    // Generate secret key for sessions
    let secret_key = Key::generate();

    info!("Server running at http://127.0.0.1:18007");
    println!("Server running at http://127.0.0.1:18007");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    secret_key.clone()
                )
                .cookie_secure(false)
                .build()
            )
            .service(
                web::scope("/api")
                    .service(api::unlock_database)
                    .service(api::get_lock_status)
                    .service(api::check_user_2fa)
                    .service(api::login)
                    .service(api::logout)
                    .service(api::check_session)
                    .service(api::change_password)
                    .service(api::enable_2fa)
                    .service(api::disable_2fa)
                    .service(api::verify_2fa)
                    .service(api::get_2fa_status)
                    .service(api::add_totp_entry)
                    .service(api::list_totp_entries)
                    .service(api::delete_totp_entry)
                    .service(api::generate_totp_code)
            )
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 18007))?
    .run()
    .await
}

