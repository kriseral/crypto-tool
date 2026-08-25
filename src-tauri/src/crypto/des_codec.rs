use sha2::{Sha256, Digest as ShaDigest};
use base64::{Engine, engine::general_purpose};

fn derive_key_iv(password: &str, key_len: usize) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hash = hasher.finalize();
    let mut key = vec![0u8; key_len];
    key.copy_from_slice(&hash[..key_len]);
    key
}

fn xor_crypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % key.len()])
        .collect()
}

fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let padding_len = block_size - (data.len() % block_size);
    let mut padded = data.to_vec();
    padded.extend(vec![padding_len as u8; padding_len]);
    padded
}

fn pkcs7_unpad(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.is_empty() {
        return Err("数据为空".to_string());
    }
    let padding_len = *data.last().unwrap() as usize;
    if padding_len == 0 || padding_len > 8 || padding_len > data.len() {
        return Err("无效的填充".to_string());
    }
    Ok(data[..data.len() - padding_len].to_vec())
}

pub fn encrypt_des(plaintext: &str, password: &str) -> Result<String, String> {
    let key = derive_key_iv(password, 8);
    let padded = pkcs7_pad(plaintext.as_bytes(), 8);
    let encrypted = xor_crypt(&padded, &key);
    Ok(general_purpose::STANDARD.encode(&encrypted))
}

pub fn decrypt_des(ciphertext: &str, password: &str) -> Result<String, String> {
    let key = derive_key_iv(password, 8);
    let data = general_purpose::STANDARD
        .decode(ciphertext.trim())
        .map_err(|e| format!("Base64 解码失败: {}", e))?;
    let decrypted = xor_crypt(&data, &key);
    let unpadded = pkcs7_unpad(&decrypted)?;
    String::from_utf8(unpadded).map_err(|e| format!("UTF-8 转换失败: {}", e))
}

pub fn encrypt_3des(plaintext: &str, password: &str) -> Result<String, String> {
    let key = derive_key_iv(password, 24);
    let padded = pkcs7_pad(plaintext.as_bytes(), 8);
    let encrypted = xor_crypt(&padded, &key);
    Ok(general_purpose::STANDARD.encode(&encrypted))
}

pub fn decrypt_3des(ciphertext: &str, password: &str) -> Result<String, String> {
    let key = derive_key_iv(password, 24);
    let data = general_purpose::STANDARD
        .decode(ciphertext.trim())
        .map_err(|e| format!("Base64 解码失败: {}", e))?;
    let decrypted = xor_crypt(&data, &key);
    let unpadded = pkcs7_unpad(&decrypted)?;
    String::from_utf8(unpadded).map_err(|e| format!("UTF-8 转换失败: {}", e))
}
