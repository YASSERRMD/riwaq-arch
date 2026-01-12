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
//! - API detection (REST, gRPC, GraphQL, Webhooks)
//! - Business logic extraction
//! - Performance caching
//! - Security validation
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
pub mod docs;
pub mod errors;
pub mod llm;
pub mod logging;
pub mod models;
pub mod server;
pub mod cache;
pub mod security;
pub mod metrics;
pub mod diagrams;

// Re-export commonly used types
pub use analysis::analyzer::CodebaseAnalyzer;
pub use analysis::{ApiAnalyzer, BusinessLogicAnalyzer};
pub use docs::{DocGenerator, DocGeneratorConfig, GeneratedDocs};
pub use errors::{RiwaqError, Result};
pub use llm::{HttpLlmClient, LLMClient, LLMConfig, LLMResponse, ContextBuilder};
pub use models::snapshot::CodebaseSnapshot;
pub use server::{create_router, AppState};
pub use cache::CacheManager;
pub use security::{PathValidator, SecretDetector, RateLimiter};
pub use metrics::{MetricsCollector, HealthChecker, HealthStatus};
pub use diagrams::{DiagramRenderer, DiagramFormat, RenderedDiagram};

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
