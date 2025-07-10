use crate::{
    file_compare::compare_files,
    scanner::scan_directory_for_duplicates,
    report::write_json_report,
    filter::parse_filter_options,
    safe_delete::delete_duplicates,
};

use std::process;

/// Handles the `compare` command
pub fn handle_compare_command(args: &[String]) {
    if args.len() != 4 {
        eprintln!("❌ Error: compare requires 2 file paths.");
        process::exit(1);
    }

    let file1 = &args[2];
    let file2 = &args[3];

    match compare_files(file1, file2) {
        Ok(true) => println!("✅ Files are identical."),
        Ok(false) => println!("❌ Files are different."),
        Err(e) => eprintln!("Error comparing files: {e}"),
    }
}

/// Handles the `scan` command
pub fn handle_scan_command(args: &[String]) {
    if args.len() < 3 {
        eprintln!("❌ Error: scan requires a directory path.");
        process::exit(1);
    }

    let dir = &args[2];
    let filter_args = &args[3..];

    let filters = parse_filter_options(filter_args);
    let duplicates = scan_directory_for_duplicates(dir, &filters);

    if duplicates.is_empty() {
        println!("✅ No duplicates found in `{}`", dir);
        return;
    }

    let mut found = false;
    println!("🔍 Duplicate files found:");
    for (hash, files) in &duplicates {
        if files.len() > 1 {
            found = true;
            println!("\n🧬 Hash: {}", hash);
            for file in files {
                println!("  - {}", file);
            }
        }
    }

    if !found {
        println!("✅ No duplicate files matched the given filters.");
    }
}

/// Handles the `report` command
pub fn handle_report_command(args: &[String]) {
    if args.len() < 4 {
        eprintln!("❌ Error: report requires a directory and an output file.");
        process::exit(1);
    }

    let dir = &args[2];
    let output_path = &args[3];
    let filter_args = &args[4..];
    let filters = parse_filter_options(filter_args);

    let duplicates = scan_directory_for_duplicates(dir, &filters);

    if let Err(e) = write_json_report(&duplicates, output_path) {
        eprintln!("❌ Failed to write report: {e}");
    } else {
        println!("📄 JSON report saved to {}", output_path);
    }
}

/// Handles the `delete` command
pub fn handle_delete_command(args: &[String]) {
    if args.len() < 3 {
        eprintln!("❌ Error: delete requires a directory.");
        process::exit(1);
    }

    let dir = &args[2];
    let mut dry_run = false;

    let mut i = 3;
    while i < args.len() {
        if args[i] == "--dry-run" {
            dry_run = true;
        }
        i += 1;
    }

    let filters = parse_filter_options(&args[3..]);

    let duplicates = scan_directory_for_duplicates(dir, &filters);

    if duplicates.is_empty() {
        println!("✅ No duplicates to delete.");
    } else {
        if let Err(e) = delete_duplicates(&duplicates, dry_run) {
            eprintln!("❌ Failed to delete duplicates: {e}");
        }
    }
}

/// Handles the `filter` command
pub fn handle_filter_command(args: &[String]) {
    if args.len() < 3 {
        eprintln!("❌ Error: filter requires a directory path.");
        process::exit(1);
    }

    let dir = &args[2];
    let filters = parse_filter_options(&args[3..]);
    let filtered = scan_directory_for_duplicates(dir, &filters);

    if filtered.is_empty() {
        println!("✅ No matching files found.");
        return;
    }

    let mut found = false;
    println!("🔍 Matching files:");

    for (hash, files) in &filtered {
        if files.len() > 1 {
            found = true;
            println!("\n🧬 Hash: {}", hash);
            for file in files {
                println!("  - {}", file);
            }
        }
    }

    if !found {
        println!("✅ No duplicate files matched the given filters.");
    }
}