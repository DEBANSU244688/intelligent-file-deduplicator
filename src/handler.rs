//! This module provides the main handler functions for the intelligent file deduplicator application.
//!
//! It re-exports core functionalities from submodules, including:
//! - `compare_files`: For comparing files to detect duplicates.
//! - `scan_directory_for_duplicates`: For scanning directories and identifying duplicate files.
//! - `write_json_report`: For generating JSON reports of duplicate findings.
//! - `parse_filter_options`: For parsing filter options to customize scanning behavior.
//! - `delete_duplicates`: For safely deleting identified duplicate files.

use crate::{
    file_compare::compare_files, filter::parse_filter_options, report::write_json_report,
    safe_delete::delete_duplicates, scanner::scan_directory_for_duplicates, ui::*,
};

use std::process;

/// Handles the `compare` command
pub fn handle_compare_command(args: &[String]) {
    if args.len() != 4 {
        print_error("Error: compare requires 2 file paths.");
        process::exit(1);
    }

    let file1 = &args[2];
    let file2 = &args[3];

    match compare_files(file1, file2) {
        Ok(true) => print_success("Files are identical."),
        Ok(false) => print_warning("Files are different."),
        Err(e) => print_error(&format!("Error comparing files: {e}")),
    }
}

/// Handles the `scan` command
pub fn handle_scan_command(args: &[String]) {
    if args.len() < 3 {
        print_error("Error: scan requires a directory path.");
        process::exit(1);
    }

    let dir = &args[2];
    let filter_args = &args[3..];
    let filters = parse_filter_options(filter_args);
    print_info(&format!("📁 Scanning directory: {}", dir));
    let duplicates = scan_directory_for_duplicates(dir, &filters);

    if duplicates.is_empty() {
        print_success("No duplicates found.");
        return;
    }

    let mut found = false;
    println!("🔍 Duplicate files found:");
    for (hash, files) in &duplicates {
        if files.len() > 1 {
            found = true;
            println!("\n🧬 Hash: {}", hash);
            print_list(files);
        }
    }

    if !found {
        print_success("No duplicate files matched the given filters.");
    }
}

/// Handles the `report` command
pub fn handle_report_command(args: &[String]) {
    if args.len() < 4 {
        print_error("Error: report requires a directory and an output file.");
        process::exit(1);
    }

    let dir = &args[2];
    let output_path = &args[3];
    let filter_args = &args[4..];
    let filters = parse_filter_options(filter_args);
    print_info(&format!("📄 Generating report for: {}", dir));
    let duplicates = scan_directory_for_duplicates(dir, &filters);

    match write_json_report(&duplicates, output_path) {
        Ok(_) => print_success(&format!("Report saved to `{}`", output_path)),
        Err(e) => print_error(&format!("Failed to write report: {e}")),
    }
}

/// Handles the `delete` command
pub fn handle_delete_command(args: &[String]) {
    if args.len() < 3 {
        print_error("Error: delete requires a directory.");
        process::exit(1);
    }

    let dir = &args[2];
    let dry_run = args.contains(&"--dry-run".to_string());
    let filter_args: Vec<String> = args.iter().skip(3).cloned().collect();
    let filters = parse_filter_options(&filter_args);

    print_info(&format!(
        "🧼 Deleting duplicates in `{}` (dry-run = {})",
        dir, dry_run
    ));

    let duplicates = scan_directory_for_duplicates(dir, &filters);

    if duplicates.is_empty() {
        print_success("No duplicates to delete.");
        return;
    }

    match delete_duplicates(&duplicates, dry_run) {
        Ok(_) => {
            if dry_run {
                print_success("Dry run complete. No files were deleted.");
            } else {
                print_success("Duplicate files deleted successfully.");
            }
        }
        Err(e) => print_error(&format!("Failed to delete duplicates: {e}")),
    }
}

/// Handles the `filter` command
pub fn handle_filter_command(args: &[String]) {
    if args.len() < 3 {
        print_error("Error: filter requires a directory path.");
        process::exit(1);
    }

    let dir = &args[2];
    let filters = parse_filter_options(&args[3..]);
    let filtered = scan_directory_for_duplicates(dir, &filters);

    if filtered.is_empty() {
        print_success("No matching files found.");
        return;
    }

    let mut found = false;
    println!("🔍 Matching files:");
    for (hash, files) in &filtered {
        if files.len() > 1 {
            found = true;
            println!("\n🧬 Hash: {}", hash);
            print_list(files);
        }
    }

    if !found {
        print_success("No duplicate files matched the given filters.");
    }
}
