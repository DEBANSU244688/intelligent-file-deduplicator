//! # Intelligent File Deduplicator Library
//!
//! This library provides core modules for detecting, comparing, filtering, and safely deleting duplicate files.
//! It is designed to be modular and extensible, allowing for efficient file deduplication workflows.
//!
//! ## Modules
//!
//! - [`hashing`]: Utilities for hashing files to identify duplicates efficiently.
//! - [`file_compare`]: Functions for comparing files at a binary or content level.
//! - [`scanner`]: Tools for scanning directories and collecting file metadata.
//! - [`report`]: Facilities for generating reports on duplicates and actions taken.
//! - [`filter`]: Mechanisms for filtering files based on user-defined criteria.
//! - [`safe_delete`]: Safe deletion utilities to remove duplicates without data loss.
//!
//! Each module is documented individually with further details and usage examples.
pub mod file_compare;
pub mod filter;
pub mod hashing;
pub mod report;
pub mod safe_delete;
pub mod scanner;
