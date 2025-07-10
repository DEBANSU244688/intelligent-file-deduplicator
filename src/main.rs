mod file_compare;
mod filter;
mod handler;
mod hashing;
mod report;
mod safe_delete;
mod scanner;
mod ui;

use handler::*;
use ui::{print_banner, print_error, print_info};

use std::{env, process};

/// Entry point of the program.
/// Parses command-line arguments and dispatches to the appropriate handler function.
fn main() {
    // Display ASCII banner at launch
    print_banner();

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Require at least one command
    if args.len() < 2 {
        print_error("Error: Not enough arguments.\n");
        print_usage();
        process::exit(1);
    }

    // Match and dispatch commands
    match args[1].as_str() {
        "compare" => handle_compare_command(&args),
        "scan" => handle_scan_command(&args),
        "report" => handle_report_command(&args),
        "delete" => handle_delete_command(&args),
        "filter" => handle_filter_command(&args),
        _ => {
            print_error(&format!("Error: Unknown command '{}'", args[1]));
            print_usage();
            process::exit(1);
        }
    }
}

/// Prints usage instructions for all commands.
fn print_usage() {
    print_info("📘 Usage Guide:\n");
    println!("  hashlaser compare <file1> <file2>        🔍 Compare two files");
    println!("  hashlaser scan <dir> [options]           🧪 Scan directory for duplicates");
    println!("      Options: --min <bytes> --max <bytes> --ext txt,csv --regex <pattern>");
    println!("  hashlaser report <dir> <output.json>     📄 Generate JSON report");
    println!("  hashlaser delete <dir> [--dry-run]       🗑️ Delete duplicate files");
    println!("  hashlaser filter <dir> [options]         🎯 Scan with filtering");
}
