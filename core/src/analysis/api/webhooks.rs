//! Webhook Endpoint Analyzer
//!
//! Detects webhook endpoints and event handling:
//! - Webhook receivers/handlers
//! - Event types (order.created, payment.completed, etc.)
//! - Retry policies
//! - Payload structures

use super::*;
use regex::Regex;
use std::path::Path;
use tokio::fs;
use walkdir::WalkDir;

pub struct WebhookAnalyzer;

impl WebhookAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub async fn analyze(&self, root_path: &Path) -> anyhow::Result<Vec<WebhookDefinition>> {
        let mut webhooks = Vec::new();

        // Walk through source files
        for entry in WalkDir::new(root_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if !Self::is_source_file(path) || Self::should_skip_dir(path) {
                continue;
            }

            if let Ok(content) = fs::read_to_string(path).await {
                let relative_path = path.strip_prefix(root_path)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string();
                
                let found = self.detect_webhooks(&content, &relative_path);
                webhooks.extend(found);
            }
        }

        Ok(webhooks)
    }

    fn detect_webhooks(&self, content: &str, file: &str) -> Vec<WebhookDefinition> {
        let mut webhooks = Vec::new();

        // Patterns for webhook detection
        let patterns = vec![
            // Common webhook patterns
            (r#"webhook[_\s]*(endpoint|handler|receiver)"#, "webhook_handler"),
            (r#"(on|handle)[_\s]*(event|webhook|hook)"#, "event_handler"),
            (r#"POST.*/webhook"#, "webhook_endpoint"),
            // Event type patterns
            (r#"event[_\s]*type\s*[:=]\s*["']([^"']+)["']"#, "event_type"),
            (r#"["']([\w]+\.(created|updated|deleted|completed|failed))["']"#, "event_name"),
            // Stripe-style webhooks
            (r#"(?i)stripe.*webhook"#, "stripe_webhook"),
            // GitHub webhooks
            (r#"(?i)x-github-event"#, "github_webhook"),
            // PayPal webhooks
            (r#"(?i)paypal.*webhook"#, "paypal_webhook"),
        ];

        for (line_num, line) in content.lines().enumerate() {
            for (pattern, _kind) in &patterns {
                let regex = Regex::new(pattern).unwrap();
                
                if regex.is_match(line) {
                    // Try to extract event type
                    let event_type = Self::extract_event_type(line, content, line_num);
                    
                    // Try to extract endpoint path
                    let endpoint = Self::extract_endpoint(line);
                    
                    // Check for retry policy
                    let retry_policy = Self::detect_retry_policy(content, line_num);

                    webhooks.push(WebhookDefinition {
                        event_type: event_type.unwrap_or_else(|| "webhook".to_string()),
                        endpoint,
                        payload_schema: None,
                        retry_policy,
                        file: file.to_string(),
                        line: line_num + 1,
                    });
                    
                    break; // Only capture once per line
                }
            }
        }

        // Deduplicate by event_type + file
        webhooks.sort_by(|a, b| {
            (&a.event_type, &a.file).cmp(&(&b.event_type, &b.file))
        });
        webhooks.dedup_by(|a, b| a.event_type == b.event_type && a.file == b.file);

        webhooks
    }

    fn extract_event_type(line: &str, content: &str, line_num: usize) -> Option<String> {
        // Try common event type patterns
        let patterns = vec![
            r#"["'](\w+\.\w+)["']"#, // "order.created"
            r#"event[_\s]*type\s*[:=]\s*["']([^"']+)["']"#, // event_type: "..."
            r#"["']([A-Z_]+)["']"#, // "ORDER_CREATED"
        ];

        for pattern in &patterns {
            let regex = Regex::new(pattern).ok()?;
            
            // Check current line
            if let Some(cap) = regex.captures(line) {
                return cap.get(1).map(|m| m.as_str().to_string());
            }
        }

        // Check surrounding lines
        let lines: Vec<&str> = content.lines().collect();
        let start = line_num.saturating_sub(3);
        let end = (line_num + 3).min(lines.len());

        for i in start..end {
            if let Some(l) = lines.get(i) {
                for pattern in &patterns {
                    if let Ok(regex) = Regex::new(pattern) {
                        if let Some(cap) = regex.captures(l) {
                            return cap.get(1).map(|m| m.as_str().to_string());
                        }
                    }
                }
            }
        }

        None
    }

    fn extract_endpoint(line: &str) -> Option<String> {
        // Try to extract URL path
        let url_regex = Regex::new(r#"["'](/[^"']+/webhook[^"']*)["']"#).ok()?;
        
        url_regex.captures(line)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn detect_retry_policy(content: &str, line_num: usize) -> Option<RetryPolicy> {
        let lines: Vec<&str> = content.lines().collect();
        let start = line_num.saturating_sub(20);
        let end = (line_num + 20).min(lines.len());
        
        let search_content: String = lines[start..end].join("\n");

        // Look for retry configuration
        let retry_regex = Regex::new(r#"(?i)max[_\s]?retr(y|ies)\s*[:=]\s*(\d+)"#).ok()?;
        let backoff_regex = Regex::new(r#"(?i)backoff\s*[:=]\s*(\d+)"#).ok()?;

        let max_retries = retry_regex.captures(&search_content)
            .and_then(|c| c.get(2))
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(3);

        let backoff_ms = backoff_regex.captures(&search_content)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(1000);

        // Only return if we found retry-related config
        if retry_regex.is_match(&search_content) || backoff_regex.is_match(&search_content) {
            return Some(RetryPolicy {
                max_retries,
                backoff_ms,
            });
        }

        None
    }

    fn is_source_file(path: &Path) -> bool {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        matches!(ext, "rs" | "py" | "js" | "ts" | "go" | "java" | "rb" | "php")
    }

    fn should_skip_dir(path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        path_str.contains("node_modules") ||
        path_str.contains("target") ||
        path_str.contains(".git") ||
        path_str.contains("vendor") ||
        path_str.contains("__pycache__")
    }
}

impl Default for WebhookAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_event_type() {
        let line = r#"handle_webhook("order.created", payload)"#;
        let event = WebhookAnalyzer::extract_event_type(line, line, 0);
        assert_eq!(event, Some("order.created".to_string()));
    }

    #[test]
    fn test_extract_endpoint() {
        let line = r#"app.post("/api/v1/webhooks/stripe", handler)"#;
        let endpoint = WebhookAnalyzer::extract_endpoint(line);
        assert_eq!(endpoint, Some("/api/v1/webhooks/stripe".to_string()));
    }
}
