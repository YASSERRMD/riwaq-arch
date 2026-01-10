//! # Riwaq Arch Core
//!
//! A powerful codebase analysis and documentation generation engine.
//!
//! This library provides:
//! - Multi-language code parsing using tree-sitter
//! - Dependency graph analysis
//! - Git history insights
//! - LLM-powered documentation generation
//! - Semantic search capabilities
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use riwaq_arch_core::{analyze_codebase, CodebaseSnapshot};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let snapshot = analyze_codebase("./my-project").await?;
//!     println!("Found {} files", snapshot.files.len());
//!     Ok(())
//! }
//! ```

pub mod analysis;
pub mod errors;
pub mod logging;
pub mod models;

// Re-export commonly used types
pub use analysis::analyzer::CodebaseAnalyzer;
pub use errors::{RiwaqError, Result};
pub use models::snapshot::CodebaseSnapshot;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration values
pub mod defaults {
    /// Default maximum file size to analyze (10MB)
    pub const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

    /// Default maximum number of commits to analyze
    pub const MAX_COMMITS: usize = 1000;

    /// Default directories to exclude from analysis
    pub const EXCLUDED_DIRS: &[&str] = &[
        "target",
        "node_modules",
        ".git",
        "dist",
        "build",
        ".next",
        "__pycache__",
        ".pytest_cache",
        "vendor",
        ".cargo",
        "coverage",
        ".nyc_output",
    ];

    /// Default file extensions to include
    pub const INCLUDED_EXTENSIONS: &[&str] = &[
        "rs", "py", "js", "ts", "tsx", "jsx", "go", "java", "kt", "rb", "php", "c", "cpp", "h",
        "hpp", "cs", "swift", "m", "scala", "clj", "ex", "exs", "erl", "hs", "ml", "fs",
    ];
}
