use serde::Serialize;
use std::{collections::HashMap, fs::File, io::Write, path::Path};

#[derive(Serialize)]
struct DuplicateGroup {
    hash: String,
    files: Vec<String>,
}

#[derive(Serialize)]
struct Report {
    duplicates: Vec<DuplicateGroup>,
}

pub fn write_json_report<P: AsRef<Path>>(
    hash_map: &HashMap<String, Vec<String>>,
    output_path: P,
) -> std::io::Result<()> {
    // A vector of `DuplicateGroup` structs, where each group represents a set of files
    // that share the same hash (i.e., are duplicates).
    //
    // This vector is constructed by filtering the entries of a hash map to include only
    // those where multiple files share the same hash, and then mapping each such entry
    // to a `DuplicateGroup` containing the hash and the list of duplicate files.
    //
    // Example:
    // // Given a hash_map of file hashes to file paths:
    // let duplicates: Vec<DuplicateGroup> = hash_map
    //     .iter()
    //     .filter(|(_, files)| files.len() > 1)
    //     .map(|(hash, files)| DuplicateGroup {
    //         hash: hash.clone(),
    //         files: files.clone(),
    //     })
    //     .collect();
    // // `duplicates` now contains groups of files with identical content.
    let duplicates: Vec<DuplicateGroup> = hash_map
        .iter()
        .filter(|(_, files)| files.len() > 1)
        .map(|(hash, files)| DuplicateGroup {
            hash: hash.clone(),
            files: files.clone(),
        })
        .collect();

    let report = Report { duplicates };
    let json = serde_json::to_string_pretty(&report).expect("Serialization failed");

    let mut file = File::create(output_path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
