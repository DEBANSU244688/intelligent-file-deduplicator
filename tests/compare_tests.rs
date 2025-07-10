use intelligent_file_deduplicator::file_compare::compare_files;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_compare_identical_files() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, "HashLaser is awesome!").unwrap();
    writeln!(file2, "HashLaser is awesome!").unwrap();

    let path1 = file1.path().to_str().unwrap();
    let path2 = file2.path().to_str().unwrap();

    let result = compare_files(path1, path2).unwrap();
    assert!(result, "Files should be identical");
}

#[test]
fn test_compare_different_files() {
    let mut file1 = NamedTempFile::new().unwrap();
    let mut file2 = NamedTempFile::new().unwrap();

    writeln!(file1, "HashLaser FTW").unwrap();
    writeln!(file2, "HashLaser is the best").unwrap();

    let path1 = file1.path().to_str().unwrap();
    let path2 = file2.path().to_str().unwrap();

    let result = compare_files(path1, path2).unwrap();
    assert!(!result, "Files should be different");
}
