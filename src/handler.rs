use crate::{
    file_compare::compare_files,
    scanner::scan_directory_for_duplicates,
    report::write_json_report,
    safe_delete::delete_duplicates,
};

use std::process;

/// Prints the CLI usage instructions.
pub fn print_usage() {
    println!("📘 Usage:");
    println!("  dedup compare <file1> <file2>       Compare two files");
    println!("  dedup scan <directory_path>        Scan directory for duplicate files");
    println!("  dedup report <directory_path> <output.json>  Generate JSON report");
    println!("  dedup delete <directory_path>      Safely delete duplicate files");
}

/// Handle the `compare` command.
pub fn handle_compare(args: &[String]) {
    if args.len() != 4 {
        eprintln!("❌ Error: compare requires 2 file paths.");
        print_usage();
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

/// Handle the `scan` command.
pub fn handle_scan(args: &[String]) {
    if args.len() != 3 {
        eprintln!("❌ Error: scan requires a directory path.");
        print_usage();
        process::exit(1);
    }

    let dir = &args[2];
    let duplicates = scan_directory_for_duplicates(dir);

    if duplicates.is_empty() {
        println!("✅ No duplicates found in `{}`", dir);
    } else {
        println!("🔍 Duplicate files found:");
        for (hash, files) in &duplicates {
            if files.len() > 1 {
                println!("\n🧬 Hash: {}", hash);
                for file in files {
                    println!("  - {}", file);
                }
            }
        }
    }
}

/// Handle the `report` command.
pub fn handle_report(args: &[String]) {
    if args.len() != 4 {
        eprintln!("❌ Error: report requires directory path and output file.");
        print_usage();
        process::exit(1);
    }

    let dir = &args[2];
    let output_file = &args[3];

    let duplicates = scan_directory_for_duplicates(dir);
    match write_json_report(&duplicates, output_file) {
        Ok(_) => println!("📄 JSON report written to {}", output_file),
        Err(e) => eprintln!("❌ Failed to write report: {e}"),
    }
}

/// Handle the `delete` command.
pub fn handle_delete(args: &[String]) {
    if args.len() != 3 {
        eprintln!("❌ Error: delete requires a directory path.");
        print_usage();
        process::exit(1);
    }

    let dir: &String = &args[2];
    let duplicates = scan_directory_for_duplicates(dir);

    if duplicates.is_empty() {
        println!("✅ No duplicates to delete.");
    } else {
        match delete_duplicates(&duplicates, false) {
            Ok(()) => println!("\n🗑️ Deleted duplicate files successfully."),
            Err(e) => eprintln!("\n❌ Failed to delete duplicates: {e}"),
        }
    }
}
