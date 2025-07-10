use intelligent_file_deduplicator::scanner::scan_directory_for_duplicates;
use intelligent_file_deduplicator::filter::FilterOptions;

use std::{
    fs::File,
    io::Write,
    path::PathBuf
};
use tempfile::tempdir;

#[test]
fn test_duplicate_detection() {
    let dir = tempdir().unwrap();
    let path1 = dir.path().join("file1.txt");
    let path2 = dir.path().join("file2.txt");
    let path3 = dir.path().join("file3.txt");

    // Write same content to file1 and file2
    let mut file1 = File::create(&path1).unwrap();
    let mut file2 = File::create(&path2).unwrap();
    let mut file3 = File::create(&path3).unwrap();

    writeln!(file1, "HashLaser").unwrap();
    writeln!(file2, "HashLaser").unwrap();
    writeln!(file3, "Different content").unwrap();

    let duplicates = scan_directory_for_duplicates(dir.path().to_str().unwrap(), &FilterOptions::default());

    let mut found = false;
    for (_hash, files) in &duplicates {
        if files.len() > 1 {
            found = true;
            let file_names: Vec<_> = files.iter().map(|f| PathBuf::from(f).file_name().unwrap().to_str().unwrap().to_string()).collect();
            assert!(file_names.contains(&"file1.txt".to_string()));
            assert!(file_names.contains(&"file2.txt".to_string()));
        }
    }

    assert!(found, "Should detect duplicates between file1 and file2");
}