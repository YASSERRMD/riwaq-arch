//! Filesystem scanner for discovering and filtering source files.
//!
//! This module provides functionality to walk a directory tree and
//! identify source code files that should be analyzed.

use ignore::WalkBuilder;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

use crate::errors::{Result, RiwaqError};
use crate::models::file::Language;

/// Configuration for the filesystem scanner.
#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// Root directory to scan.
    pub root: PathBuf,

    /// Directories to exclude.
    pub excluded_dirs: HashSet<String>,

    /// File extensions to include.
    pub included_extensions: HashSet<String>,

    /// Maximum file size in bytes.
    pub max_file_size: u64,

    /// Whether to include hidden files.
    pub include_hidden: bool,

    /// Whether to include test files.
    pub include_tests: bool,

    /// Whether to follow symlinks.
    pub follow_symlinks: bool,

    /// Maximum depth to scan (None = unlimited).
    pub max_depth: Option<usize>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            excluded_dirs: crate::defaults::EXCLUDED_DIRS
                .iter()
                .map(|s| s.to_string())
                .collect(),
            included_extensions: crate::defaults::INCLUDED_EXTENSIONS
                .iter()
                .map(|s| s.to_string())
                .collect(),
            max_file_size: crate::defaults::MAX_FILE_SIZE,
            include_hidden: false,
            include_tests: true,
            follow_symlinks: false,
            max_depth: None,
        }
    }
}

impl ScanConfig {
    /// Create a new scan config for the given root.
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
            ..Default::default()
        }
    }

    /// Add directories to exclude.
    pub fn exclude_dirs(mut self, dirs: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for dir in dirs {
            self.excluded_dirs.insert(dir.into());
        }
        self
    }

    /// Set the file extensions to include.
    pub fn include_extensions(mut self, exts: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.included_extensions = exts.into_iter().map(|e| e.into()).collect();
        self
    }

    /// Set the maximum file size.
    pub fn max_file_size(mut self, size: u64) -> Self {
        self.max_file_size = size;
        self
    }

    /// Set whether to include test files.
    pub fn include_tests(mut self, include: bool) -> Self {
        self.include_tests = include;
        self
    }

    /// Set the maximum scan depth.
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }
}

/// Result of scanning a single file.
#[derive(Debug, Clone)]
pub struct ScannedFile {
    /// Absolute path to the file.
    pub absolute_path: PathBuf,

    /// Relative path from the scan root.
    pub relative_path: PathBuf,

    /// Detected language.
    pub language: Language,

    /// File size in bytes.
    pub size: u64,

    /// Whether this appears to be a test file.
    pub is_test: bool,
}

impl ScannedFile {
    /// Read the file contents.
    pub fn read_contents(&self) -> Result<String> {
        std::fs::read_to_string(&self.absolute_path).map_err(|e| {
            RiwaqError::file_system(&self.absolute_path, e.to_string())
        })
    }
}

/// Filesystem scanner for discovering source files.
pub struct FsScanner {
    config: ScanConfig,
}

impl FsScanner {
    /// Create a new scanner with the given configuration.
    pub fn new(config: ScanConfig) -> Self {
        Self { config }
    }

    /// Scan the filesystem and return all discovered source files.
    pub fn scan(&self) -> Result<Vec<ScannedFile>> {
        let root = self.config.root.canonicalize().map_err(|_e| {
            RiwaqError::DirectoryNotFound(self.config.root.clone())
        })?;

        if !root.is_dir() {
            return Err(RiwaqError::DirectoryNotFound(root));
        }

        info!(root = ?root, "Starting filesystem scan");

        let mut builder = WalkBuilder::new(&root);
        builder
            .hidden(!self.config.include_hidden)
            .follow_links(self.config.follow_symlinks)
            .standard_filters(true); // Respects .gitignore

        if let Some(depth) = self.config.max_depth {
            builder.max_depth(Some(depth));
        }

        let mut files = Vec::new();
        let mut skipped_count = 0;

        for entry in builder.build() {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    warn!(error = %e, "Error walking directory");
                    continue;
                }
            };

            let path = entry.path();

