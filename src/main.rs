mod hashing;
mod file_compare;
mod scanner;
mod report;
mod filter;
mod safe_delete; 
mod handler;

use handler::*;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("❌ Error: Not enough arguments.");
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "compare" => handle_compare_command(&args),
        "scan" => handle_scan_command(&args),
        "report" => handle_report_command(&args),
        "delete" => handle_delete_command(&args),
        "filter" => handle_filter_command(&args),
        _ => {
            eprintln!("❌ Error: Unknown command '{}'", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  dedup compare <file1> <file2>          Compare two files");
    println!("  dedup scan <dir> [--min <bytes>] [--max <bytes>] [--ext txt,csv] [--regex pattern]");
    println!("  dedup report <dir> <output.json>       Generate JSON report of duplicates");
    println!("  dedup delete <dir>                     Delete duplicate files");
    println!("  dedup filter <dir>                     Scan with advanced filtering");
}