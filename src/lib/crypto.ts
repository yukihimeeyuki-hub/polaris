import { invoke } from '@tauri-apps/api/core'

export const crypto = {
  // 哈希算法
  async md5(data: string): Promise<string> {
    return invoke('crypto_md5', { data })
  },

  async sha1(data: string): Promise<string> {
    return invoke('crypto_sha1', { data })
  },

  async sha256(data: string): Promise<string> {
    return invoke('crypto_sha256', { data })
  },

  async sha512(data: string): Promise<string> {
    return invoke('crypto_sha512', { data })
  },

  // AES 加密解密
  async aesEncrypt(data: string, key: string): Promise<string> {
    return invoke('crypto_aes_encrypt', { data, key })
  },

  async aesDecrypt(encryptedData: string, key: string): Promise<string> {
    return invoke('crypto_aes_decrypt', { data: encryptedData, key })
  },

  // RSA 密钥对生成
  async rsaGenerateKeyPair(): Promise<{ privateKey: string; publicKey: string }> {
    const [privateKey, publicKey] = await invoke<[string, string]>('crypto_rsa_generate_keypair')
    return { privateKey, publicKey }
  },

  // RSA 加密解密
  async rsaEncrypt(data: string, publicKey: string): Promise<string> {
    return invoke('crypto_rsa_encrypt', { data, publicKey })
  },

  async rsaDecrypt(encryptedData: string, privateKey: string): Promise<string> {
    return invoke('crypto_rsa_decrypt', { encryptedData, privateKey })
  },

  // RSA 签名验证
  async sign(data: string, privateKey: string): Promise<string> {
    return invoke('crypto_sign', { data, privateKey })
  },

  async verify(data: string, signature: string, publicKey: string): Promise<boolean> {
    return invoke('crypto_verify', { data, signature, publicKey })
  },
}
