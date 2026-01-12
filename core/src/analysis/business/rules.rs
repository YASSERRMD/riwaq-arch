//! Business Rule Analyzer
//!
//! Extracts business rules from code:
//! - Validation rules
//! - Authorization checks
//! - Business constraints
//! - Calculation formulas

use super::*;
use regex::Regex;
use std::path::Path;
use tokio::fs;
use walkdir::WalkDir;

pub struct RuleAnalyzer;

impl RuleAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub async fn analyze(&self, root_path: &Path) -> anyhow::Result<Vec<BusinessRule>> {
        let mut rules = Vec::new();

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
                
                let found = self.extract_rules(&content, &relative_path);
                rules.extend(found);
            }
        }

        Ok(rules)
    }

    fn extract_rules(&self, content: &str, file: &str) -> Vec<BusinessRule> {
        let mut rules = Vec::new();

        // Extract validation rules
        rules.extend(self.extract_validation_rules(content, file));
        
        // Extract authorization checks
        rules.extend(self.extract_authorization_rules(content, file));
        
        // Extract business constraints
        rules.extend(self.extract_constraints(content, file));
        
        // Extract from comments (TODO/BUSINESS_RULE markers)
        rules.extend(self.extract_documented_rules(content, file));

        rules
    }

    fn extract_validation_rules(&self, content: &str, file: &str) -> Vec<BusinessRule> {
        let mut rules = Vec::new();

        let patterns = vec![
            // Validation checks
            (r#"if\s+(\w+)\s*([<>=!]+)\s*(\d+)"#, "Numeric constraint"),
            (r#"\.len\(\)\s*([<>=!]+)\s*(\d+)"#, "Length constraint"),
            (r#"validate[_\s]*(\w+)\s*\([^)]*\)"#, "Validation function"),
            // Email/format validation
            (r#"(?i)is_valid_email|validate_email|email_regex"#, "Email validation"),
            // Required field checks
            (r#"(?i)(is_none|is_null|is_empty)\s*\(\s*\)"#, "Required field"),
            // Range checks
            (r#"(\w+)\s*>=\s*(\d+)\s*&&\s*\1\s*<=\s*(\d+)"#, "Range constraint"),
        ];

        for (pattern, rule_type) in patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for cap in regex.captures_iter(content) {
                    let line = content[..cap.get(0).unwrap().start()]
                        .lines().count() + 1;
                    
                    let condition = cap.get(0).map(|m| m.as_str().to_string()).unwrap_or_default();
                    let description = self.describe_validation(&condition, rule_type);

                    rules.push(BusinessRule {
                        name: format!("{}_{}", rule_type.replace(' ', "_"), line),
                        description,
                        rule_type: BusinessRuleType::Validation,
                        condition,
                        action: None,
                        entities: Vec::new(),
                        file: file.to_string(),
                        line,
                        severity: RuleSeverity::Medium,
                    });
                }
            }
        }

        rules
    }

    fn extract_authorization_rules(&self, content: &str, file: &str) -> Vec<BusinessRule> {
        let mut rules = Vec::new();

        let patterns = vec![
            // Permission checks
            (r#"(?i)has_permission\s*\(\s*["']([^"']+)["']\s*\)"#, "Permission check"),
            (r#"(?i)is_authorized|can_access|has_access"#, "Authorization check"),
            (r#"(?i)role\s*==?\s*["'](\w+)["']"#, "Role check"),
            // Admin checks
            (r#"(?i)is_admin|is_superuser|is_staff"#, "Admin check"),
            // Owner checks
            (r#"(?i)user_id\s*==?\s*(\w+)\.(?:user_id|owner_id|created_by)"#, "Owner check"),
        ];

        for (pattern, rule_type) in patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for cap in regex.captures_iter(content) {
                    let line = content[..cap.get(0).unwrap().start()]
                        .lines().count() + 1;
                    
                    let condition = cap.get(0).map(|m| m.as_str().to_string()).unwrap_or_default();
                    let permission = cap.get(1).map(|m| m.as_str()).unwrap_or("unknown");
                    
                    let description = format!("{}: {}", rule_type, permission);

                    rules.push(BusinessRule {
                        name: format!("auth_{}_{}", permission, line),
                        description,
                        rule_type: BusinessRuleType::Authorization,
                        condition,
                        action: Some("Deny access if unauthorized".to_string()),
                        entities: Vec::new(),
                        file: file.to_string(),
                        line,
                        severity: RuleSeverity::Critical,
                    });
                }
            }
        }

        rules
    }

    fn extract_constraints(&self, content: &str, file: &str) -> Vec<BusinessRule> {
        let mut rules = Vec::new();

        let patterns = vec![
            // Amount/money constraints
            (r#"(?i)(amount|balance|price|total)\s*([<>=!]+)\s*(\d+)"#, "Amount constraint"),
            // Quantity constraints
            (r#"(?i)(quantity|count|limit)\s*([<>=!]+)\s*(\d+)"#, "Quantity constraint"),
            // Date constraints
            (r#"(?i)(expires?_at|valid_until|deadline)\s*([<>=!]+)"#, "Date constraint"),
            // Status constraints
            (r#"(?i)status\s*==?\s*["']?(\w+)["']?"#, "Status constraint"),
            // Unique constraints
            (r#"(?i)unique|already_exists|duplicate"#, "Uniqueness constraint"),
        ];

        for (pattern, rule_type) in patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for cap in regex.captures_iter(content) {
                    let line = content[..cap.get(0).unwrap().start()]
                        .lines().count() + 1;
                    
                    let condition = cap.get(0).map(|m| m.as_str().to_string()).unwrap_or_default();
                    let field = cap.get(1).map(|m| m.as_str()).unwrap_or("field");
                    
                    let description = format!("{} on {}", rule_type, field);

                    rules.push(BusinessRule {
                        name: format!("constraint_{}_{}", field, line),
                        description,
                        rule_type: BusinessRuleType::Constraint,
                        condition,
                        action: None,
                        entities: Vec::new(),
                        file: file.to_string(),
                        line,
                        severity: RuleSeverity::High,
                    });
                }
            }
        }

        rules
    }

    fn extract_documented_rules(&self, content: &str, file: &str) -> Vec<BusinessRule> {
        let mut rules = Vec::new();

        // Look for business rule markers in comments
        let patterns = vec![
            r#"(?i)//\s*(?:BUSINESS_RULE|BR|RULE):\s*(.+)"#,
            r#"(?i)#\s*(?:BUSINESS_RULE|BR|RULE):\s*(.+)"#,
            r#"(?i)/\*\*?\s*(?:BUSINESS_RULE|BR|RULE):\s*(.+)\*/"#,
            r#"(?i)///\s*(?:BUSINESS_RULE|BR|RULE):\s*(.+)"#,
        ];

        for pattern in patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for cap in regex.captures_iter(content) {
                    let line = content[..cap.get(0).unwrap().start()]
                        .lines().count() + 1;
                    
                    let description = cap.get(1)
                        .map(|m| m.as_str().trim().to_string())
                        .unwrap_or_default();

                    rules.push(BusinessRule {
                        name: format!("documented_rule_{}", line),
                        description: description.clone(),
                        rule_type: BusinessRuleType::Policy,
                        condition: description,
                        action: None,
                        entities: Vec::new(),
                        file: file.to_string(),
                        line,
                        severity: RuleSeverity::Info,
                    });
                }
            }
        }

        rules
    }

    fn describe_validation(&self, condition: &str, rule_type: &str) -> String {
        if condition.contains('<') && condition.contains('>') {
            format!("{}: Value must be within range", rule_type)
        } else if condition.contains("len()") {
            format!("{}: Length constraint", rule_type)
        } else if condition.contains(">=") {
            format!("{}: Minimum value constraint", rule_type)
        } else if condition.contains("<=") {
            format!("{}: Maximum value constraint", rule_type)
        } else if condition.contains("==") {
            format!("{}: Equality constraint", rule_type)
        } else if condition.contains("!=") {
            format!("{}: Not-equal constraint", rule_type)
        } else {
            format!("{}: {}", rule_type, condition)
        }
    }

    fn is_source_file(path: &Path) -> bool {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        matches!(ext, "rs" | "py" | "js" | "ts" | "go" | "java")
    }

    fn should_skip_dir(path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        path_str.contains("node_modules") ||
        path_str.contains("target") ||
        path_str.contains(".git") ||
        path_str.contains("vendor") ||
        path_str.contains("test")
    }
}

impl Default for RuleAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
