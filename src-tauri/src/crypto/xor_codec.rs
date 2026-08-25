use base64::{Engine, engine::general_purpose};

pub fn encrypt(plaintext: &str, password: &str) -> Result<String, String> {
    let key_bytes = password.as_bytes();
    if key_bytes.is_empty() {
        return Err("密钥不能为空".to_string());
    }
    let encrypted: Vec<u8> = plaintext
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
        .collect();
    Ok(general_purpose::STANDARD.encode(&encrypted))
}

pub fn decrypt(ciphertext: &str, password: &str) -> Result<String, String> {
    let key_bytes = password.as_bytes();
    if key_bytes.is_empty() {
        return Err("密钥不能为空".to_string());
    }
    let data = general_purpose::STANDARD
        .decode(ciphertext.trim())
        .map_err(|e| format!("Base64 解码失败: {}", e))?;
    let decrypted: Vec<u8> = data
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
        .collect();
    String::from_utf8(decrypted).map_err(|e| format!("UTF-8 转换失败: {}", e))
}
