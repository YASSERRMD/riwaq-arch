//! Main codebase analyzer that orchestrates all analysis components.
//!
//! This module ties together filesystem scanning, code parsing, git analysis,
//! and dependency graph construction to produce a complete `CodebaseSnapshot`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tracing::{info, warn};

use crate::analysis::fs_scanner::{FsScanner, ScanConfig, ScannedFile};
use crate::analysis::git::{GitAnalysisConfig, GitAnalyzer};
use crate::analysis::parser::CodeParser;
use crate::errors::Result;
use crate::logging::ProgressReporter;
use crate::models::file::Language;
use crate::models::module::{DependencyGraph, DependencyKind, ModuleSummary};
use crate::models::snapshot::{AnalysisConfig, CodebaseSnapshot, ServiceInfo, ServiceKind};

/// Configuration for the codebase analyzer.
#[derive(Debug, Clone)]
pub struct AnalyzerConfig {
    /// Filesystem scan configuration.
    pub scan_config: ScanConfig,

    /// Git analysis configuration.
    pub git_config: GitAnalysisConfig,

    /// Whether to parse code (vs just listing files).
    pub parse_code: bool,

    /// Whether to analyze git history.
    pub analyze_git: bool,

    /// Whether to build dependency graph.
    pub build_dependency_graph: bool,

    /// Whether to detect services/entry points.
    pub detect_services: bool,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            scan_config: ScanConfig::default(),
            git_config: GitAnalysisConfig::default(),
            parse_code: true,
            analyze_git: true,
            build_dependency_graph: true,
            detect_services: true,
        }
    }
}

/// Main codebase analyzer.
pub struct CodebaseAnalyzer {
    config: AnalyzerConfig,
    root_path: PathBuf,
}

impl CodebaseAnalyzer {
    /// Create a new analyzer for the given path.
    pub fn new(path: impl AsRef<Path>) -> Self {
        let root_path = path.as_ref().to_path_buf();
        Self {
            config: AnalyzerConfig {
                scan_config: ScanConfig::new(&root_path),
                ..Default::default()
            },
            root_path,
        }
    }

    /// Create an analyzer with custom configuration.
    pub fn with_config(path: impl AsRef<Path>, config: AnalyzerConfig) -> Self {
        Self {
            config,
            root_path: path.as_ref().to_path_buf(),
        }
    }

    /// Set the maximum number of commits to analyze.
    pub fn with_max_commits(mut self, max: usize) -> Self {
        self.config.git_config.max_commits = max;
        self
    }

    /// Set whether to include test files.
    pub fn with_include_tests(mut self, include: bool) -> Self {
        self.config.scan_config.include_tests = include;
        self
    }

    /// Disable git analysis.
    pub fn without_git(mut self) -> Self {
        self.config.analyze_git = false;
        self
    }

    /// Disable code parsing (just scan files).
    pub fn without_parsing(mut self) -> Self {
        self.config.parse_code = false;
        self
    }

    /// Run the full analysis and return a codebase snapshot.
    pub async fn analyze(&self) -> Result<CodebaseSnapshot> {
        let start = Instant::now();
        info!(path = ?self.root_path, "Starting codebase analysis");

        let mut snapshot = CodebaseSnapshot::new(self.root_path.clone());

        // Step 1: Scan filesystem
        let files = self.scan_files().await?;
        info!(count = files.len(), "Files discovered");

        // Step 2: Parse files
        if self.config.parse_code {
            snapshot.files = self.parse_files(&files).await?;
        } else {
            // Just create basic summaries
            snapshot.files = files
                .into_iter()
                .map(|f| {
                    let mut summary = crate::models::file::FileSummary::new(f.relative_path, f.language);
                    summary.size_bytes = f.size;
                    summary.is_test = f.is_test;
                    summary
                })
                .collect();
        }

        // Step 3: Analyze git history
        if self.config.analyze_git {
            match self.analyze_git().await {
                Ok(insights) => {
                    snapshot.git_insights = insights;
                }
                Err(e) => {
                    warn!(error = %e, "Git analysis failed, continuing without git insights");
                }
            }
        }

        // Step 4: Build modules and dependency graph
        if self.config.build_dependency_graph {
            let (modules, graph) = self.build_modules_and_graph(&snapshot.files)?;
            snapshot.modules = modules;
            snapshot.dependency_graph = graph;
        }

        // Step 5: Detect services
        if self.config.detect_services {
            snapshot.services = self.detect_services(&snapshot.files)?;
        }

        // Step 6: Compute statistics
        snapshot.compute_statistics();

        // Update metadata
        snapshot.metadata.analysis_duration_ms = start.elapsed().as_millis() as u64;
        snapshot.metadata.config = self.build_analysis_config();

        info!(
            duration_ms = snapshot.metadata.analysis_duration_ms,
            files = snapshot.files.len(),
            modules = snapshot.modules.len(),
            "Analysis complete"
        );

        Ok(snapshot)
    }

