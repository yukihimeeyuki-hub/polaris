use ring::rand::SystemRandom;
use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM};
use ring::error::Unspecified;

const NONCE_LEN: usize = 12;

struct CounterNonceSequence {
    nonce: [u8; NONCE_LEN],
}

impl CounterNonceSequence {
    fn new(nonce: [u8; NONCE_LEN]) -> Self {
        Self { nonce }
    }
}

impl NonceSequence for CounterNonceSequence {
    fn advance(&mut self) -> Result<Nonce, Unspecified> {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        nonce_bytes.copy_from_slice(&self.nonce);
        
        // 递增计数器
        for byte in self.nonce.iter_mut() {
            *byte = byte.wrapping_add(1);
            if *byte != 0 {
                break;
            }
        }
        
        Ok(Nonce::assume_unique_for_key(nonce_bytes))
    }
}

/// AES-256-GCM 加密
/// 返回格式: nonce + encrypted_data
pub fn aes_encrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes for AES-256".to_string());
    }

    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; NONCE_LEN];
    ring::rand::SecureRandom::fill(&rng, &mut nonce_bytes)
        .map_err(|_| "Failed to generate nonce")?;

    let unbound_key = UnboundKey::new(&AES_256_GCM, key)
        .map_err(|_| "Failed to create key")?;
    
    let nonce_sequence = CounterNonceSequence::new(nonce_bytes);
    let mut sealing_key = SealingKey::new(unbound_key, nonce_sequence);

    let mut in_out = plaintext.to_vec();
    sealing_key.seal_in_place_append_tag(Aad::empty(), &mut in_out)
        .map_err(|_| "Encryption failed")?;

    let mut result = Vec::with_capacity(NONCE_LEN + in_out.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&in_out);

    Ok(result)
}

/// AES-256-GCM 解密
/// 输入格式: nonce + encrypted_data
pub fn aes_decrypt(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes for AES-256".to_string());
    }

    if ciphertext.len() < NONCE_LEN {
        return Err("Ciphertext too short".to_string());
    }

    let (nonce_bytes, encrypted_data) = ciphertext.split_at(NONCE_LEN);
    let mut nonce_array = [0u8; NONCE_LEN];
    nonce_array.copy_from_slice(nonce_bytes);

    let unbound_key = UnboundKey::new(&AES_256_GCM, key)
        .map_err(|_| "Failed to create key")?;
    
    let nonce_sequence = CounterNonceSequence::new(nonce_array);
    let mut opening_key = OpeningKey::new(unbound_key, nonce_sequence);

    let mut in_out = encrypted_data.to_vec();
    let decrypted = opening_key.open_in_place(Aad::empty(), &mut in_out)
        .map_err(|_| "Decryption failed")?;

    Ok(decrypted.to_vec())
}
