/// Imports the `hash_file` function from the `hashing` module within the current crate.
///
/// The `hash_file` function is typically used to compute a hash value for a given file,
/// which can be useful for tasks such as file comparison, deduplication, or integrity checking.
use crate::hashing::hash_file;
use std::io;

pub fn compare_files(path1: &str, path2: &str) -> io::Result<bool> {
    let hash1 = hash_file(path1)?;
    let hash2 = hash_file(path2)?;
    Ok(hash1 == hash2)
}