    /// Scan the filesystem for source files.
    async fn scan_files(&self) -> Result<Vec<ScannedFile>> {
        let scanner = FsScanner::new(self.config.scan_config.clone());
        scanner.scan()
    }

    /// Parse all discovered files.
    async fn parse_files(
        &self,
        files: &[ScannedFile],
    ) -> Result<Vec<crate::models::file::FileSummary>> {
        let mut parser = CodeParser::new()?;
        let mut summaries = Vec::with_capacity(files.len());
        let mut progress = ProgressReporter::new("Parsing files", files.len());

        for file in files {
            progress.tick();

            // Read file content
            let content = match file.read_contents() {
                Ok(c) => c,
                Err(e) => {
                    warn!(path = ?file.relative_path, error = %e, "Failed to read file");
                    continue;
                }
            };

            // Parse the file
            match parser.parse_file(&file.relative_path, &content, file.language) {
                Ok(mut summary) => {
                    summary.size_bytes = file.size;
                    summary.is_test = file.is_test;
                    summaries.push(summary);
                }
                Err(e) => {
                    warn!(path = ?file.relative_path, error = %e, "Failed to parse file");
                }
            }
        }

        progress.finish();
        Ok(summaries)
    }

    /// Analyze git history.
    async fn analyze_git(&self) -> Result<crate::models::git::GitInsights> {
        let analyzer = GitAnalyzer::with_config(self.config.git_config.clone());
        analyzer.analyze(&self.root_path)
    }

    /// Build module summaries and dependency graph from parsed files.
    fn build_modules_and_graph(
        &self,
        files: &[crate::models::file::FileSummary],
    ) -> Result<(Vec<ModuleSummary>, DependencyGraph)> {
        let mut modules: HashMap<String, ModuleSummary> = HashMap::new();
        let mut graph = DependencyGraph::new();

        // Group files into modules
        for file in files {
            let module_path = self.infer_module_path(&file.path, file.language);

            let module = modules.entry(module_path.clone()).or_insert_with(|| {
                let name = module_path.split("::").last().unwrap_or(&module_path).to_string();
                ModuleSummary::new(name, module_path.clone())
            });

            module.files.push(file.path.clone());
            module.metrics.file_count += 1;
            module.metrics.lines_of_code += file.line_count;
            module.metrics.function_count += file.functions.len();
            module.metrics.type_count += file.types.len();

            // Collect public items
            for func in &file.functions {
                if func.visibility == crate::models::file::Visibility::Public {
                    module.public_functions.push(func.name.clone());
                }
            }
            for typ in &file.types {
                if typ.visibility == crate::models::file::Visibility::Public {
                    module.public_types.push(typ.name.clone());
                }
            }

            // Add to graph
            graph.add_node(module_path.clone());

            // Create edges from imports
            for import in &file.imports {
                let target_module = self.import_to_module(&import.path, file.language);
                if !target_module.is_empty() && target_module != module_path {
                    graph.add_edge(&module_path, &target_module, DependencyKind::Function);
                }
            }
        }

        // Find circular dependencies
        graph.find_circular_dependencies();

        // Compute module metrics (cohesion, coupling)
        for module in modules.values_mut() {
            let deps = graph.get_dependencies(&module.path);
            let _dependents = graph.get_dependents(&module.path);

            // Simple coupling score: ratio of external dependencies to internal items
            let external_deps = deps.len();
            let internal_items = module.public_functions.len() + module.public_types.len();
            module.metrics.coupling_score = if internal_items > 0 {
                (external_deps as f32 / (internal_items + external_deps) as f32).min(1.0)
            } else {
                0.5
            };

            // Simple cohesion score (inverse of coupling for now)
            module.metrics.cohesion_score = 1.0 - module.metrics.coupling_score;

            // Add dependencies to module
            for dep in deps {
                module.dependencies.push(crate::models::module::ModuleDependency {
                    target: dep.target.clone(),
                    kind: dep.kind,
                    reference_count: dep.weight,
                });
            }
        }

        Ok((modules.into_values().collect(), graph))
    }

