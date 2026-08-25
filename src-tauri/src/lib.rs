mod crypto;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoResult {
    pub success: bool,
    pub data: String,
    pub error: Option<String>,
}

#[tauri::command]
fn process_text(
    input: String,
    key: String,
    algorithm: String,
    mode: String,
) -> CryptoResult {
    let result = match (algorithm.as_str(), mode.as_str()) {
        ("base64", "encrypt") => crypto::base64_codec::encode(&input),
        ("base64", "decrypt") => crypto::base64_codec::decode(&input),
        ("base64url", "encrypt") => crypto::base64_codec::encode_url(&input),
        ("base64url", "decrypt") => crypto::base64_codec::decode_url(&input),
        ("hex", "encrypt") => crypto::base64_codec::encode_hex(&input),
        ("hex", "decrypt") => crypto::base64_codec::decode_hex(&input),
        ("aes", "encrypt") => crypto::aes_codec::encrypt(&input, &key),
        ("aes", "decrypt") => crypto::aes_codec::decrypt(&input, &key),
        ("des", "encrypt") => crypto::des_codec::encrypt_des(&input, &key),
        ("des", "decrypt") => crypto::des_codec::decrypt_des(&input, &key),
        ("3des", "encrypt") => crypto::des_codec::encrypt_3des(&input, &key),
        ("3des", "decrypt") => crypto::des_codec::decrypt_3des(&input, &key),
        ("xor", "encrypt") => crypto::xor_codec::encrypt(&input, &key),
        ("xor", "decrypt") => crypto::xor_codec::decrypt(&input, &key),
        ("md5", _) => Ok(crypto::hash::md5_hash(&input)),
        ("sha256", _) => Ok(crypto::hash::sha256_hash(&input)),
        ("sha512", _) => Ok(crypto::hash::sha512_hash(&input)),
        ("blake2b", _) => Ok(crypto::hash::blake2b_hash(&input)),
        ("blake2s", _) => Ok(crypto::hash::blake2s_hash(&input)),
        ("crc32", _) => Ok(crypto::crc::crc32(&input)),
        _ => Err(format!("不支持的算法: {} / {}", algorithm, mode)),
    };

    match result {
        Ok(data) => CryptoResult {
            success: true,
            data,
            error: None,
        },
        Err(e) => CryptoResult {
            success: false,
            data: String::new(),
            error: Some(e),
        },
    }
}

#[tauri::command]
fn process_file(
    file_path: String,
    output_path: String,
    key: String,
    algorithm: String,
    mode: String,
) -> CryptoResult {
    use std::fs;

    let input = match fs::read(&file_path) {
        Ok(data) => data,
        Err(e) => {
            return CryptoResult {
                success: false,
                data: String::new(),
                error: Some(format!("读取文件失败: {}", e)),
            }
        }
    };

    let result = match (algorithm.as_str(), mode.as_str()) {
        ("base64", "encrypt") => {
            use base64::Engine;
            Ok(base64::engine::general_purpose::STANDARD.encode(&input))
        }
        ("base64", "decrypt") => {
            use base64::Engine;
            base64::engine::general_purpose::STANDARD
                .decode(&input)
                .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
                .map_err(|e| format!("Base64 解码失败: {}", e))
        }
        ("aes", "encrypt") => {
            let plaintext = String::from_utf8_lossy(&input).to_string();
            crypto::aes_codec::encrypt(&plaintext, &key)
        }
        ("aes", "decrypt") => {
            let decoded = String::from_utf8_lossy(&input).to_string();
            crypto::aes_codec::decrypt(&decoded, &key)
        }
        ("des", "encrypt") => {
            let plaintext = String::from_utf8_lossy(&input).to_string();
            crypto::des_codec::encrypt_des(&plaintext, &key)
        }
        ("des", "decrypt") => {
            let decoded = String::from_utf8_lossy(&input).to_string();
            crypto::des_codec::decrypt_des(&decoded, &key)
        }
        ("xor", "encrypt") | ("xor", "decrypt") => {
            let plaintext = String::from_utf8_lossy(&input).to_string();
            crypto::xor_codec::encrypt(&plaintext, &key)
        }
        _ => {
            return CryptoResult {
                success: false,
                data: String::new(),
                error: Some(format!("文件不支持该算法: {}", algorithm)),
            }
        }
    };

    match result {
        Ok(data) => {
            if let Err(e) = fs::write(&output_path, data.as_bytes()) {
                CryptoResult {
                    success: false,
                    data: String::new(),
                    error: Some(format!("写入文件失败: {}", e)),
                }
            } else {
                CryptoResult {
                    success: true,
                    data: format!("文件已保存: {}", output_path),
                    error: None,
                }
            }
        }
        Err(e) => CryptoResult {
            success: false,
            data: String::new(),
            error: Some(e),
        },
    }
}

#[tauri::command]
fn select_file() -> Option<String> {
    rfd::FileDialog::new()
        .pick_file()
        .map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
fn select_save_file() -> Option<String> {
    rfd::FileDialog::new()
        .save_file()
        .map(|p| p.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            process_text,
            process_file,
            select_file,
            select_save_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
