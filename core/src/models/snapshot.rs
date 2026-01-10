//! Codebase snapshot model - the main output of analysis.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::file::FileSummary;
use super::git::GitInsights;
use super::module::{DependencyGraph, ModuleSummary};

/// Complete snapshot of a codebase after analysis.
///
/// This is the main data structure produced by the analyzer and
/// consumed by documentation generators and the LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodebaseSnapshot {
    /// Snapshot metadata.
    pub metadata: SnapshotMetadata,

    /// All analyzed files.
    pub files: Vec<FileSummary>,

    /// Detected modules/packages.
    pub modules: Vec<ModuleSummary>,

    /// Dependency graph between modules.
    pub dependency_graph: DependencyGraph,

    /// Git history insights.
    pub git_insights: GitInsights,

    /// Detected services/entry points.
    pub services: Vec<ServiceInfo>,

    /// API endpoints (if detected).
    pub api_endpoints: Vec<ApiEndpoint>,

    /// Overall codebase statistics.
    pub statistics: CodebaseStatistics,
}

impl CodebaseSnapshot {
    /// Create a new empty snapshot.
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            metadata: SnapshotMetadata::new(root_path),
            files: Vec::new(),
            modules: Vec::new(),
            dependency_graph: DependencyGraph::default(),
            git_insights: GitInsights::default(),
            services: Vec::new(),
            api_endpoints: Vec::new(),
            statistics: CodebaseStatistics::default(),
        }
    }

    /// Get a file by path.
    pub fn get_file(&self, path: &PathBuf) -> Option<&FileSummary> {
        self.files.iter().find(|f| &f.path == path)
    }

    /// Get a module by path.
    pub fn get_module(&self, path: &str) -> Option<&ModuleSummary> {
        self.modules.iter().find(|m| m.path == path)
    }

    /// Compute and update statistics.
    pub fn compute_statistics(&mut self) {
        self.statistics = CodebaseStatistics {
            total_files: self.files.len(),
            total_modules: self.modules.len(),
            total_functions: self.files.iter().map(|f| f.functions.len()).sum(),
            total_types: self.files.iter().map(|f| f.types.len()).sum(),
            total_lines: self.files.iter().map(|f| f.line_count).sum(),
            languages: self.count_languages(),
            test_coverage_estimate: self.estimate_test_coverage(),
            circular_dependencies: self.dependency_graph.circular_deps.len(),
        };
    }

    fn count_languages(&self) -> Vec<LanguageCount> {
        use std::collections::HashMap;
        let mut counts: HashMap<_, (usize, usize)> = HashMap::new();

        for file in &self.files {
            let entry = counts.entry(file.language).or_insert((0, 0));
            entry.0 += 1;
            entry.1 += file.line_count;
        }

        counts
            .into_iter()
            .map(|(lang, (files, lines))| LanguageCount {
                language: lang.to_string(),
                file_count: files,
                line_count: lines,
            })
            .collect()
    }

    fn estimate_test_coverage(&self) -> f32 {
        let test_files = self.files.iter().filter(|f| f.is_test).count();
        let total = self.files.len();
        if total == 0 {
            0.0
        } else {
            (test_files as f32 / total as f32) * 100.0
        }
    }
}

/// Metadata about the snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotMetadata {
    /// Project root path.
    pub root_path: PathBuf,

    /// Project name (from directory name or config).
    pub project_name: String,

    /// Version of the analyzer.
    pub analyzer_version: String,

    /// Timestamp when the snapshot was created.
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Duration of the analysis in milliseconds.
    pub analysis_duration_ms: u64,

    /// Configuration used for analysis.
    pub config: AnalysisConfig,
}

impl SnapshotMetadata {
    /// Create new metadata.
    pub fn new(root_path: PathBuf) -> Self {
        let project_name = root_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Self {
            root_path,
            project_name,
            analyzer_version: crate::VERSION.to_string(),
            created_at: chrono::Utc::now(),
            analysis_duration_ms: 0,
            config: AnalysisConfig::default(),
        }
    }
}

/// Configuration used for analysis.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisConfig {
    /// Maximum file size to analyze.
    pub max_file_size: u64,

    /// Maximum commits to analyze.
    pub max_commits: usize,

    /// Include test files.
    pub include_tests: bool,

    /// Excluded directories.
    pub excluded_dirs: Vec<String>,

    /// Included file extensions.
    pub included_extensions: Vec<String>,
}

/// Overall codebase statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodebaseStatistics {
    /// Total number of files analyzed.
    pub total_files: usize,

    /// Total number of modules detected.
    pub total_modules: usize,

    /// Total number of functions.
    pub total_functions: usize,

    /// Total number of types (classes, structs, etc.).
    pub total_types: usize,

    /// Total lines of code.
    pub total_lines: usize,

    /// Languages and their line counts.
    pub languages: Vec<LanguageCount>,

    /// Estimated test coverage percentage.
    pub test_coverage_estimate: f32,

    /// Number of circular dependencies.
    pub circular_dependencies: usize,
}

/// Count of files and lines per language.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageCount {
    /// Language name.
    pub language: String,

    /// Number of files.
    pub file_count: usize,

    /// Number of lines.
    pub line_count: usize,
}

/// Information about a service or entry point.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInfo {
    /// Service name.
    pub name: String,

    /// Service type.
    pub kind: ServiceKind,

    /// Entry point file.
    pub entry_file: PathBuf,

    /// Description.
    pub description: Option<String>,

    /// Dependencies (other services).
    pub dependencies: Vec<String>,

    /// API endpoints exposed by this service.
    pub endpoints: Vec<String>,
}

/// Kind of service.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceKind {
    HttpServer,
    GrpcServer,
    GraphQlServer,
    WebSocket,
    Worker,
    Cli,
    Library,
    Unknown,
}

/// API endpoint information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiEndpoint {
    /// HTTP method (GET, POST, etc.).
    pub method: String,

    /// Path pattern.
    pub path: String,

    /// Handler function name.
    pub handler: String,

    /// Source file.
    pub file: PathBuf,

    /// Line number.
    pub line: usize,

    /// Description (from doc comments).
    pub description: Option<String>,

    /// Request parameters.
    pub parameters: Vec<ApiParameter>,

    /// Response type.
    pub response_type: Option<String>,
}

/// API parameter information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiParameter {
    /// Parameter name.
    pub name: String,

    /// Parameter type.
    pub param_type: String,

    /// Where the parameter comes from (path, query, body, header).
    pub location: ParameterLocation,

    /// Whether the parameter is required.
    pub required: bool,

    /// Description.
    pub description: Option<String>,
}

/// Location of an API parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterLocation {
    Path,
    Query,
    Body,
    Header,
    Cookie,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_creation() {
        let snapshot = CodebaseSnapshot::new(PathBuf::from("/test/project"));
        assert_eq!(snapshot.metadata.project_name, "project");
        assert!(snapshot.files.is_empty());
    }

    #[test]
    fn test_compute_statistics() {
        let mut snapshot = CodebaseSnapshot::new(PathBuf::from("/test"));
        snapshot.compute_statistics();
        assert_eq!(snapshot.statistics.total_files, 0);
    }
}
