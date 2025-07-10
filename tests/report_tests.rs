use intelligent_file_deduplicator::report::write_json_report;
use std::collections::HashMap;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_json_report_generation() {
    let mut duplicates = HashMap::new();
    duplicates.insert(
        "dummyhash123".to_string(),
        vec![
            "path/to/file1.txt".to_string(),
            "path/to/file2.txt".to_string(),
        ],
    );

    let temp = tempdir().unwrap();
    let report_path = temp.path().join("report.json");

    write_json_report(&duplicates, &report_path).expect("Failed to write report");

    let contents = fs::read_to_string(&report_path).expect("Failed to read report");
    assert!(contents.contains("dummyhash123"));
    assert!(contents.contains("file1.txt"));
    assert!(contents.contains("file2.txt"));
}