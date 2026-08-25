use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit},
};
use sha2::{Sha256, Digest};
use base64::{Engine, engine::general_purpose};
use rand::RngCore;

fn derive_key(password: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hash = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&hash);
    key
}

pub fn encrypt(plaintext: &str, password: &str) -> Result<String, String> {
    let key_bytes = derive_key(password);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| format!("AES 加密失败: {}", e))?;

    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(general_purpose::STANDARD.encode(&result))
}

pub fn decrypt(ciphertext: &str, password: &str) -> Result<String, String> {
    let key_bytes = derive_key(password);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let data = general_purpose::STANDARD
        .decode(ciphertext.trim())
        .map_err(|e| format!("Base64 解码失败: {}", e))?;

    if data.len() < 12 {
        return Err("密文数据太短".to_string());
    }

    let (nonce_bytes, ciphertext_bytes) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext_bytes)
        .map_err(|e| format!("AES 解密失败: {}", e))?;

    String::from_utf8(plaintext).map_err(|e| format!("UTF-8 转换失败: {}", e))
}
