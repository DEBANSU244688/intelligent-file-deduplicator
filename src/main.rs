mod hashing;
mod file_compare;

use file_compare::compare_files;
use std::{env, process};

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file1> <file2>", args[0]);
        process::exit(1);
    }

    let file1 = &args[1];
    let file2 = &args[2];

    match compare_files(file1, file2) {
        Ok(true) => println!("✅ Files are identical."),
        Ok(false) => println!("❌ Files are different."),
        Err(e) => eprintln!("Error: {e}"),
    }
}