use std::{
    fs::File,
    io::{self, Read},
};
use sha2::{Sha256, Digest};

/// Computes the SHA-256 hash of the file at the given path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the file to be hashed.
///
/// # Returns
///
/// * `io::Result<String>` - On success, returns the hexadecimal string representation of the file's SHA-256 hash.
///   On failure, returns an `io::Error`.
pub fn hash_file(path: &str) -> io::Result<String> {
    // Open the file at the specified path
    let mut file = File::open(path)?;
    // Create a new SHA-256 hasher instance
    let mut hasher = Sha256::new();
    // Buffer to read file chunks
    let mut buffer = [0u8; 1024];

    // Read the file in chunks and update the hasher
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 { break; } // End of file reached
        hasher.update(&buffer[..bytes_read]);
    }

    // Finalize the hash and return it as a hexadecimal string
    Ok(format!("{:x}", hasher.finalize()))
}