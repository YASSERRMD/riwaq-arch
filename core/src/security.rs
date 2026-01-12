//! Security module for input validation and safety checks
//!
//! Provides:
//! - Path traversal prevention
//! - Secret detection
//! - Input sanitization
//! - Rate limiting helpers

use std::path::{Path, PathBuf};
use std::collections::HashSet;
use regex::Regex;

/// Security validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub issues: Vec<SecurityIssue>,
}

/// Security issue detected
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub message: String,
    pub location: Option<String>,
}

/// Issue severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Issue category
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IssueCategory {
    PathTraversal,
    SecretExposure,
    UnsafeInput,
    InvalidFormat,
}

/// Path validator for preventing directory traversal attacks
pub struct PathValidator {
    allowed_roots: Vec<PathBuf>,
    denied_patterns: Vec<String>,
}

impl PathValidator {
    pub fn new(allowed_roots: Vec<PathBuf>) -> Self {
        Self {
            allowed_roots,
            denied_patterns: vec![
                "..".to_string(),
                "~".to_string(),
            ],
        }
    }

    /// Validate a path is safe to access
    pub fn validate(&self, path: &Path) -> ValidationResult {
        let mut issues = Vec::new();

        // Check for path traversal patterns
        let path_str = path.to_string_lossy();
        
        for pattern in &self.denied_patterns {
            if path_str.contains(pattern) {
                issues.push(SecurityIssue {
                    severity: IssueSeverity::Critical,
                    category: IssueCategory::PathTraversal,
                    message: format!("Path contains forbidden pattern: {}", pattern),
                    location: Some(path_str.to_string()),
                });
            }
        }

        // Check if path is within allowed roots
        if let Ok(canonical) = path.canonicalize() {
            let is_allowed = self.allowed_roots.iter().any(|root| {
                root.canonicalize()
                    .map(|r| canonical.starts_with(&r))
                    .unwrap_or(false)
            });

            if !is_allowed && !self.allowed_roots.is_empty() {
                issues.push(SecurityIssue {
                    severity: IssueSeverity::Critical,
                    category: IssueCategory::PathTraversal,
                    message: "Path is outside allowed directories".to_string(),
                    location: Some(path_str.to_string()),
                });
            }
        }

        // Check for symbolic link attacks
        if path.is_symlink() {
            issues.push(SecurityIssue {
                severity: IssueSeverity::Medium,
                category: IssueCategory::PathTraversal,
                message: "Symbolic links may be security risks".to_string(),
                location: Some(path_str.to_string()),
            });
        }

        ValidationResult {
            valid: issues.is_empty() || issues.iter().all(|i| i.severity != IssueSeverity::Critical),
            issues,
        }
    }

    /// Sanitize a path by removing traversal attempts
    pub fn sanitize(&self, path: &str) -> String {
        let mut sanitized = path.to_string();
        
        // Remove .. components
        while sanitized.contains("..") {
            sanitized = sanitized.replace("..", "");
        }
        
        // Remove double slashes
        while sanitized.contains("//") {
            sanitized = sanitized.replace("//", "/");
        }
        
        // Remove leading/trailing slashes for safety
        sanitized.trim_matches('/').to_string()
    }
}

