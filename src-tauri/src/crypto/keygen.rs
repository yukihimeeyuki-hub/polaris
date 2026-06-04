use ring::rand::{SecureRandom, SystemRandom};
use base64::{engine::general_purpose, Engine};
// crypto/keygen.rs

#[derive(Debug, Clone)]
pub struct Aes256Key {
    pub key: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct Aes128Key {
    pub key: [u8; 16],
}
pub fn generate_aes256_key() -> Result<Aes256Key, ring::error::Unspecified> {
    let rng = SystemRandom::new();

    let mut key = [0u8; 32];
    rng.fill(&mut key)?;

    Ok(Aes256Key { key })
}

pub fn generate_aes128_key() -> Result<Aes128Key, ring::error::Unspecified> {
    let rng = SystemRandom::new();

    let mut key = [0u8; 16];
    rng.fill(&mut key)?;

    Ok(Aes128Key { key })
}
pub fn generate_nonce() -> Result<[u8; 12], ring::error::Unspecified> {
    let rng = SystemRandom::new();

    let mut nonce = [0u8; 12];
    rng.fill(&mut nonce)?;

    Ok(nonce)
}
impl Aes256Key {
    pub fn to_base64(&self) -> String {
        general_purpose::STANDARD.encode(self.key)
    }
}