//! Analysis modules for codebase parsing and inspection.

pub mod analyzer;
pub mod fs_scanner;
pub mod git;
pub mod parser;

// Advanced analysis modules
pub mod api;
pub mod business;

// Re-export main analyzers
pub use api::ApiAnalyzer;
pub use business::BusinessLogicAnalyzer;

