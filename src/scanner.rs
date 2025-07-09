use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use crate::hashing::hash_file;

/// Scans a directory recursively and finds duplicate files based on SHA-256 hash.
/// Uses rayon to parallelize file hashing for performance.
pub fn scan_directory_for_duplicates(dir: &str) -> HashMap<String, Vec<String>> {
    let files = collect_files_recursively(Path::new(dir));

    let hash_map: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));

    files.par_iter().for_each(|file_path| {
        if let Ok(hash) = hash_file(file_path.to_str().unwrap()) {
            let mut map = hash_map.lock().unwrap();
            map.entry(hash).or_default().push(file_path.to_string_lossy().to_string());
        }
    });

    Arc::try_unwrap(hash_map).unwrap().into_inner().unwrap()
}

/// Recursively collects all file paths under the given directory.
fn collect_files_recursively(dir: &Path) -> Vec<PathBuf> {
    let mut all_files = Vec::new();
    let entries = fs::read_dir(dir);

    if let Ok(entries) = entries {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                all_files.extend(collect_files_recursively(&path));
            } else if path.is_file() {
                all_files.push(path);
            }
        }
    }

    all_files
}