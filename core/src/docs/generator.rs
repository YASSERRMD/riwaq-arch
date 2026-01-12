//! Main documentation generator.
//!
//! This module orchestrates the generation of all documentation types
//! from a codebase snapshot, optionally using LLM for enhanced content.
//! 
//! All diagrams are rendered as SVG images, not as Mermaid text.

use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

use super::markdown::MarkdownBuilder;
use super::mermaid;
use crate::diagrams::{DiagramRenderer, DiagramRendererConfig};
use crate::errors::Result;
use crate::llm::LLMConfig;
use crate::models::snapshot::CodebaseSnapshot;

/// Configuration for documentation generation.
#[derive(Debug, Clone)]
pub struct DocGeneratorConfig {
    /// Output directory for generated docs.
    pub output_dir: PathBuf,

    /// Whether to include Mermaid diagrams.
    pub include_diagrams: bool,

    /// Whether to generate ADRs.
    pub generate_adrs: bool,

    /// Whether to generate API reference.
    pub generate_api_ref: bool,

    /// Whether to use LLM for enhanced content.
    pub use_llm: bool,

    /// LLM configuration (if using LLM).
    pub llm_config: Option<LLMConfig>,

    /// Project name override.
    pub project_name: Option<String>,
}

impl Default for DocGeneratorConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./generated-docs"),
            include_diagrams: true,
            generate_adrs: true,
            generate_api_ref: true,
            use_llm: true,
            llm_config: None,
            project_name: None,
        }
    }
}

impl DocGeneratorConfig {
    /// Create a new config with the output directory.
    pub fn new(output_dir: impl AsRef<Path>) -> Self {
        Self {
            output_dir: output_dir.as_ref().to_path_buf(),
            ..Default::default()
        }
    }

    /// Set whether to use LLM.
    pub fn with_llm(mut self, use_llm: bool) -> Self {
        self.use_llm = use_llm;
        self
    }

    /// Set the LLM configuration.
    pub fn with_llm_config(mut self, config: LLMConfig) -> Self {
        self.llm_config = Some(config);
        self.use_llm = true;
        self
    }

    /// Set the project name.
    pub fn with_project_name(mut self, name: impl Into<String>) -> Self {
        self.project_name = Some(name.into());
        self
    }
}

/// Result of documentation generation.
#[derive(Debug)]
pub struct GeneratedDocs {
    /// List of generated files.
    pub files: Vec<GeneratedFile>,

    /// Output directory.
    pub output_dir: PathBuf,

    /// Total generation time in milliseconds.
    pub generation_time_ms: u64,
}

/// A generated documentation file.
#[derive(Debug)]
pub struct GeneratedFile {
    /// Relative path from output directory.
    pub path: PathBuf,

    /// Absolute path.
    pub absolute_path: PathBuf,

    /// File size in bytes.
    pub size: u64,

    /// Type of documentation.
    pub doc_type: DocType,
}

/// Type of documentation file.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DocType {
    ArchitectureOverview,
    ModuleDoc,
    ApiReference,
    DependencyReport,
    Adr,
    Index,
}

/// Main documentation generator.
pub struct DocGenerator {
    config: DocGeneratorConfig,
}

impl DocGenerator {
    /// Create a new documentation generator.
    pub fn new(config: DocGeneratorConfig) -> Self {
        Self { config }
    }

    /// Generate all documentation from a snapshot.
    pub async fn generate(&self, snapshot: &CodebaseSnapshot) -> Result<GeneratedDocs> {
        let start = std::time::Instant::now();

        // Create output directory
        fs::create_dir_all(&self.config.output_dir)?;

        let mut files = Vec::new();
        let project_name = self.config.project_name.clone()
            .unwrap_or_else(|| snapshot.metadata.project_name.clone());

        info!(output_dir = ?self.config.output_dir, "Generating documentation");

        // 1. Generate index/README
        let index_file = self.generate_index(snapshot, &project_name)?;
        files.push(index_file);

        // 2. Generate architecture overview
        let arch_file = self.generate_architecture_overview(snapshot, &project_name).await?;
        files.push(arch_file);

        // 3. Generate module documentation
        let modules_file = self.generate_modules_doc(snapshot)?;
        files.push(modules_file);

        // 4. Generate dependency report
        let deps_file = self.generate_dependency_report(snapshot)?;
        files.push(deps_file);

        // 5. Generate API reference (if services found)
        if self.config.generate_api_ref && !snapshot.services.is_empty() {
            let api_file = self.generate_api_reference(snapshot)?;
            files.push(api_file);
        }

        // 6. Generate ADRs
        if self.config.generate_adrs {
            let adr_dir = self.config.output_dir.join("adrs");
            fs::create_dir_all(&adr_dir)?;
            
            let adr_files = self.generate_adrs(snapshot, &adr_dir).await?;
            files.extend(adr_files);
        }

        let generation_time_ms = start.elapsed().as_millis() as u64;

        info!(
            file_count = files.len(),
            generation_time_ms,
            "Documentation generation complete"
        );

        Ok(GeneratedDocs {
            files,
            output_dir: self.config.output_dir.clone(),
            generation_time_ms,
        })
    }

