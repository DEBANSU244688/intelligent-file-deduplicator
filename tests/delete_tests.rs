use intelligent_file_deduplicator::safe_delete::delete_duplicates;
use std::{collections::HashMap, fs::File, io::Write};
use tempfile::tempdir;

#[test]
fn test_dry_run_deletion() {
    let dir = tempdir().unwrap();

    let file1 = dir.path().join("dup1.txt");
    let file2 = dir.path().join("dup2.txt");

    File::create(&file1)
        .unwrap()
        .write_all(b"same content")
        .unwrap();
    File::create(&file2)
        .unwrap()
        .write_all(b"same content")
        .unwrap();

    let mut duplicates = HashMap::new();
    duplicates.insert(
        "dummyhash".to_string(),
        vec![
            file1.to_string_lossy().to_string(),
            file2.to_string_lossy().to_string(),
        ],
    );

    // Dry run (no file should be deleted)
    let result = delete_duplicates(&duplicates, true);
    assert!(result.is_ok());
    assert!(file1.exists());
    assert!(file2.exists());
}

#[test]
fn test_actual_deletion() {
    let dir = tempdir().unwrap();

    let file1 = dir.path().join("file1.txt");
    let file2 = dir.path().join("file2.txt");

    File::create(&file1)
        .unwrap()
        .write_all(b"duplicate")
        .unwrap();
    File::create(&file2)
        .unwrap()
        .write_all(b"duplicate")
        .unwrap();

    let mut duplicates = HashMap::new();
    duplicates.insert(
        "hashkey".to_string(),
        vec![
            file1.to_string_lossy().to_string(),
            file2.to_string_lossy().to_string(),
        ],
    );

    let result = delete_duplicates(&duplicates, false);
    assert!(result.is_ok());

    // Only one file should be kept, so at least one must exist
    let remaining = [file1.exists(), file2.exists()];
    assert_eq!(remaining.iter().filter(|&&exists| exists).count(), 1);
}