            // Skip directories
            if path.is_dir() {
                // Check if this directory should be excluded
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if self.config.excluded_dirs.contains(name) {
                        debug!(path = ?path, "Skipping excluded directory");
                        continue;
                    }
                }
                continue;
            }

            // Check file extension
            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            if !self.config.included_extensions.contains(extension) {
                skipped_count += 1;
                continue;
            }

            // Check file size
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(e) => {
                    warn!(path = ?path, error = %e, "Could not read file metadata");
                    continue;
                }
            };

            if metadata.len() > self.config.max_file_size {
                warn!(
                    path = ?path,
                    size = metadata.len(),
                    max = self.config.max_file_size,
                    "Skipping file (too large)"
                );
                skipped_count += 1;
                continue;
            }

            // Determine language
            let language = Language::from_extension(extension);

            // Calculate relative path
            let relative_path = path.strip_prefix(&root).unwrap_or(path).to_path_buf();

            // Check if it's a test file
            let is_test = Self::is_test_file(&relative_path);

            if !self.config.include_tests && is_test {
                skipped_count += 1;
                continue;
            }

            files.push(ScannedFile {
                absolute_path: path.to_path_buf(),
                relative_path,
                language,
                size: metadata.len(),
                is_test,
            });
        }

        info!(
            found = files.len(),
            skipped = skipped_count,
            "Filesystem scan complete"
        );

        Ok(files)
    }

    /// Check if a file appears to be a test file.
    fn is_test_file(path: &Path) -> bool {
        let path_str = path.to_string_lossy().to_lowercase();
        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        // Check path patterns
        if path_str.contains("/test/")
            || path_str.contains("/tests/")
            || path_str.contains("/__tests__/")
            || path_str.contains("/__test__/")
            || path_str.contains("/spec/")
            || path_str.contains("/specs/")
        {
            return true;
        }

        // Check file name patterns
        file_name.starts_with("test_")
            || file_name.ends_with("_test")
            || file_name.ends_with("_tests")
            || file_name.ends_with("_spec")
            || file_name.ends_with(".test")
            || file_name.ends_with(".spec")
    }

    /// Get statistics about the scanned files.
    pub fn get_stats(files: &[ScannedFile]) -> ScanStats {
        let mut stats = ScanStats::default();
        stats.total_files = files.len();

        for file in files {
            stats.total_size += file.size;
            if file.is_test {
                stats.test_files += 1;
            }
            *stats.languages.entry(file.language).or_insert(0) += 1;
        }

        stats
    }
}

/// Statistics from a filesystem scan.
#[derive(Debug, Clone, Default)]
pub struct ScanStats {
    /// Total number of files found.
    pub total_files: usize,

    /// Total size of all files in bytes.
    pub total_size: u64,

    /// Number of test files.
    pub test_files: usize,

    /// Count of files per language.
    pub languages: std::collections::HashMap<Language, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    fn create_test_project() -> TempDir {
        let dir = TempDir::new().unwrap();

        // Create directory structure
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::create_dir_all(dir.path().join("tests")).unwrap();
        fs::create_dir_all(dir.path().join("node_modules")).unwrap();

        // Create source files
        fs::write(dir.path().join("src/main.rs"), "fn main() {}").unwrap();
        fs::write(dir.path().join("src/lib.rs"), "pub fn hello() {}").unwrap();
        fs::write(dir.path().join("tests/test_main.rs"), "#[test] fn test() {}").unwrap();

        // Create node_modules file (should be ignored)
        fs::write(dir.path().join("node_modules/package.js"), "module.exports = {}").unwrap();

        dir
    }

    #[test]
    fn test_scan_basic() {
        let dir = create_test_project();
        let config = ScanConfig::new(dir.path());
        let scanner = FsScanner::new(config);

        let files = scanner.scan().unwrap();
        
        // Should find src/ files but not node_modules/
        assert!(files.len() >= 2);
        assert!(files.iter().all(|f| !f.relative_path.to_string_lossy().contains("node_modules")));
    }

    #[test]
    fn test_is_test_file() {
        assert!(FsScanner::is_test_file(Path::new("tests/test_main.rs")));
        assert!(FsScanner::is_test_file(Path::new("src/__tests__/app.ts")));
        assert!(FsScanner::is_test_file(Path::new("app.test.js")));
        assert!(!FsScanner::is_test_file(Path::new("src/main.rs")));
    }

    #[test]
    fn test_exclude_tests() {
        let dir = create_test_project();
        let config = ScanConfig::new(dir.path()).include_tests(false);
        let scanner = FsScanner::new(config);

        let files = scanner.scan().unwrap();
        assert!(files.iter().all(|f| !f.is_test));
    }

    #[test]
    fn test_scan_stats() {
        let files = vec![
            ScannedFile {
                absolute_path: PathBuf::from("/test/main.rs"),
                relative_path: PathBuf::from("main.rs"),
                language: Language::Rust,
                size: 100,
                is_test: false,
            },
            ScannedFile {
                absolute_path: PathBuf::from("/test/test_main.rs"),
                relative_path: PathBuf::from("test_main.rs"),
                language: Language::Rust,
                size: 50,
                is_test: true,
            },
        ];

        let stats = FsScanner::get_stats(&files);
        assert_eq!(stats.total_files, 2);
        assert_eq!(stats.total_size, 150);
        assert_eq!(stats.test_files, 1);
    }
}
