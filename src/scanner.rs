use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use crate::hashing::hash_file;
use crate::filter::FilterOptions;

/// Scans a directory recursively and finds duplicate files based on SHA-256 hash.
/// Applies filter options and uses rayon for parallelism.
pub fn scan_directory_for_duplicates(dir: &str, filters: &FilterOptions) -> HashMap<String, Vec<String>> {
    let files = collect_files_recursively(Path::new(dir));

    let filtered_files: Vec<PathBuf> = files
        .into_iter()
        .filter(|path| filters.matches(path))
        .collect();

    let hash_map: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));

    filtered_files.par_iter().for_each(|file_path| {
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
    if let Ok(entries) = fs::read_dir(dir) {
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