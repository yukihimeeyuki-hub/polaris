use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt, pkcs1v15};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, DecodePrivateKey, DecodePublicKey, LineEnding};
use rsa::signature::{Signer, SignatureEncoding};
use base64::{Engine as _, engine::general_purpose};
use sha2::Sha256;
use rand::rngs::OsRng;

/// 生成 RSA 密钥对 (PKCS8 格式，Base64 编码)
/// 返回 (private_key_base64, public_key_base64)
pub fn rsa_generate_keypair() -> Result<(String, String), String> {
    let mut rng = OsRng;
    
    // 生成 2048 位 RSA 密钥对
    let private_key = RsaPrivateKey::new(&mut rng, 2048)
        .map_err(|e| format!("Failed to generate RSA key pair: {}", e))?;
    
    let public_key = RsaPublicKey::from(&private_key);
    
    // 导出为 PKCS8 格式并 Base64 编码
    let private_pem = private_key
        .to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| format!("Failed to export private key: {}", e))?;
    
    let public_pem = public_key
        .to_public_key_pem(LineEnding::LF)
        .map_err(|e| format!("Failed to export public key: {}", e))?;
    
    let private_base64 = general_purpose::STANDARD.encode(private_pem.as_bytes());
    let public_base64 = general_purpose::STANDARD.encode(public_pem.as_bytes());
    
    Ok((private_base64, public_base64))
}

/// RSA 加密 (使用公钥加密，PKCS1v15 padding)
pub fn rsa_encrypt(data: &[u8], public_key_base64: &str) -> Result<String, String> {
    let public_key_pem = general_purpose::STANDARD.decode(public_key_base64)
        .map_err(|e| format!("Invalid public key base64: {}", e))?;
    
    let public_key_pem_str = String::from_utf8(public_key_pem)
        .map_err(|e| format!("Invalid PEM encoding: {}", e))?;
    
    let public_key = RsaPublicKey::from_public_key_pem(&public_key_pem_str)
        .map_err(|e| format!("Failed to parse public key: {}", e))?;
    
    let mut rng = OsRng;
    let encrypted = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, data)
        .map_err(|e| format!("Encryption failed: {}", e))?;
    
    Ok(general_purpose::STANDARD.encode(encrypted))
}

/// RSA 解密 (使用私钥解密，PKCS1v15 padding)
pub fn rsa_decrypt(encrypted_data: &str, private_key_base64: &str) -> Result<Vec<u8>, String> {
    let private_key_pem = general_purpose::STANDARD.decode(private_key_base64)
        .map_err(|e| format!("Invalid private key base64: {}", e))?;
    
    let private_key_pem_str = String::from_utf8(private_key_pem)
        .map_err(|e| format!("Invalid PEM encoding: {}", e))?;
    
    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem_str)
        .map_err(|e| format!("Failed to parse private key: {}", e))?;
    
    let encrypted_bytes = general_purpose::STANDARD.decode(encrypted_data)
        .map_err(|e| format!("Invalid encrypted data base64: {}", e))?;
    
    let decrypted = private_key
        .decrypt(Pkcs1v15Encrypt, &encrypted_bytes)
        .map_err(|e| format!("Decryption failed: {}", e))?;
    
    Ok(decrypted)
}

/// RSA 签名 (使用私钥签名，PKCS1v15 with SHA256)
pub fn sign(data: &[u8], private_key_base64: &str) -> Result<String, String> {
    let private_key_pem = general_purpose::STANDARD.decode(private_key_base64)
        .map_err(|e| format!("Invalid private key base64: {}", e))?;
    
    let private_key_pem_str = String::from_utf8(private_key_pem)
        .map_err(|e| format!("Invalid PEM encoding: {}", e))?;
    
    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key_pem_str)
        .map_err(|e| format!("Failed to parse private key: {}", e))?;
    
    let signing_key = pkcs1v15::SigningKey::<Sha256>::new(private_key);
    let signature = signing_key.sign(data);
    
    Ok(general_purpose::STANDARD.encode(signature.to_bytes()))
}

/// RSA 签名验证 (使用公钥验证，PKCS1v15 with SHA256)
pub fn verify(data: &[u8], signature_base64: &str, public_key_base64: &str) -> Result<bool, String> {
    let public_key_pem = general_purpose::STANDARD.decode(public_key_base64)
        .map_err(|e| format!("Invalid public key base64: {}", e))?;
    
    let public_key_pem_str = String::from_utf8(public_key_pem)
        .map_err(|e| format!("Invalid PEM encoding: {}", e))?;
    
    let public_key = RsaPublicKey::from_public_key_pem(&public_key_pem_str)
        .map_err(|e| format!("Failed to parse public key: {}", e))?;
    
    let signature_bytes = general_purpose::STANDARD.decode(signature_base64)
        .map_err(|e| format!("Invalid signature base64: {}", e))?;
    
    let verifying_key = pkcs1v15::VerifyingKey::<Sha256>::new(public_key);
    let signature = pkcs1v15::Signature::try_from(signature_bytes.as_slice())
        .map_err(|e| format!("Invalid signature format: {}", e))?;
    
    use rsa::signature::Verifier;
    Ok(verifying_key.verify(data, &signature).is_ok())
}