impl Default for PathValidator {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

/// Secret detector for finding exposed credentials
pub struct SecretDetector {
    patterns: Vec<SecretPattern>,
}

struct SecretPattern {
    name: &'static str,
    regex: Regex,
    severity: IssueSeverity,
}

impl SecretDetector {
    pub fn new() -> Self {
        let patterns = vec![
            SecretPattern {
                name: "AWS Access Key",
                regex: Regex::new(r#"(?i)(aws_)?access_key(_id)?['":\s]*[=:]\s*['"]?(AKIA[0-9A-Z]{16})['"]?"#).unwrap(),
                severity: IssueSeverity::Critical,
            },
            SecretPattern {
                name: "AWS Secret Key",
                regex: Regex::new(r#"(?i)(aws_)?secret_?(access_)?key['":\s]*[=:]\s*['"]?([A-Za-z0-9/+=]{40})['"]?"#).unwrap(),
                severity: IssueSeverity::Critical,
            },
            SecretPattern {
                name: "API Key",
                regex: Regex::new(r#"(?i)(api[_-]?key|apikey)['":\s]*[=:]\s*['"]?([a-zA-Z0-9_\-]{20,})['"]?"#).unwrap(),
                severity: IssueSeverity::High,
            },
            SecretPattern {
                name: "Private Key",
                regex: Regex::new(r#"-----BEGIN (?:RSA |EC |DSA )?PRIVATE KEY-----"#).unwrap(),
                severity: IssueSeverity::Critical,
            },
            SecretPattern {
                name: "Password",
                regex: Regex::new(r#"(?i)(password|passwd|pwd)['":\s]*[=:]\s*['"]?([^'"\s]{8,})['"]?"#).unwrap(),
                severity: IssueSeverity::High,
            },
            SecretPattern {
                name: "JWT Secret",
                regex: Regex::new(r#"(?i)jwt[_-]?secret['":\s]*[=:]\s*['"]?([^'"\s]{16,})['"]?"#).unwrap(),
                severity: IssueSeverity::Critical,
            },
            SecretPattern {
                name: "Database URL",
                regex: Regex::new(r#"(?i)(postgres|mysql|mongodb)://[^:]+:[^@]+@"#).unwrap(),
                severity: IssueSeverity::High,
            },
            SecretPattern {
                name: "Stripe Key",
                regex: Regex::new(r#"sk_live_[0-9a-zA-Z]{24}"#).unwrap(),
                severity: IssueSeverity::Critical,
            },
            SecretPattern {
                name: "GitHub Token",
                regex: Regex::new(r#"gh[pousr]_[0-9a-zA-Z]{36}"#).unwrap(),
                severity: IssueSeverity::Critical,
            },
            SecretPattern {
                name: "Slack Token",
                regex: Regex::new(r#"xox[baprs]-[0-9]{10,12}-[0-9]{10,12}-[a-zA-Z0-9]{24}"#).unwrap(),
                severity: IssueSeverity::High,
            },
        ];

        Self { patterns }
    }

    /// Scan content for secrets
    pub fn scan(&self, content: &str, file_path: Option<&str>) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();

        for pattern in &self.patterns {
            for mat in pattern.regex.find_iter(content) {
                // Get line number
                let line_num = content[..mat.start()].lines().count();
                
                issues.push(SecurityIssue {
                    severity: pattern.severity,
                    category: IssueCategory::SecretExposure,
                    message: format!("Potential {} found at line {}", pattern.name, line_num),
                    location: file_path.map(|p| format!("{}:{}", p, line_num)),
                });
            }
        }

        issues
    }

    /// Check if content contains any secrets
    pub fn has_secrets(&self, content: &str) -> bool {
        self.patterns.iter().any(|p| p.regex.is_match(content))
    }
}

impl Default for SecretDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Input sanitizer for user-provided text
pub struct InputSanitizer;

impl InputSanitizer {
    /// Sanitize HTML/script content
    pub fn sanitize_html(input: &str) -> String {
        input
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }

    /// Sanitize for markdown
    pub fn sanitize_markdown(input: &str) -> String {
        // Escape markdown special characters that could be exploited
        let specials = ['[', ']', '(', ')', '#', '*', '_', '`', '\\'];
        let mut result = input.to_string();
        
        for c in specials {
            result = result.replace(c, &format!("\\{}", c));
        }
        
        result
    }

    /// Validate and sanitize a project name
    pub fn sanitize_project_name(name: &str) -> String {
        name.chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
            .take(128)
            .collect()
    }

    /// Validate a URL
    pub fn is_valid_url(url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }
}

/// Rate limiter for API endpoints
pub struct RateLimiter {
    /// Requests per window
    max_requests: usize,
    /// Window duration in seconds
    window_seconds: u64,
    /// Request counts by key
    requests: std::sync::RwLock<std::collections::HashMap<String, (usize, std::time::Instant)>>,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_seconds,
            requests: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Check if a request is allowed
    pub fn check(&self, key: &str) -> bool {
        let mut requests = match self.requests.write() {
            Ok(r) => r,
            Err(_) => return true, // Allow on lock failure
        };

        let now = std::time::Instant::now();
        let window = std::time::Duration::from_secs(self.window_seconds);

        if let Some((count, start)) = requests.get_mut(key) {
            if now.duration_since(*start) > window {
                // Reset window
                *count = 1;
                *start = now;
                true
            } else if *count >= self.max_requests {
                false
            } else {
                *count += 1;
                true
            }
        } else {
            requests.insert(key.to_string(), (1, now));
            true
        }
    }

    /// Get remaining requests for a key
    pub fn remaining(&self, key: &str) -> usize {
        let requests = match self.requests.read() {
            Ok(r) => r,
            Err(_) => return self.max_requests,
        };

        if let Some((count, start)) = requests.get(key) {
            let window = std::time::Duration::from_secs(self.window_seconds);
            if std::time::Instant::now().duration_since(*start) > window {
                self.max_requests
            } else {
                self.max_requests.saturating_sub(*count)
            }
        } else {
            self.max_requests
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(100, 60) // 100 requests per minute
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_validator() {
        let validator = PathValidator::new(vec![PathBuf::from("/tmp")]);
        
        let result = validator.validate(Path::new("/tmp/safe/file.txt"));
        // Can't test canonicalize without actual file, but pattern check works
        
        let result = validator.validate(Path::new("/tmp/../etc/passwd"));
        assert!(!result.valid);
    }

    #[test]
    fn test_secret_detector() {
        let detector = SecretDetector::new();
        
        let content = r#"
            aws_access_key_id = "AKIAIOSFODNN7EXAMPLE"
            password = "supersecret123"
        "#;
        
        let issues = detector.scan(content, Some("config.txt"));
        assert!(!issues.is_empty());
        assert!(detector.has_secrets(content));
    }

    #[test]
    fn test_input_sanitizer() {
        assert_eq!(
            InputSanitizer::sanitize_html("<script>alert('xss')</script>"),
            "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;"
        );
        
        assert_eq!(
            InputSanitizer::sanitize_project_name("my-project_v1.0!@#"),
            "my-project_v1.0"
        );
    }

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(3, 60);
        
        assert!(limiter.check("user1"));
        assert!(limiter.check("user1"));
        assert!(limiter.check("user1"));
        assert!(!limiter.check("user1")); // Exceeded
        
        assert!(limiter.check("user2")); // Different key
    }
}
