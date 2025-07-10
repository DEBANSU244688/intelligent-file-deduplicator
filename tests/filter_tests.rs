use intelligent_file_deduplicator::filter::FilterOptions;
use regex::Regex;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use tempfile::tempdir;

#[test]
fn test_filter_options_match() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_log.txt");

    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Sample log").unwrap(); // Create small file

    let metadata = fs::metadata(&file_path).unwrap();
    let file_size = metadata.len();

    let options = FilterOptions {
        min_size: Some(file_size - 1),
        max_size: Some(file_size + 10),
        extensions: Some(vec!["txt".to_string()]),
        regex: Some(Regex::new(r".*log.*").unwrap()),
    };

    assert!(options.matches(Path::new(&file_path)));
}

#[test]
fn test_filter_excludes_large_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("large.txt");

    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "{}", "X".repeat(10_000)).unwrap();

    let options = FilterOptions {
        min_size: None,
        max_size: Some(5000),
        extensions: None,
        regex: None,
    };

    assert!(!options.matches(Path::new(&file_path)));
}
