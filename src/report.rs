use std::{
    fs::File,
    io::Write,
    path::Path,
    collections::HashMap,
};
use serde::Serialize;

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