    /// Infer module path from file path.
    fn infer_module_path(&self, path: &Path, language: Language) -> String {
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

        // Remove common file suffixes
        let clean_stem = stem
            .trim_end_matches("_test")
            .trim_end_matches("_spec")
            .trim_end_matches(".test")
            .trim_end_matches(".spec");

        // Build path parts
        let parts: Vec<&str> = path
            .parent()
            .unwrap_or(Path::new(""))
            .components()
            .filter_map(|c| c.as_os_str().to_str())
            .filter(|&s| s != "src" && s != "lib" && s != "pkg")
            .collect();

        let separator = match language {
            Language::Rust => "::",
            Language::Python => ".",
            Language::Go => "/",
            _ => "/",
        };

        if parts.is_empty() {
            clean_stem.to_string()
        } else {
            format!("{}{}{}", parts.join(separator), separator, clean_stem)
        }
    }

    /// Convert import path to module path.
    fn import_to_module(&self, import_path: &str, language: Language) -> String {
        match language {
            Language::Rust => {
                // Extract crate and top-level module
                let parts: Vec<&str> = import_path.split("::").collect();
                if parts.len() >= 2 {
                    parts[..2].join("::")
                } else {
                    import_path.to_string()
                }
            }
            Language::Python => {
                // Take first two parts
                let parts: Vec<&str> = import_path
                    .trim_start_matches("from ")
                    .trim_start_matches("import ")
                    .split(&['.', ' '][..])
                    .filter(|s| !s.is_empty())
                    .collect();
                if parts.len() >= 2 {
                    parts[..2].join(".")
                } else {
                    parts.join(".")
                }
            }
            Language::JavaScript | Language::TypeScript => {
                // Handle relative and package imports
                let clean = import_path
                    .trim_start_matches("import ")
                    .trim_start_matches("from ")
                    .trim_matches(&['"', '\'', ';', ' '][..]);

                if clean.starts_with('.') {
                    // Relative import - use as-is
                    clean.to_string()
                } else {
                    // Package import - take package name
                    clean.split('/').next().unwrap_or(clean).to_string()
                }
            }
            Language::Go => {
                // Take the import path
                import_path
                    .trim_matches(&['"', '\'', '`'][..])
                    .to_string()
            }
            _ => import_path.to_string(),
        }
    }

    /// Detect services and entry points.
    fn detect_services(
        &self,
        files: &[crate::models::file::FileSummary],
    ) -> Result<Vec<ServiceInfo>> {
        let mut services = Vec::new();

        for file in files {
            // Check for main files
            let file_name = file.file_name().to_lowercase();
            let is_main = file_name == "main.rs"
                || file_name == "main.py"
                || file_name == "main.go"
                || file_name == "index.js"
                || file_name == "index.ts"
                || file_name == "app.js"
                || file_name == "app.ts"
                || file_name == "server.js"
                || file_name == "server.ts";

            if !is_main {
                continue;
            }

            // Determine service kind from file content patterns
            let kind = self.detect_service_kind(file);

            let name = file
                .path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("main")
                .to_string();

            services.push(ServiceInfo {
                name,
                kind,
                entry_file: file.path.clone(),
                description: None,
                dependencies: Vec::new(),
                endpoints: Vec::new(),
            });
        }

        Ok(services)
    }

