use intelligent_file_deduplicator::hashing::hash_file;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_hash_consistency() {
    let mut temp = NamedTempFile::new().unwrap();
    writeln!(temp, "HashLaser").unwrap();

    let path = temp.path().to_str().unwrap();
    let hash1 = hash_file(path).unwrap();
    let hash2 = hash_file(path).unwrap();

    assert_eq!(hash1, hash2);
}
