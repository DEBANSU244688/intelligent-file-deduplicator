mod hashing;
mod file_compare;
mod scanner;
mod report;

use file_compare::compare_files;
use scanner::scan_directory_for_duplicates;
use report::write_json_report;

use std::{env, process};

fn print_usage() {
    println!("📘 Usage:");
    println!("  dedup compare <file1> <file2>       Compare two files");
    println!("  dedup scan <directory_path>        Scan directory for duplicate files");
}

fn run_compare(file1: &str, file2: &str) {
    match compare_files(file1, file2) {
        Ok(true) => println!("✅ Files are identical."),
        Ok(false) => println!("❌ Files are different."),
        Err(e) => eprintln!("❌ Error comparing files: {e}"),
    }
}

fn run_scan(dir: &str) {
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

        // ✅ Generate JSON report
        if let Err(e) = write_json_report(&duplicates, "report.json") {
            eprintln!("❌ Failed to write report: {e}");
        } else {
            println!("📄 Report saved to 'report.json'");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("❌ Error: Not enough arguments.");
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "compare" => {
            if args.len() != 4 {
                eprintln!("❌ Error: compare requires 2 file paths.");
                print_usage();
                process::exit(1);
            }

            let file1 = &args[2];
            let file2 = &args[3];
            run_compare(file1, file2);
        }

        "scan" => {
            if args.len() != 3 {
                eprintln!("❌ Error: scan requires a directory path.");
                print_usage();
                process::exit(1);
            }

            let dir = &args[2];
            run_scan(dir);
        }

        _ => {
            eprintln!("❌ Error: Unknown command '{}'", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}