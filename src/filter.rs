use regex::Regex;
use std::{
    ffi::OsStr,
    fs,
    path::Path,
};

/// Structure to hold filtering options
pub struct FilterOptions {
    pub min_size: Option<u64>,      // in bytes
    pub max_size: Option<u64>,
    pub extensions: Option<Vec<String>>,
    pub regex: Option<Regex>,
}

impl FilterOptions {
    pub fn matches(&self, path: &Path) -> bool {
        self.check_size(path)
            && self.check_extension(path)
            && self.check_regex(path)
    }

    fn check_size(&self, path: &Path) -> bool {
        if let Ok(metadata) = fs::metadata(path) {
            if let Some(min) = self.min_size {
                if metadata.len() < min {
                    return false;
                }
            }
            if let Some(max) = self.max_size {
                if metadata.len() > max {
                    return false;
                }
            }
        }
        true
    }

    fn check_extension(&self, path: &Path) -> bool {
        if let Some(ref exts) = self.extensions {
            if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                return exts.iter().any(|e| e.eq_ignore_ascii_case(ext));
            } else {
                return false; // No extension
            }
        }
        true
    }

    fn check_regex(&self, path: &Path) -> bool {
        if let Some(ref re) = self.regex {
            if let Some(name) = path.file_name().and_then(OsStr::to_str) {
                return re.is_match(name);
            } else {
                return false; // No file name
            }
        }
        true
    }
}

/// Helper to parse filtering options from CLI args
pub fn parse_filter_options(args: &[String]) -> FilterOptions {
    let mut options = FilterOptions::default();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--min" => {
                i += 1;
                options.min_size = args.get(i).and_then(|v| v.parse::<u64>().ok());
            }
            "--max" => {
                i += 1;
                options.max_size = args.get(i).and_then(|v| v.parse::<u64>().ok());
            }
            "--ext" => {
                i += 1;
                options.extensions = args.get(i).map(|v| {
                    v.split(',').map(|s| s.trim().to_string()).collect()
                });
            }
            "--regex" => {
                i += 1;
                options.regex = args.get(i).and_then(|v| Regex::new(v).ok());
            }
            _ => {}
        }
        i += 1;
    }

    options
}

/// Default implementation: match all files
impl Default for FilterOptions {
    fn default() -> Self {
        FilterOptions {
            min_size: None,
            max_size: None,
            extensions: None,
            regex: None,
        }
    }
}