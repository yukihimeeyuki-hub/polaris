use ring::rand::SystemRandom;
use ring::signature::{
    RsaKeyPair,
    RSA_PKCS1_SHA256,
};
use ring::signature;

// 签名
pub fn sign(
    private_key_der: &[u8],
    msg: &[u8],
) -> Vec<u8> {
    let rng = SystemRandom::new();

    let key_pair =
        RsaKeyPair::from_pkcs8(private_key_der)
            .unwrap();

    let mut sig =
        vec![0; key_pair.public().modulus_len()];

    key_pair.sign(
        &RSA_PKCS1_SHA256,
        &rng,
        msg,
        &mut sig,
    ).unwrap();

    sig
}
// 验证
pub fn verify_signature(
    public_key_der: &[u8],
    message: &[u8],
    signature_bytes: &[u8],
) -> bool {
    let public_key =
        signature::UnparsedPublicKey::new(
            &signature::RSA_PKCS1_2048_8192_SHA256,
            public_key_der,
        );

    public_key
        .verify(message, signature_bytes)
        .is_ok()
}