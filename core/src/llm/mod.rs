//! LLM integration module for documentation generation and Q&A.
//!
//! This module provides:
//! - `LLMClient` trait for abstracting LLM providers
//! - HTTP client implementation for GLM-4.7 and OpenRouter-compatible APIs
//! - Prompt templates for architecture documentation
//! - Context management for large codebases

pub mod client;
pub mod config;
pub mod prompts;

pub use client::{HttpLlmClient, LLMClient, LLMResponse};
pub use config::LLMConfig;
