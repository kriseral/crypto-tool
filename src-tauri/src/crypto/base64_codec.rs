use base64::{Engine, engine::general_purpose};

pub fn encode(input: &str) -> Result<String, String> {
    Ok(general_purpose::STANDARD.encode(input.as_bytes()))
}

pub fn decode(input: &str) -> Result<String, String> {
    let bytes = general_purpose::STANDARD
        .decode(input.trim())
        .map_err(|e| format!("Base64 解码失败: {}", e))?;
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 转换失败: {}", e))
}

pub fn encode_url(input: &str) -> Result<String, String> {
    Ok(general_purpose::URL_SAFE_NO_PAD.encode(input.as_bytes()))
}

pub fn decode_url(input: &str) -> Result<String, String> {
    let bytes = general_purpose::URL_SAFE_NO_PAD
        .decode(input.trim())
        .map_err(|e| format!("Base64URL 解码失败: {}", e))?;
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 转换失败: {}", e))
}

pub fn encode_hex(input: &str) -> Result<String, String> {
    Ok(hex::encode(input.as_bytes()))
}

pub fn decode_hex(input: &str) -> Result<String, String> {
    let bytes = hex::decode(input.trim())
        .map_err(|e| format!("Hex 解码失败: {}", e))?;
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 转换失败: {}", e))
}
