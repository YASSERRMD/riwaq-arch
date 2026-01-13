//! Documentation generation module.
//!
//! This module provides functionality to generate various types of documentation
//! from a codebase analysis snapshot:
//!
//! - **Architecture Overview**: High-level system architecture documentation
//! - **Module Documentation**: Per-module/component documentation
//! - **API Reference**: HTTP/gRPC/GraphQL endpoint documentation
//! - **Business Model**: Entity diagrams, workflows, business rules
//! - **ADRs**: Architecture Decision Records
//! - **Dependency Report**: Dependency analysis and risk assessment

pub mod generator;
pub mod markdown;
pub mod mermaid;
pub mod api_reference;
pub mod business_model;
pub mod agentic_generator;

pub use generator::{DocGenerator, DocGeneratorConfig, GeneratedDocs};
pub use api_reference::ApiReferenceGenerator;
pub use business_model::BusinessModelGenerator;
pub use agentic_generator::{AgenticDocGenerator, DocumentType};


