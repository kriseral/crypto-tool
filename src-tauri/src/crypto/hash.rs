use blake2::{Blake2b512, Blake2s256};

pub fn md5_hash(input: &str) -> String {
    use md5::Md5;
    use md5::Digest;
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn sha256_hash(input: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn sha512_hash(input: &str) -> String {
    use sha2::{Sha512, Digest};
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn blake2b_hash(input: &str) -> String {
    use blake2::Digest;
    let mut hasher = Blake2b512::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn blake2s_hash(input: &str) -> String {
    use blake2::Digest;
    let mut hasher = Blake2s256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
