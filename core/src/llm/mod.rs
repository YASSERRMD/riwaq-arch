//! LLM integration module for documentation generation and Q&A.
//!
//! This module provides:
//! - `LLMClient` trait for abstracting LLM providers
//! - HTTP client implementation for GLM-4.7 and OpenRouter-compatible APIs
//! - Prompt templates for architecture documentation
//! - Context management for large codebases
//! - Enhanced context building with semantic matching
//! - Response caching for performance

pub mod client;
pub mod config;
pub mod prompts;
pub mod context;

pub use client::{HttpLlmClient, LLMClient, LLMResponse};
pub use config::LLMConfig;
pub use context::{ContextBuilder, EnhancedContext, QuestionIntent};

