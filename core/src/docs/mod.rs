//! Documentation generation module.
//!
//! This module provides functionality to generate various types of documentation
//! from a codebase analysis snapshot:
//!
//! - **Architecture Overview**: High-level system architecture documentation
//! - **Module Documentation**: Per-module/component documentation
//! - **API Reference**: HTTP/gRPC/GraphQL endpoint documentation
//! - **ADRs**: Architecture Decision Records
//! - **Dependency Report**: Dependency analysis and risk assessment

pub mod generator;
pub mod markdown;
pub mod mermaid;

pub use generator::{DocGenerator, DocGeneratorConfig, GeneratedDocs};