    /// Detect the kind of service from file content.
    fn detect_service_kind(&self, file: &crate::models::file::FileSummary) -> ServiceKind {
        // Check imports for framework patterns
        for import in &file.imports {
            let import_lower = import.path.to_lowercase();

            // HTTP servers
            if import_lower.contains("actix")
                || import_lower.contains("axum")
                || import_lower.contains("express")
                || import_lower.contains("fastify")
                || import_lower.contains("flask")
                || import_lower.contains("django")
                || import_lower.contains("gin")
                || import_lower.contains("fiber")
                || import_lower.contains("net/http")
            {
                return ServiceKind::HttpServer;
            }

            // gRPC
            if import_lower.contains("grpc") || import_lower.contains("tonic") {
                return ServiceKind::GrpcServer;
            }

            // GraphQL
            if import_lower.contains("graphql") || import_lower.contains("apollo") {
                return ServiceKind::GraphQlServer;
            }

            // WebSocket
            if import_lower.contains("websocket") || import_lower.contains("ws") {
                return ServiceKind::WebSocket;
            }
        }

        // Check for main function (CLI)
        for func in &file.functions {
            if func.name == "main" {
                return ServiceKind::Cli;
            }
        }

        ServiceKind::Unknown
    }

    /// Build the analysis config for the snapshot.
    fn build_analysis_config(&self) -> AnalysisConfig {
        AnalysisConfig {
            max_file_size: self.config.scan_config.max_file_size,
            max_commits: self.config.git_config.max_commits,
            include_tests: self.config.scan_config.include_tests,
            excluded_dirs: self.config.scan_config.excluded_dirs.iter().cloned().collect(),
            included_extensions: self.config.scan_config.included_extensions.iter().cloned().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    fn create_test_project() -> TempDir {
        let dir = TempDir::new().unwrap();

        // Create src directory
        fs::create_dir_all(dir.path().join("src")).unwrap();

        // Create main.rs
        fs::write(
            dir.path().join("src/main.rs"),
            r#"
use mylib::utils;

fn main() {
    println!("Hello!");
}
"#,
        )
        .unwrap();

        // Create lib.rs
        fs::write(
            dir.path().join("src/lib.rs"),
            r#"
pub mod utils;
pub mod models;

pub fn hello() -> String {
    "Hello".to_string()
}
"#,
        )
        .unwrap();

        // Create utils.rs
        fs::write(
            dir.path().join("src/utils.rs"),
            r#"
pub fn format_name(name: &str) -> String {
    format!("Name: {}", name)
}
"#,
        )
        .unwrap();

        dir
    }

    #[tokio::test]
    async fn test_analyzer_basic() {
        let dir = create_test_project();
        let analyzer = CodebaseAnalyzer::new(dir.path()).without_git();

        let snapshot = analyzer.analyze().await.unwrap();

        assert!(!snapshot.files.is_empty());
        assert!(!snapshot.modules.is_empty());
    }

    #[test]
    fn test_infer_module_path() {
        let analyzer = CodebaseAnalyzer::new(".");

        let path = Path::new("src/utils/helpers.rs");
        let module = analyzer.infer_module_path(path, Language::Rust);
        assert!(module.contains("helpers"));

        let path = Path::new("src/utils/helpers_test.rs");
        let module = analyzer.infer_module_path(path, Language::Rust);
        assert!(module.contains("helpers"));
        assert!(!module.contains("test"));
    }

    #[test]
    fn test_import_to_module() {
        let analyzer = CodebaseAnalyzer::new(".");

        let module = analyzer.import_to_module("std::collections::HashMap", Language::Rust);
        assert_eq!(module, "std::collections");

        let module = analyzer.import_to_module("from flask import Flask", Language::Python);
        assert!(module.contains("flask"));
    }
}
