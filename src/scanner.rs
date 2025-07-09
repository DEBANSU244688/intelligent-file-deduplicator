use std::{
    collections::HashMap,
    fs,
    path::Path,
};
use crate::hashing::hash_file;

pub fn scan_directory_for_duplicates(dir: &str) -> HashMap<String, Vec<String>> {
    let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();

    visit_dirs(Path::new(dir), &mut hash_map).expect("Directory scan failed");

    hash_map
}

fn visit_dirs(dir: &Path, hash_map: &mut HashMap<String, Vec<String>>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, hash_map)?;
            } else if path.is_file() {
                if let Ok(hash) = hash_file(path.to_str().unwrap()) {
                    hash_map.entry(hash).or_default().push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}