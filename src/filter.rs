use regex::Regex;
use std::{ffi::OsStr, fs, path::Path};

/// Structure to hold filtering options
pub struct FilterOptions {
    pub min_size: Option<u64>, // in bytes
    pub max_size: Option<u64>,
    pub extensions: Option<Vec<String>>,
    pub regex: Option<Regex>,
}

/// Implements filtering logic for file paths based on size, extension, and regex criteria.
impl FilterOptions {
    /// Checks if the given `path` matches all filter options: size, extension, and regex.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path to check against the filter options.
    ///
    /// # Returns
    ///
    /// * `true` if the path matches all filter criteria, `false` otherwise.
    pub fn matches(&self, path: &Path) -> bool {
        self.check_size(path) && self.check_extension(path) && self.check_regex(path)
    }

    /// Checks if the file at the given `path` matches the size constraints.
    ///
    /// Returns `true` if the file size is within the specified minimum and maximum size (if set),
    /// or if no size constraints are specified.
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

    /// Checks if the file at the given `path` matches the allowed extensions.
    ///
    /// Returns `true` if the file extension is in the allowed list (if set),
    /// or if no extension constraints are specified.
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

    /// Checks if the file name at the given `path` matches the specified regex pattern.
    ///
    /// Returns `true` if the file name matches the regex (if set),
    /// or if no regex constraint is specified.
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
                options.extensions = args
                    .get(i)
                    .map(|v| v.split(',').map(|s| s.trim().to_string()).collect());
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