    /// Generate the index/README file.
    fn generate_index(&self, snapshot: &CodebaseSnapshot, project_name: &str) -> Result<GeneratedFile> {
        let mut md = MarkdownBuilder::new();

        md.h1(&format!("{} Documentation", project_name));
        md.paragraph("Auto-generated documentation for the codebase.");
        md.newline();

        md.h2("Quick Links");
        md.bullet_list(&[
            "[Architecture Overview](./ARCHITECTURE_OVERVIEW.md)",
            "[Modules & Services](./MODULES.md)",
            "[Dependencies & Risks](./DEPENDENCIES.md)",
            "[API Reference](./API_REFERENCE.md)",
            "[Architecture Decision Records](./adrs/)",
        ]);

        md.h2("Statistics");
        md.table(
            &["Metric", "Value"],
            &[
                vec!["Files", &snapshot.statistics.total_files.to_string()],
                vec!["Modules", &snapshot.statistics.total_modules.to_string()],
                vec!["Functions", &snapshot.statistics.total_functions.to_string()],
                vec!["Types", &snapshot.statistics.total_types.to_string()],
                vec!["Lines of Code", &snapshot.statistics.total_lines.to_string()],
            ],
        );

        md.h2("Languages");
        if !snapshot.statistics.languages.is_empty() {
            let mut rows: Vec<Vec<String>> = Vec::new();
            for lang in &snapshot.statistics.languages {
                rows.push(vec![lang.language.clone(), lang.file_count.to_string()]);
            }
            let rows_refs: Vec<Vec<&str>> = rows.iter()
                .map(|row| row.iter().map(|s| s.as_str()).collect())
                .collect();
            md.table(&["Language", "Files"], &rows_refs);
        }

        md.hr();
        md.paragraph(&format!(
            "*Generated on {} by [Riwaq Arch](https://github.com/YASSERRMD/riwaq-arch)*",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        self.write_file("README.md", &md.build(), DocType::Index)
    }

    /// Generate architecture overview.
    async fn generate_architecture_overview(
        &self,
        snapshot: &CodebaseSnapshot,
        project_name: &str,
    ) -> Result<GeneratedFile> {
        let mut md = MarkdownBuilder::new();

        md.h1(&format!("{} - Architecture Overview", project_name));

        // High-level summary
        md.h2("System Overview");
        md.paragraph(&format!(
            "This document provides an architectural overview of the {} codebase, \
            consisting of {} modules across {} files with {} total lines of code.",
            project_name,
            snapshot.statistics.total_modules,
            snapshot.statistics.total_files,
            snapshot.statistics.total_lines,
        ));

        // Architecture diagram - rendered as SVG image
        if self.config.include_diagrams {
            md.h2("Architecture Diagram");
            let mermaid_code = mermaid::generate_architecture_diagram(snapshot);
            if let Some(svg_path) = self.render_diagram_to_svg(&mermaid_code, "architecture_diagram") {
                md.paragraph(&format!("![Architecture Diagram]({})", svg_path));
            }
        }

        // Services and entry points
        if !snapshot.services.is_empty() {
            md.h2("Services & Entry Points");
            
            let mut rows: Vec<Vec<String>> = Vec::new();
            for service in &snapshot.services {
                rows.push(vec![
                    service.name.clone(),
                    format!("{:?}", service.kind),
                    service.entry_file.to_string_lossy().to_string(),
                ]);
            }
            let rows_refs: Vec<Vec<&str>> = rows.iter()
                .map(|row| row.iter().map(|s| s.as_str()).collect())
                .collect();
            md.table(&["Service", "Type", "Entry File"], &rows_refs);
        }

        // Module overview
        md.h2("Modules");
        for module in &snapshot.modules {
            md.h3(&module.name);
            md.paragraph(&format!("**Path**: `{}`", module.path));
            md.paragraph(&format!("**Files**: {}", module.files.len()));
            
            if !module.public_functions.is_empty() {
                let funcs: Vec<String> = module.public_functions.iter()
                    .take(10)
                    .map(|f| format!("`{}`", f))
                    .collect();
                md.paragraph(&format!("**Public Functions**: {}", funcs.join(", ")));
                if module.public_functions.len() > 10 {
                    md.paragraph(&format!("... and {} more", module.public_functions.len() - 10));
                }
            }
            md.newline();
        }

        // Dependency diagram - rendered as SVG image
        if self.config.include_diagrams && !snapshot.dependency_graph.edges.is_empty() {
            md.h2("Module Dependencies");
            let mermaid_code = mermaid::generate_dependency_diagram(&snapshot.dependency_graph);
            if let Some(svg_path) = self.render_diagram_to_svg(&mermaid_code, "dependency_diagram") {
                md.paragraph(&format!("![Dependency Diagram]({})", svg_path));
            }
        }

        // Circular dependencies warning
        if !snapshot.dependency_graph.circular_deps.is_empty() {
            md.h2("⚠️ Circular Dependencies");
            md.admonition(
                super::markdown::AdmonitionKind::Warning,
                "The following circular dependencies were detected:",
            );
            
            for cycle in &snapshot.dependency_graph.circular_deps {
                md.paragraph(&format!("- {}", cycle.join(" → ")));
            }
        }

        self.write_file("ARCHITECTURE_OVERVIEW.md", &md.build(), DocType::ArchitectureOverview)
    }

    /// Generate modules documentation.
    fn generate_modules_doc(&self, snapshot: &CodebaseSnapshot) -> Result<GeneratedFile> {
        let mut md = MarkdownBuilder::new();

        md.h1("Modules & Services");
        md.paragraph("Detailed documentation for each module in the codebase.");

        for module in &snapshot.modules {
            md.h2(&module.name);
            md.paragraph(&format!("**Path**: `{}`", module.path));

            // Files in module
            md.h3("Files");
            let file_list: Vec<&str> = module.files.iter()
                .map(|f| f.to_str().unwrap_or(""))
                .take(20)
                .collect();
            md.bullet_list(&file_list);
            if module.files.len() > 20 {
                md.paragraph(&format!("... and {} more files", module.files.len() - 20));
            }

            // Public API
            if !module.public_functions.is_empty() || !module.public_types.is_empty() {
                md.h3("Public API");
                
                if !module.public_types.is_empty() {
                    md.h4("Types");
                    let types: Vec<&str> = module.public_types.iter()
                        .map(|s| s.as_str())
                        .collect();
                    md.bullet_list(&types);
                }

                if !module.public_functions.is_empty() {
                    md.h4("Functions");
                    let funcs: Vec<&str> = module.public_functions.iter()
                        .map(|s| s.as_str())
                        .collect();
                    md.bullet_list(&funcs);
                }
            }

            // Dependencies
            if !module.dependencies.is_empty() {
                md.h3("Dependencies");
                let deps: Vec<&str> = module.dependencies.iter()
                    .map(|d| d.target.as_str())
                    .collect();
                md.bullet_list(&deps);
            }

            // Metrics
            md.h3("Metrics");
            md.table(
                &["Metric", "Value"],
                &[
                    vec!["Files", &module.metrics.file_count.to_string()],
                    vec!["Lines of Code", &module.metrics.lines_of_code.to_string()],
                    vec!["Functions", &module.metrics.function_count.to_string()],
                    vec!["Types", &module.metrics.type_count.to_string()],
                    vec!["Coupling Score", &format!("{:.2}", module.metrics.coupling_score)],
                    vec!["Cohesion Score", &format!("{:.2}", module.metrics.cohesion_score)],
                ],
            );

            md.hr();
        }

        self.write_file("MODULES.md", &md.build(), DocType::ModuleDoc)
    }

    /// Generate dependency report.
    fn generate_dependency_report(&self, snapshot: &CodebaseSnapshot) -> Result<GeneratedFile> {
        let mut md = MarkdownBuilder::new();

        md.h1("Dependencies & Risk Analysis");

        // Language breakdown
        md.h2("Technology Stack");
        let mut lang_rows: Vec<Vec<String>> = Vec::new();
        for lang in &snapshot.statistics.languages {
            lang_rows.push(vec![
                lang.language.clone(),
                lang.file_count.to_string(),
                lang.line_count.to_string(),
                format!("{:.1}%", (lang.line_count as f32 / snapshot.statistics.total_lines.max(1) as f32) * 100.0),
            ]);
        }
        let lang_rows_refs: Vec<Vec<&str>> = lang_rows.iter()
            .map(|row| row.iter().map(|s| s.as_str()).collect())
            .collect();
        md.table(&["Language", "Files", "Lines", "Percentage"], &lang_rows_refs);

        // Dependency graph
        if self.config.include_diagrams && !snapshot.dependency_graph.edges.is_empty() {
            md.h2("Dependency Graph");
            md.raw(&mermaid::generate_dependency_diagram(&snapshot.dependency_graph));
        }

        // Circular dependencies
        md.h2("Circular Dependencies");
        if snapshot.dependency_graph.circular_deps.is_empty() {
            md.paragraph("✅ No circular dependencies detected.");
        } else {
            md.admonition(
                super::markdown::AdmonitionKind::Warning,
                &format!("{} circular dependency chain(s) detected", snapshot.dependency_graph.circular_deps.len()),
            );
            
            for (i, cycle) in snapshot.dependency_graph.circular_deps.iter().enumerate() {
                md.h3(&format!("Cycle {}", i + 1));
                md.paragraph(&cycle.join(" → "));
            }
        }

        // Git insights
        if let Some(repo) = &snapshot.git_insights.repository {
            md.h2("Repository Insights");
            md.table(
                &["Metric", "Value"],
                &[
                    vec!["Repository", &repo.name],
                    vec!["Total Commits", &repo.total_commits.to_string()],
                    vec!["Default Branch", &repo.default_branch],
                ],
            );
        }

        // Domains
        if !snapshot.git_insights.domains.is_empty() {
            md.h2("Detected Domains");
            md.paragraph("Domains inferred from commit messages and file groupings:");
            
            let mut domain_rows: Vec<Vec<String>> = Vec::new();
            for domain in &snapshot.git_insights.domains {
                domain_rows.push(vec![
                    domain.name.clone(),
                    domain.files.len().to_string(),
                    format!("{:.0}%", domain.activity_level * 100.0),
                ]);
            }
            let domain_rows_refs: Vec<Vec<&str>> = domain_rows.iter()
                .map(|row| row.iter().map(|s| s.as_str()).collect())
                .collect();
            md.table(&["Domain", "Files", "Activity"], &domain_rows_refs);
        }

        self.write_file("DEPENDENCIES.md", &md.build(), DocType::DependencyReport)
    }

    /// Generate API reference.
    fn generate_api_reference(&self, snapshot: &CodebaseSnapshot) -> Result<GeneratedFile> {
        let mut md = MarkdownBuilder::new();

        md.h1("API Reference");
        md.paragraph("Documentation for HTTP, gRPC, and other service endpoints.");

        for service in &snapshot.services {
            md.h2(&format!("{} ({:?})", service.name, service.kind));
            md.paragraph(&format!("**Entry File**: `{}`", service.entry_file.display()));

            if !service.endpoints.is_empty() {
                md.h3("Endpoints");
                let endpoints: Vec<&str> = service.endpoints.iter()
                    .map(|e| e.as_str())
                    .collect();
                md.bullet_list(&endpoints);
            } else {
                md.paragraph("*Endpoints will be documented when route analysis is implemented.*");
            }

            md.hr();
        }

        self.write_file("API_REFERENCE.md", &md.build(), DocType::ApiReference)
    }

    /// Generate ADRs.
    async fn generate_adrs(
        &self,
        snapshot: &CodebaseSnapshot,
        output_dir: &Path,
    ) -> Result<Vec<GeneratedFile>> {
        let mut files = Vec::new();

        // Generate ADR index
        let mut md = MarkdownBuilder::new();
        md.h1("Architecture Decision Records");
        md.paragraph("This directory contains the Architecture Decision Records (ADRs) for this project.");
        md.newline();
        md.h2("List of ADRs");

        // Generate a basic ADR based on detected patterns
        let adr_content = self.generate_adr_content(snapshot);
        let adr_path = output_dir.join("ADR-001-inferred-architecture.md");
        fs::write(&adr_path, &adr_content)?;
        
        let size = fs::metadata(&adr_path)?.len();
        files.push(GeneratedFile {
            path: PathBuf::from("adrs/ADR-001-inferred-architecture.md"),
            absolute_path: adr_path,
            size,
            doc_type: DocType::Adr,
        });

        md.bullet_list(&["[ADR-001: Inferred Architecture](./ADR-001-inferred-architecture.md)"]);

        // Write index
        let index_path = output_dir.join("README.md");
        fs::write(&index_path, md.build())?;
        
        let index_size = fs::metadata(&index_path)?.len();
        files.push(GeneratedFile {
            path: PathBuf::from("adrs/README.md"),
            absolute_path: index_path,
            size: index_size,
            doc_type: DocType::Index,
        });

        Ok(files)
    }

    /// Generate ADR content for inferred architecture.
    fn generate_adr_content(&self, snapshot: &CodebaseSnapshot) -> String {
        let mut md = MarkdownBuilder::new();

        md.h1("ADR-001: Inferred Architecture");
        md.newline();

        md.h2("Status");
        md.paragraph("Proposed (Auto-generated)");

        md.h2("Context");
        md.paragraph(&format!(
            "This ADR documents the architecture inferred from static analysis of the codebase. \
            The project consists of {} modules with {} total files.",
            snapshot.modules.len(),
            snapshot.statistics.total_files,
        ));

        md.h2("Decision");
        
        // Infer architecture style
        let arch_style = if snapshot.services.len() > 3 {
            "Microservices or service-oriented architecture"
        } else if snapshot.modules.len() > 10 {
            "Modular monolith architecture"
        } else {
            "Simple modular architecture"
        };
        
        md.paragraph(&format!("The codebase follows a **{}** pattern.", arch_style));

        if !snapshot.modules.is_empty() {
            md.h3("Key Modules");
            let modules: Vec<&str> = snapshot.modules.iter()
                .take(5)
                .map(|m| m.name.as_str())
                .collect();
            md.bullet_list(&modules);
        }

        md.h2("Consequences");
        md.paragraph("**Positive:**");
        md.bullet_list(&[
            "Clear separation of concerns through module boundaries",
            "Enables independent development of modules",
        ]);

        if !snapshot.dependency_graph.circular_deps.is_empty() {
            md.paragraph("**Negative:**");
            md.bullet_list(&[
                &format!("{} circular dependencies detected - may complicate refactoring", 
                    snapshot.dependency_graph.circular_deps.len()),
            ]);
        }

        md.build()
    }

    /// Write a file and return the GeneratedFile info.
    fn write_file(&self, name: &str, content: &str, doc_type: DocType) -> Result<GeneratedFile> {
        let path = self.config.output_dir.join(name);
        fs::write(&path, content)?;
        
        let size = fs::metadata(&path)?.len();
        
        Ok(GeneratedFile {
            path: PathBuf::from(name),
            absolute_path: path,
            size,
            doc_type,
        })
    }

    /// Render Mermaid diagram to SVG and save to diagrams directory.
    /// Returns the relative path to the SVG file for use in markdown.
    fn render_diagram_to_svg(&self, mermaid_code: &str, name: &str) -> Option<String> {
        // Create diagrams subdirectory
        let diagrams_dir = self.config.output_dir.join("diagrams");
        if let Err(e) = fs::create_dir_all(&diagrams_dir) {
            warn!(error = %e, "Failed to create diagrams directory");
            return None;
        }

        // Set up renderer config
        let config = DiagramRendererConfig {
            output_dir: diagrams_dir.clone(),
        };

        // Create renderer
        let renderer = match DiagramRenderer::new(config) {
            Ok(r) => r,
            Err(e) => {
                warn!(error = %e, "Failed to create diagram renderer");
                return None;
            }
        };

        // Render to SVG
        match renderer.render_to_svg(mermaid_code) {
            Ok(svg) => {
                let svg_filename = format!("{}.svg", name);
                let svg_path = diagrams_dir.join(&svg_filename);
                
                if let Err(e) = fs::write(&svg_path, &svg) {
                    warn!(error = %e, "Failed to write SVG file");
                    return None;
                }

                info!(path = %svg_path.display(), "Rendered diagram to SVG");
                
                // Return relative path for markdown
                Some(format!("diagrams/{}", svg_filename))
            }
            Err(e) => {
                warn!(error = %e, "Failed to render Mermaid to SVG");
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_generate_docs() {
        let dir = TempDir::new().unwrap();
        let config = DocGeneratorConfig::new(dir.path())
            .with_llm(false);
        
        let generator = DocGenerator::new(config);
        let snapshot = CodebaseSnapshot::new(PathBuf::from("."));
        
        let result = generator.generate(&snapshot).await.unwrap();
        
        assert!(!result.files.is_empty());
        assert!(dir.path().join("README.md").exists());
    }
}
