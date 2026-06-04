use ring::{
    aead,
    rand::{SecureRandom, SystemRandom},
};

pub fn decrypt(
    key_bytes: &[u8; 32],
    nonce_bytes: [u8; 12],
    ciphertext: &[u8],
) -> Result<Vec<u8>, ring::error::Unspecified> {
    let unbound_key =
        aead::UnboundKey::new(&aead::AES_256_GCM, key_bytes)?;

    let key = aead::LessSafeKey::new(unbound_key);

    let nonce =
        aead::Nonce::assume_unique_for_key(nonce_bytes);

    let mut in_out = ciphertext.to_vec();

    let plaintext = key.open_in_place(
        nonce,
        aead::Aad::empty(),
        &mut in_out,
    )?;

    Ok(plaintext.to_vec())
}


pub fn encrypt(
    key_bytes: &[u8; 32],
    plaintext: &[u8],
) -> Result<(Vec<u8>, [u8; 12]), ring::error::Unspecified> {
    let unbound_key =
        aead::UnboundKey::new(&aead::AES_256_GCM, key_bytes)?;

    let key = aead::LessSafeKey::new(unbound_key);

    let rng = SystemRandom::new();

    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes)?;

    let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);

    let mut in_out = plaintext.to_vec();

    key.seal_in_place_append_tag(
        nonce,
        aead::Aad::empty(),
        &mut in_out,
    )?;

    Ok((in_out, nonce_bytes))
}