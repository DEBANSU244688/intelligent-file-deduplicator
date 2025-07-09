use std::{
    collections::HashMap,
    fs, io,
};

/// Deletes duplicate files from the hash map, keeping only one copy per group.
/// If `dry_run` is true, no files are deleted, only listed.
pub fn delete_duplicates(
    duplicates: &HashMap<String, Vec<String>>,
    dry_run: bool,
) -> io::Result<()> {
    for (hash, files) in duplicates {
        if files.len() <= 1 {
            continue;
        }

        // Keep the first file, delete the rest
        let (keep, delete) = files.split_first().unwrap();

        println!("\n🧬 Duplicate group (Hash: {})", hash);
        println!("📂 Keeping: {}", keep);

        for path in delete {
            if dry_run {
                println!("🧪 Would delete: {}", path);
            } else {
                match fs::remove_file(path) {
                    Ok(_) => println!("🗑️ Deleted: {}", path),
                    Err(e) => eprintln!("❌ Failed to delete {}: {}", path, e),
                }
            }
        }
    }

    Ok(())
}