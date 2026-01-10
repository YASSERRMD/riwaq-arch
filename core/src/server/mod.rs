//! HTTP server module for the VS Code extension.
//!
//! Provides REST API endpoints for:
//! - Codebase analysis
//! - Documentation generation
//! - Q&A functionality
//! - Architecture diagrams

pub mod handlers;
pub mod routes;
pub mod state;

pub use routes::create_router;
pub use state::AppState;
