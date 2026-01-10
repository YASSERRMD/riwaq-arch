//! Function-level data models.

use serde::{Deserialize, Serialize};

use super::file::Visibility;

/// Summary of a function or method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSummary {
    /// Function name.
    pub name: String,

    /// Full signature (e.g., "fn process(data: &str) -> Result<()>").
    pub signature: String,

    /// Documentation comment.
    pub doc_comment: Option<String>,

    /// Line where the function starts.
    pub line_start: usize,

    /// Line where the function ends.
    pub line_end: usize,

    /// Parameters.
    pub parameters: Vec<ParameterInfo>,

    /// Return type (as string).
    pub return_type: Option<String>,

    /// Whether this is an async function.
    pub is_async: bool,

    /// Visibility.
    pub visibility: Visibility,

    /// Complexity metrics.
    pub complexity: Option<ComplexityMetrics>,

    /// Functions called within this function.
    pub calls: Vec<String>,
}

impl FunctionSummary {
    /// Create a new function summary with minimal information.
    pub fn new(name: String, signature: String, line_start: usize, line_end: usize) -> Self {
        Self {
            name,
            signature,
            doc_comment: None,
            line_start,
            line_end,
            parameters: Vec::new(),
            return_type: None,
            is_async: false,
            visibility: Visibility::Private,
            complexity: None,
            calls: Vec::new(),
        }
    }

    /// Get the number of lines in this function.
    pub fn line_count(&self) -> usize {
        self.line_end.saturating_sub(self.line_start) + 1
    }

    /// Check if this is a test function.
    pub fn is_test(&self) -> bool {
        self.name.starts_with("test_")
            || self.name.ends_with("_test")
            || self.doc_comment.as_ref().is_some_and(|d| d.contains("#[test]"))
    }
}

/// Function parameter information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    /// Parameter name.
    pub name: String,

    /// Type annotation (as string).
    pub type_annotation: Option<String>,

    /// Default value (if any).
    pub default_value: Option<String>,

    /// Whether this is a rest/variadic parameter.
    pub is_variadic: bool,
}

impl ParameterInfo {
    /// Create a new parameter info.
    pub fn new(name: String) -> Self {
        Self {
            name,
            type_annotation: None,
            default_value: None,
            is_variadic: false,
        }
    }

    /// Create a parameter with type annotation.
    pub fn with_type(name: String, type_annotation: String) -> Self {
        Self {
            name,
            type_annotation: Some(type_annotation),
            default_value: None,
            is_variadic: false,
        }
    }
}

/// Complexity metrics for a function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    /// Cyclomatic complexity.
    pub cyclomatic: u32,

    /// Number of branches (if/else, match, etc.).
    pub branches: u32,

    /// Number of loops.
    pub loops: u32,

    /// Nesting depth.
    pub max_nesting: u32,
}

impl ComplexityMetrics {
    /// Create default complexity metrics.
    pub fn new() -> Self {
        Self {
            cyclomatic: 1,
            branches: 0,
            loops: 0,
            max_nesting: 0,
        }
    }

    /// Check if the function has high complexity.
    pub fn is_high_complexity(&self) -> bool {
        self.cyclomatic > 10 || self.max_nesting > 4
    }
}

impl Default for ComplexityMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_summary_line_count() {
        let func = FunctionSummary::new(
            "test".to_string(),
            "fn test()".to_string(),
            10,
            15,
        );
        assert_eq!(func.line_count(), 6);
    }

    #[test]
    fn test_function_is_test() {
        let test_func = FunctionSummary::new(
            "test_something".to_string(),
            "fn test_something()".to_string(),
            1,
            5,
        );
        assert!(test_func.is_test());

        let regular_func = FunctionSummary::new(
            "process".to_string(),
            "fn process()".to_string(),
            1,
            5,
        );
        assert!(!regular_func.is_test());
    }

    #[test]
    fn test_complexity_metrics() {
        let mut metrics = ComplexityMetrics::new();
        assert!(!metrics.is_high_complexity());

        metrics.cyclomatic = 15;
        assert!(metrics.is_high_complexity());
    }
}
