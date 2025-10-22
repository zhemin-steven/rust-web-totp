use totp_rs::{Algorithm, Secret, TOTP};
use qrcode::QrCode;
use base64::{Engine as _, engine::general_purpose};

pub fn generate_secret() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Generate random bytes
    let secret_bytes: Vec<u8> = (0..20).map(|_| rng.gen()).collect();
    
    // Encode to Base32 (only A-Z and 2-7)
    let base32_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut result = String::new();
    
    let mut bits = 0u32;
    let mut bit_count = 0;
    
    for &byte in &secret_bytes {
        bits = (bits << 8) | byte as u32;
        bit_count += 8;
        
        while bit_count >= 5 {
            bit_count -= 5;
            let index = ((bits >> bit_count) & 0x1F) as usize;
            result.push(base32_chars.chars().nth(index).unwrap());
        }
    }
    
    // Handle remaining bits
    if bit_count > 0 {
        let index = ((bits << (5 - bit_count)) & 0x1F) as usize;
        result.push(base32_chars.chars().nth(index).unwrap());
    }
    
    result
}

pub fn generate_totp_code(secret: &str) -> Result<(String, u64), Box<dyn std::error::Error>> {
    let secret_bytes = Secret::Encoded(secret.to_string())
        .to_bytes()
        .map_err(|e| format!("Failed to parse secret: {:?}", e))?;
    
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret_bytes,
        None,
        String::from(""),
    )?;
    
    let code = totp.generate_current()?;
    let remaining = 30 - (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() % 30);
    
    Ok((code, remaining))
}

pub fn verify_totp_code(secret: &str, code: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let secret_bytes = Secret::Encoded(secret.to_string())
        .to_bytes()
        .map_err(|e| format!("Failed to parse secret: {:?}", e))?;
    
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret_bytes,
        None,
        String::from(""),
    )?;
    
    Ok(totp.check_current(code)?)
}

pub fn generate_qr_code(secret: &str, username: &str, issuer: &str) -> Result<String, Box<dyn std::error::Error>> {
    let otpauth_url = format!(
        "otpauth://totp/{}:{}?secret={}&issuer={}",
        issuer, username, secret, issuer
    );
    
    let code = QrCode::new(otpauth_url.as_bytes())?;
    
    // Render to SVG string for simplicity
    let svg_data = code.render()
        .min_dimensions(200, 200)
        .dark_color(qrcode::render::svg::Color("#000000"))
        .light_color(qrcode::render::svg::Color("#ffffff"))
        .build();
    
    // Convert SVG to base64
    let svg_base64 = general_purpose::STANDARD.encode(svg_data.as_bytes());
    Ok(format!("data:image/svg+xml;base64,{}", svg_base64))
}

