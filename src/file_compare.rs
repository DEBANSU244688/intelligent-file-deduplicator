use crate::hashing::hash_file;
use std::io;

pub fn compare_files(path1: &str, path2: &str) -> io::Result<bool> {
    let hash1 = hash_file(path1)?;
    let hash2 = hash_file(path2)?;
    Ok(hash1 == hash2)
}