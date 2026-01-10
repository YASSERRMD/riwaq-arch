//! Data models for Riwaq Arch.
//!
//! This module contains all the core data structures used throughout
//! the codebase analysis and documentation generation process.

pub mod file;
pub mod function;
pub mod git;
pub mod module;
pub mod snapshot;

// Re-export common types
pub use file::FileSummary;
pub use function::FunctionSummary;
pub use git::GitInsights;
pub use module::ModuleSummary;
pub use snapshot::CodebaseSnapshot;
