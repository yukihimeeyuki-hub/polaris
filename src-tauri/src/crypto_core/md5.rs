use md5::{Md5, Digest};

/// 计算 MD5 哈希并返回十六进制字符串
pub fn md5_hex(data: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}
