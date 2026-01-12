//! REST API Analyzer
//!
//! Detects REST endpoints with:
//! - API versioning (/v1/, /v2/, etc.)
//! - Authentication patterns (Bearer, API Key, OAuth2)
//! - Rate limiting configuration
//! - Request/response schemas

use super::*;
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use tokio::fs;
use walkdir::WalkDir;

pub struct RestAnalyzer {
    /// Patterns for detecting route definitions
    route_patterns: Vec<RoutePattern>,
    /// Patterns for detecting auth
    auth_patterns: Vec<AuthPattern>,
}

struct RoutePattern {
    name: &'static str,
    regex: Regex,
    method_group: usize,
    path_group: usize,
}

struct AuthPattern {
    name: &'static str,
    regex: Regex,
    auth_type: AuthMethod,
}

impl RestAnalyzer {
    pub fn new() -> Self {
        Self {
            route_patterns: Self::build_route_patterns(),
            auth_patterns: Self::build_auth_patterns(),
        }
    }

    fn build_route_patterns() -> Vec<RoutePattern> {
        vec![
            // Rust Axum/Actix patterns
            RoutePattern {
                name: "axum",
                regex: Regex::new(r#"\.route\(\s*"([^"]+)"\s*,\s*(get|post|put|delete|patch)\s*\("#).unwrap(),
                method_group: 2,
                path_group: 1,
            },
            RoutePattern {
                name: "axum_method",
                regex: Regex::new(r#"\.(get|post|put|delete|patch)\(\s*"([^"]+)""#).unwrap(),
                method_group: 1,
                path_group: 2,
            },
            // Python Flask/FastAPI patterns
            RoutePattern {
                name: "flask",
                regex: Regex::new(r#"@app\.(get|post|put|delete|patch)\(\s*["']([^"']+)["']"#).unwrap(),
                method_group: 1,
                path_group: 2,
            },
            RoutePattern {
                name: "fastapi",
                regex: Regex::new(r#"@router\.(get|post|put|delete|patch)\(\s*["']([^"']+)["']"#).unwrap(),
                method_group: 1,
                path_group: 2,
            },
            // Node.js Express patterns
            RoutePattern {
                name: "express",
                regex: Regex::new(r#"(?:app|router)\.(get|post|put|delete|patch)\(\s*["']([^"']+)["']"#).unwrap(),
                method_group: 1,
                path_group: 2,
            },
            // Go Gin/Echo patterns
            RoutePattern {
                name: "gin",
                regex: Regex::new(r#"(?:r|router|group)\.(GET|POST|PUT|DELETE|PATCH)\(\s*"([^"]+)""#).unwrap(),
                method_group: 1,
                path_group: 2,
            },
        ]
    }

    fn build_auth_patterns() -> Vec<AuthPattern> {
        vec![
            AuthPattern {
                name: "bearer",
                regex: Regex::new(r#"(?i)(bearer|authorization.*bearer|jwt)"#).unwrap(),
                auth_type: AuthMethod::Bearer,
            },
            AuthPattern {
                name: "api_key",
                regex: Regex::new(r#"(?i)(x-api-key|api[_-]?key)"#).unwrap(),
                auth_type: AuthMethod::ApiKey { header: "X-API-Key".to_string() },
            },
            AuthPattern {
                name: "oauth",
                regex: Regex::new(r#"(?i)(oauth2?|access_token)"#).unwrap(),
                auth_type: AuthMethod::OAuth2 { flows: vec!["authorization_code".to_string()] },
            },
            AuthPattern {
                name: "basic",
                regex: Regex::new(r#"(?i)(basic\s+auth|authorization.*basic)"#).unwrap(),
                auth_type: AuthMethod::Basic,
            },
        ]
    }

    pub async fn analyze(&self, root_path: &Path) -> anyhow::Result<RestApiSummary> {
        let mut endpoints = Vec::new();
        let mut versions = HashSet::new();
        let mut auth_methods = HashSet::new();
        let mut base_paths = HashSet::new();
        let mut rate_limiting = None;

        // Walk through source files
        for entry in WalkDir::new(root_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Skip non-source files and common ignored directories
            if !Self::is_source_file(path) || Self::should_skip_dir(path) {
                continue;
            }

            if let Ok(content) = fs::read_to_string(path).await {
                let found_endpoints = self.extract_endpoints(&content, path, root_path);
                
                for endpoint in &found_endpoints {
                    // Extract version from path
                    if let Some(version) = Self::extract_version(&endpoint.path) {
                        versions.insert(version);
                    }
                    
                    // Extract base path
                    if let Some(base) = Self::extract_base_path(&endpoint.path) {
                        base_paths.insert(base);
                    }
                }
                
                endpoints.extend(found_endpoints);
                
                // Detect auth patterns
                for auth_pattern in &self.auth_patterns {
                    if auth_pattern.regex.is_match(&content) {
                        auth_methods.insert(auth_pattern.name);
                    }
                }
                
                // Detect rate limiting
                if rate_limiting.is_none() {
                    rate_limiting = Self::detect_rate_limiting(&content);
                }
            }
        }

        // Convert auth method names to AuthMethod enum
        let auth_methods: Vec<AuthMethod> = auth_methods.into_iter()
            .filter_map(|name| {
                self.auth_patterns.iter()
                    .find(|p| p.name == name)
                    .map(|p| p.auth_type.clone())
            })
            .collect();

        Ok(RestApiSummary {
            endpoints,
            versions: versions.into_iter().collect(),
            auth_methods,
            rate_limiting,
            base_paths: base_paths.into_iter().collect(),
        })
    }

    fn extract_endpoints(&self, content: &str, file_path: &Path, root_path: &Path) -> Vec<RestEndpoint> {
        let mut endpoints = Vec::new();
        let relative_path = file_path.strip_prefix(root_path)
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();

        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.route_patterns {
                if let Some(captures) = pattern.regex.captures(line) {
                    let method = captures.get(pattern.method_group)
                        .map(|m| m.as_str().to_uppercase())
                        .unwrap_or_default();
                    let path = captures.get(pattern.path_group)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default();

                    if !path.is_empty() {
                        // Extract handler name (next word after the path)
                        let handler = Self::extract_handler_name(line, &path);
                        
                        // Check for deprecation markers
                        let deprecated = Self::is_deprecated(content, line_num);
                        
                        // Extract doc comment above
                        let description = Self::extract_doc_comment(content, line_num);
                        
                        // Extract version from path
                        let version = Self::extract_version(&path);
                        
                        // Detect if auth is required for this endpoint
                        let auth_required = Self::detect_auth_required(content, line_num);

                        endpoints.push(RestEndpoint {
                            method,
                            path,
                            version,
                            handler,
                            file: relative_path.clone(),
                            line: line_num + 1,
                            description,
                            parameters: Vec::new(), // TODO: Extract from handler signature
                            request_body: None,
                            responses: Vec::new(),
                            auth_required,
                            deprecated,
                            tags: Vec::new(),
                        });
                    }
                }
            }
        }

        endpoints
    }

    fn extract_handler_name(line: &str, path: &str) -> String {
        // Try to find a function/handler name after the path
        let after_path = line.split(path).nth(1).unwrap_or("");
        
        // Look for common patterns like ", handler_name)" or ", handler_name,"
        let handler_regex = Regex::new(r#"[,\s]+([a-zA-Z_][a-zA-Z0-9_]*)"#).unwrap();
        
        if let Some(cap) = handler_regex.captures(after_path) {
            cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default()
        } else {
            "handler".to_string()
        }
    }

    fn extract_version(path: &str) -> Option<String> {
        let version_regex = Regex::new(r#"/v(\d+)(?:/|$)"#).unwrap();
        version_regex.captures(path)
            .and_then(|c| c.get(1))
            .map(|m| format!("v{}", m.as_str()))
    }

    fn extract_base_path(path: &str) -> Option<String> {
        // Extract the first path segment after version
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        
        if segments.is_empty() {
            return None;
        }

        // Skip version segment if present
        let start_idx = if segments[0].starts_with('v') && segments[0][1..].chars().all(|c| c.is_ascii_digit()) {
            1
        } else {
            0
        };

        segments.get(start_idx).map(|s| format!("/{}", s))
    }

    fn is_deprecated(content: &str, line_num: usize) -> bool {
        let lines: Vec<&str> = content.lines().collect();
        
        // Check previous lines for deprecation markers
        for i in (0..line_num).rev().take(5) {
            if let Some(line) = lines.get(i) {
                if line.contains("deprecated") || line.contains("Deprecated") || 
                   line.contains("#[deprecated") || line.contains("@deprecated") {
                    return true;
                }
            }
        }
        false
    }

    fn extract_doc_comment(content: &str, line_num: usize) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut comments = Vec::new();
        
        // Look for doc comments above the line
        for i in (0..line_num).rev().take(10) {
            if let Some(line) = lines.get(i) {
                let trimmed = line.trim();
                if trimmed.starts_with("///") {
                    comments.push(trimmed.trim_start_matches("///").trim().to_string());
                } else if trimmed.starts_with("//!") {
                    comments.push(trimmed.trim_start_matches("//!").trim().to_string());
                } else if trimmed.starts_with('#') || trimmed.starts_with("\"\"\"") {
                    // Python docstring
                    comments.push(trimmed.trim_matches('"').trim().to_string());
                } else if trimmed.starts_with("/*") || trimmed.starts_with("*") {
                    comments.push(trimmed.trim_start_matches(['/', '*', ' ']).to_string());
                } else if !trimmed.is_empty() && !trimmed.starts_with('@') && !trimmed.starts_with('#') {
                    break;
                }
            }
        }
        
        if comments.is_empty() {
            None
        } else {
            comments.reverse();
            Some(comments.join(" "))
        }
    }

    fn detect_auth_required(content: &str, line_num: usize) -> bool {
        let lines: Vec<&str> = content.lines().collect();
        
        // Check nearby lines for auth middleware/decorators
        let start = line_num.saturating_sub(5);
        let end = (line_num + 5).min(lines.len());
        
        for i in start..end {
            if let Some(line) = lines.get(i) {
                if line.contains("auth") || line.contains("Auth") ||
                   line.contains("protected") || line.contains("Protected") ||
                   line.contains("requires_auth") || line.contains("jwt") ||
                   line.contains("bearer") || line.contains("authenticate") {
                    return true;
                }
            }
        }
        false
    }

    fn detect_rate_limiting(content: &str) -> Option<RateLimitInfo> {
        let rate_regex = Regex::new(r#"(?i)rate[_\s]?limit[^\d]*(\d+)"#).unwrap();
        
        if let Some(cap) = rate_regex.captures(content) {
            let requests = cap.get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(1000);
            
            return Some(RateLimitInfo {
                requests_per_period: requests,
                period_seconds: 3600, // Default to hourly
                header_names: vec![
                    "X-RateLimit-Limit".to_string(),
                    "X-RateLimit-Remaining".to_string(),
                ],
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

impl Default for RestAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Hash for AuthMethod to use in HashSet
impl std::hash::Hash for AuthMethod {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl Eq for AuthMethod {}
impl PartialEq for AuthMethod {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_version() {
        assert_eq!(RestAnalyzer::extract_version("/v1/users"), Some("v1".to_string()));
        assert_eq!(RestAnalyzer::extract_version("/v2/orders/123"), Some("v2".to_string()));
        assert_eq!(RestAnalyzer::extract_version("/api/users"), None);
    }

    #[test]
    fn test_extract_base_path() {
        assert_eq!(RestAnalyzer::extract_base_path("/v1/users/123"), Some("/users".to_string()));
        assert_eq!(RestAnalyzer::extract_base_path("/api/orders"), Some("/api".to_string()));
    }
}
