use ring::digest;

/// 计算 SHA1 哈希并返回十六进制字符串
pub fn sha1_hex(data: &[u8]) -> String {
    let digest = digest::digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, data);
    hex::encode(digest.as_ref())
}

/// 计算 SHA256 哈希并返回十六进制字符串
pub fn sha256_hex(data: &[u8]) -> String {
    let digest = digest::digest(&digest::SHA256, data);
    hex::encode(digest.as_ref())
}

/// 计算 SHA512 哈希并返回十六进制字符串
pub fn sha512_hex(data: &[u8]) -> String {
    let digest = digest::digest(&digest::SHA512, data);
    hex::encode(digest.as_ref())
}
