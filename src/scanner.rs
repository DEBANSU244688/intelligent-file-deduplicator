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
///
/// # Arguments
///
/// * `dir` - The root directory to scan for files.
/// * `filters` - Filter options to apply to each file.
///
/// # Returns
///
/// A `HashMap` where the key is the SHA-256 hash of the file contents,
/// and the value is a vector of file paths (as strings) that have that hash.
///
/// # Example
///
/// ```text
/// // let duplicates = scan_directory_for_duplicates("/some/path", &filter_options);
/// ```
pub fn scan_directory_for_duplicates(dir: &str, filters: &FilterOptions) -> HashMap<String, Vec<String>> {
    // Collect all files recursively from the directory
    let files = collect_files_recursively(Path::new(dir));

    // Filter files based on the provided filter options
    let filtered_files: Vec<PathBuf> = files
        .into_iter()
        .filter(|path| filters.matches(path))
        .collect();

    // Shared, thread-safe map to store hashes and their corresponding file paths
    let hash_map: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));

    // Compute hashes in parallel and populate the map
    filtered_files.par_iter().for_each(|file_path| {
        if let Ok(hash) = hash_file(file_path.to_str().unwrap()) {
            let mut map = hash_map.lock().unwrap();
            map.entry(hash).or_default().push(file_path.to_string_lossy().to_string());
        }
    });

    // Unwrap the Arc and Mutex to return the final HashMap
    Arc::try_unwrap(hash_map).unwrap().into_inner().unwrap()
}

/// Recursively collects all file paths under the given directory.
///
/// # Arguments
///
/// * `dir` - The directory path to start scanning from.
///
/// # Returns
///
/// A vector of `PathBuf` objects representing all files found under the directory.
///
/// # Example
///
/// ```text
/// // let files = collect_files_recursively(Path::new("/some/path"));
/// ```
fn collect_files_recursively(dir: &Path) -> Vec<PathBuf> {
    // Vector to store all found file paths
    let mut all_files = Vec::new();
    // Read directory entries, if possible
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Recurse into subdirectories
                all_files.extend(collect_files_recursively(&path));
            } else if path.is_file() {
                // Add file to the list
                all_files.push(path);
            }
        }
    }
    all_files
}