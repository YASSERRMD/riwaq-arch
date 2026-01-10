//! LLM configuration.

use serde::{Deserialize, Serialize};

/// Configuration for the LLM client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    /// API endpoint URL.
    /// Supports OpenRouter, Z.ai, or any OpenAI-compatible endpoint.
    #[serde(default = "default_endpoint")]
    pub endpoint: String,

    /// API key (loaded from environment if not set).
    #[serde(skip_serializing)]
    pub api_key: Option<String>,

    /// Model identifier.
    #[serde(default = "default_model")]
    pub model: String,

    /// Maximum tokens to generate.
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,

    /// Temperature for generation (0.0 = deterministic, 1.0 = creative).
    #[serde(default = "default_temperature")]
    pub temperature: f32,

    /// Enable thinking mode (extended reasoning for complex tasks).
    #[serde(default)]
    pub thinking_mode: bool,

    /// Request timeout in seconds.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,

    /// Maximum retries for failed requests.
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,

    /// Context window size (tokens).
    #[serde(default = "default_context_window")]
    pub context_window: usize,
}

fn default_endpoint() -> String {
    std::env::var("RIWAQ_LLM_ENDPOINT")
        .unwrap_or_else(|_| "https://api.openrouter.ai/api/v1".to_string())
}

fn default_model() -> String {
    std::env::var("RIWAQ_LLM_MODEL").unwrap_or_else(|_| "glm-4".to_string())
}

fn default_max_tokens() -> usize {
    4096
}

fn default_temperature() -> f32 {
    0.3
}

fn default_timeout() -> u64 {
    120
}

fn default_max_retries() -> u32 {
    3
}

fn default_context_window() -> usize {
    200000 // GLM-4.7 supports 200K context
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            endpoint: default_endpoint(),
            api_key: std::env::var("RIWAQ_LLM_API_KEY").ok(),
            model: default_model(),
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
            thinking_mode: false,
            timeout_secs: default_timeout(),
            max_retries: default_max_retries(),
            context_window: default_context_window(),
        }
    }
}

impl LLMConfig {
    /// Create a new LLM config with the given endpoint.
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            ..Default::default()
        }
    }

    /// Set the API key.
    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Set the model.
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Set max tokens.
    pub fn with_max_tokens(mut self, tokens: usize) -> Self {
        self.max_tokens = tokens;
        self
    }

    /// Set temperature.
    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = temp;
        self
    }

    /// Enable thinking mode.
    pub fn with_thinking_mode(mut self, enabled: bool) -> Self {
        self.thinking_mode = enabled;
        self
    }

    /// Validate the configuration.
    pub fn validate(&self) -> crate::errors::Result<()> {
        if self.api_key.is_none() {
            return Err(crate::errors::RiwaqError::MissingEnvVar(
                "RIWAQ_LLM_API_KEY".to_string(),
            ));
        }

        if self.endpoint.is_empty() {
            return Err(crate::errors::RiwaqError::config("Endpoint cannot be empty"));
        }

        if self.max_tokens == 0 {
            return Err(crate::errors::RiwaqError::config("max_tokens must be > 0"));
        }

        Ok(())
    }

    /// Estimate tokens for a given text (rough approximation).
    /// Uses ~4 characters per token as a heuristic.
    pub fn estimate_tokens(text: &str) -> usize {
        text.len() / 4
    }

    /// Check if content fits within context window.
    pub fn fits_in_context(&self, text: &str) -> bool {
        Self::estimate_tokens(text) < self.context_window
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LLMConfig::default();
        assert_eq!(config.max_tokens, 4096);
        assert!(config.temperature >= 0.0 && config.temperature <= 1.0);
    }

    #[test]
    fn test_builder_pattern() {
        let config = LLMConfig::new("https://api.example.com")
            .with_model("gpt-4")
            .with_max_tokens(8192)
            .with_temperature(0.7);

        assert_eq!(config.endpoint, "https://api.example.com");
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.max_tokens, 8192);
        assert_eq!(config.temperature, 0.7);
    }

    #[test]
    fn test_estimate_tokens() {
        let text = "Hello, world!"; // 13 chars
        let tokens = LLMConfig::estimate_tokens(text);
        assert_eq!(tokens, 3); // 13 / 4 = 3
    }
}
