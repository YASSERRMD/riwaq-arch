//! File-level data models.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::function::FunctionSummary;

/// Supported programming languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Java,
    Kotlin,
    Ruby,
    Php,
    C,
    Cpp,
    CSharp,
    Swift,
    Scala,
    Unknown,
}

impl Language {
    /// Detect language from file extension.
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Language::Rust,
            "py" => Language::Python,
            "js" | "mjs" | "cjs" => Language::JavaScript,
            "ts" | "tsx" => Language::TypeScript,
            "jsx" => Language::JavaScript,
            "go" => Language::Go,
            "java" => Language::Java,
            "kt" | "kts" => Language::Kotlin,
            "rb" => Language::Ruby,
            "php" => Language::Php,
            "c" | "h" => Language::C,
            "cpp" | "cc" | "cxx" | "hpp" | "hxx" => Language::Cpp,
            "cs" => Language::CSharp,
            "swift" => Language::Swift,
            "scala" | "sc" => Language::Scala,
            _ => Language::Unknown,
        }
    }

    /// Check if the language is supported for tree-sitter parsing.
    pub fn is_supported(&self) -> bool {
        matches!(
            self,
            Language::Rust
                | Language::Python
                | Language::JavaScript
                | Language::TypeScript
                | Language::Go
        )
    }

    /// Get the file extension for this language.
    pub fn extension(&self) -> &'static str {
        match self {
            Language::Rust => "rs",
            Language::Python => "py",
            Language::JavaScript => "js",
            Language::TypeScript => "ts",
            Language::Go => "go",
            Language::Java => "java",
            Language::Kotlin => "kt",
            Language::Ruby => "rb",
            Language::Php => "php",
            Language::C => "c",
            Language::Cpp => "cpp",
            Language::CSharp => "cs",
            Language::Swift => "swift",
            Language::Scala => "scala",
            Language::Unknown => "",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Language::Rust => "Rust",
            Language::Python => "Python",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Go => "Go",
            Language::Java => "Java",
            Language::Kotlin => "Kotlin",
            Language::Ruby => "Ruby",
            Language::Php => "PHP",
            Language::C => "C",
            Language::Cpp => "C++",
            Language::CSharp => "C#",
            Language::Swift => "Swift",
            Language::Scala => "Scala",
            Language::Unknown => "Unknown",
        };
        write!(f, "{}", name)
    }
}

/// Summary of a source code file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSummary {
    /// Relative path from project root.
    pub path: PathBuf,

    /// Detected programming language.
    pub language: Language,

    /// File size in bytes.
    pub size_bytes: u64,

    /// Number of lines in the file.
    pub line_count: usize,

    /// Functions defined in this file.
    pub functions: Vec<FunctionSummary>,

    /// Classes/structs/types defined in this file.
    pub types: Vec<TypeSummary>,

    /// Imports in this file.
    pub imports: Vec<ImportInfo>,

    /// Exports from this file.
    pub exports: Vec<ExportInfo>,

    /// Module path (e.g., "myproject::utils::helpers").
    pub module_path: Option<String>,

    /// Whether this file is a test file.
    pub is_test: bool,

    /// Content hash for change detection.
    pub content_hash: String,
}

impl FileSummary {
    /// Create a new file summary.
    pub fn new(path: PathBuf, language: Language) -> Self {
        Self {
            path,
            language,
            size_bytes: 0,
            line_count: 0,
            functions: Vec::new(),
            types: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            module_path: None,
            is_test: false,
            content_hash: String::new(),
        }
    }

    /// Get the file name without path.
    pub fn file_name(&self) -> &str {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
    }
}

/// Summary of a type (class, struct, enum, interface).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeSummary {
    /// Name of the type.
    pub name: String,

    /// Kind of type.
    pub kind: TypeKind,

    /// Documentation comment.
    pub doc_comment: Option<String>,

    /// Line number where the type is defined.
    pub line_start: usize,

    /// Line number where the type ends.
    pub line_end: usize,

    /// Methods defined on this type.
    pub methods: Vec<FunctionSummary>,

    /// Fields/properties.
    pub fields: Vec<FieldInfo>,

    /// Visibility (public, private, etc.)
    pub visibility: Visibility,
}

/// Kind of type definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TypeKind {
    Class,
    Struct,
    Enum,
    Interface,
    Trait,
    TypeAlias,
    Union,
}

/// Visibility modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    #[default]
    Private,
    Protected,
    Internal,
    Crate,
}

/// Field information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldInfo {
    /// Field name.
    pub name: String,

    /// Field type (as string).
    pub type_annotation: Option<String>,

    /// Documentation comment.
    pub doc_comment: Option<String>,

    /// Visibility.
    pub visibility: Visibility,
}

/// Import statement information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportInfo {
    /// Import path (e.g., "std::collections::HashMap").
    pub path: String,

    /// Imported items (e.g., ["HashMap", "HashSet"]).
    pub items: Vec<String>,

    /// Whether it's a wildcard import (e.g., "use foo::*").
    pub is_wildcard: bool,

    /// Alias if renamed (e.g., "use foo as bar").
    pub alias: Option<String>,

    /// Line number.
    pub line: usize,
}

/// Export statement information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportInfo {
    /// Exported item name.
    pub name: String,

    /// Whether it's a default export.
    pub is_default: bool,

    /// Whether it's a re-export.
    pub is_reexport: bool,

    /// Line number.
    pub line: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_extension() {
        assert_eq!(Language::from_extension("rs"), Language::Rust);
        assert_eq!(Language::from_extension("py"), Language::Python);
        assert_eq!(Language::from_extension("ts"), Language::TypeScript);
        assert_eq!(Language::from_extension("tsx"), Language::TypeScript);
        assert_eq!(Language::from_extension("unknown"), Language::Unknown);
    }

    #[test]
    fn test_language_is_supported() {
        assert!(Language::Rust.is_supported());
        assert!(Language::Python.is_supported());
        assert!(!Language::Java.is_supported());
    }

    #[test]
    fn test_file_summary_creation() {
        let summary = FileSummary::new(PathBuf::from("src/main.rs"), Language::Rust);
        assert_eq!(summary.file_name(), "main.rs");
        assert!(!summary.is_test);
    }
}
