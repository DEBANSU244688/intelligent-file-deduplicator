mod hashing;
mod file_compare;
mod scanner;
mod report;
mod safe_delete;
mod handler; 

use std::{env, process};
use handler::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("❌ Error: Not enough arguments.");
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "compare" => handle_compare(&args),
        "scan"    => handle_scan(&args),
        "report"  => handle_report(&args),
        "delete"  => handle_delete(&args),
        _         => {
            eprintln!("❌ Error: Unknown command '{}'", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}