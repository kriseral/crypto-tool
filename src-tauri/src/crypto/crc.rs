use crc32fast::Hasher;

pub fn crc32(input: &str) -> String {
    let mut hasher = Hasher::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:08x}", result)
}

pub fn crc32_hex(input: &str) -> String {
    let mut hasher = Hasher::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:08X}", result)
}
