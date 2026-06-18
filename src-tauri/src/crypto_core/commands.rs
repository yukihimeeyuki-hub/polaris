use crate::crypto_core;
use base64::{Engine as _, engine::general_purpose};

/// 加密命令
#[tauri::command]
pub fn crypto_md5(data: String) -> Result<String, String> {
    Ok(crypto_core::md5_hex(data.as_bytes()))
}

#[tauri::command]
pub fn crypto_sha1(data: String) -> Result<String, String> {
    Ok(crypto_core::sha1_hex(data.as_bytes()))
}

#[tauri::command]
pub fn crypto_sha256(data: String) -> Result<String, String> {
    Ok(crypto_core::sha256_hex(data.as_bytes()))
}

#[tauri::command]
pub fn crypto_sha512(data: String) -> Result<String, String> {
    Ok(crypto_core::sha512_hex(data.as_bytes()))
}

#[tauri::command]
pub fn crypto_aes_encrypt(data: String, key: String) -> Result<String, String> {
    let encrypted = crypto_core::aes_encrypt(data.as_bytes(), key.as_bytes())?;
    Ok(general_purpose::STANDARD.encode(&encrypted))
}

#[tauri::command]
pub fn crypto_aes_decrypt(data: String, key: String) -> Result<String, String> {
    let encrypted = general_purpose::STANDARD.decode(&data)
        .map_err(|e| format!("Invalid base64: {}", e))?;
    let decrypted = crypto_core::aes_decrypt(&encrypted, key.as_bytes())?;
    String::from_utf8(decrypted).map_err(|e| format!("Invalid UTF-8: {}", e))
}

#[tauri::command]
pub fn crypto_rsa_generate_keypair() -> Result<(String, String), String> {
    crypto_core::rsa_generate_keypair()
}

#[tauri::command]
pub fn crypto_rsa_encrypt(data: String, public_key: String) -> Result<String, String> {
    crypto_core::rsa_encrypt(data.as_bytes(), &public_key)
}

#[tauri::command]
pub fn crypto_rsa_decrypt(encrypted_data: String, private_key: String) -> Result<String, String> {
    let decrypted = crypto_core::rsa_decrypt(&encrypted_data, &private_key)?;
    String::from_utf8(decrypted).map_err(|e| format!("Invalid UTF-8: {}", e))
}

#[tauri::command]
pub fn crypto_sign(data: String, private_key: String) -> Result<String, String> {
    crypto_core::sign(data.as_bytes(), &private_key)
}

#[tauri::command]
pub fn crypto_verify(data: String, signature: String, public_key: String) -> Result<bool, String> {
    crypto_core::verify(data.as_bytes(), &signature, &public_key)
}
