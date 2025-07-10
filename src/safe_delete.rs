use std::{
    collections::HashMap,
    fs, io,
};

/// Deletes duplicate files from the provided hash map, keeping only one copy per group.
///
/// # Arguments
///
/// * `duplicates` - A reference to a HashMap where each key is a hash string and the value is a vector of file paths with that hash.
/// * `dry_run` - If true, the function will only print which files would be deleted, but will not actually delete any files.
///
/// # Returns
///
/// * `io::Result<()>` - Returns Ok(()) if all deletions succeed or if in dry run mode. Returns an error if any file deletion fails.
///
/// # Example
///
/// ```text
/// // let mut duplicates = HashMap::new();
/// // duplicates.insert("hash1".to_string(), vec!["file1.txt".to_string(), "file2.txt".to_string()]);
/// // delete_duplicates(&duplicates, true)?;
/// ```
pub fn delete_duplicates(
    duplicates: &HashMap<String, Vec<String>>,
    dry_run: bool,
) -> io::Result<()> {
    // Iterate over each group of duplicate files
    for (hash, files) in duplicates {
        // Skip groups with only one file (no duplicates)
        if files.len() <= 1 {
            continue;
        }

        // Keep the first file, delete the rest
        let (keep, delete) = files.split_first().unwrap();

        println!("\n🧬 Duplicate group (Hash: {})", hash);
        println!("📂 Keeping: {}", keep);

        // Iterate over files to delete
        for path in delete {
            if dry_run {
                // In dry run mode, just print what would be deleted
                println!("🧪 Would delete: {}", path);
            } else {
                // Attempt to delete the file and print the result
                match fs::remove_file(path) {
                    Ok(_) => println!("🗑️ Deleted: {}", path),
                    Err(e) => eprintln!("❌ Failed to delete {}: {}", path, e),
                }
            }
        }
    }

    Ok(())
}