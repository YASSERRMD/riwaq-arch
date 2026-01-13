//! Main documentation generator.
//!
//! This module orchestrates the generation of all documentation types
//! from a codebase snapshot, optionally using LLM for enhanced content.
//! 
//! All diagrams are rendered as SVG images, not as Mermaid text.

use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
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
    BusinessRequirements,
    SoftwareRequirements,
}

/// Main documentation generator.
pub struct DocGenerator {
    config: DocGeneratorConfig,
    llm_client: Box<dyn crate::llm::LLMClient>,
}

impl DocGenerator {
    /// Create a new documentation generator.
    /// Create a new documentation generator.
    pub fn new(config: DocGeneratorConfig) -> Self {
        let llm_client: Box<dyn crate::llm::LLMClient> = if config.use_llm {
            if let Some(llm_config) = config.llm_config.clone() {
                match crate::llm::HttpLlmClient::new(llm_config) {
                    Ok(client) => Box::new(client),
                    Err(e) => {
                        warn!("Failed to create LLM client: {}", e);
                        Box::new(crate::llm::client::MockLlmClient::new())
                    }
                }
            } else {
                 match crate::llm::HttpLlmClient::with_defaults() {
                    Ok(client) => Box::new(client),
                    Err(_) => Box::new(crate::llm::client::MockLlmClient::new()),
                 }
            }
        } else {
            Box::new(crate::llm::client::MockLlmClient::new())
        };
        
        Self { config, llm_client }
    }
    
    /// Create with an external LLM client (use when you have a configured client)
    pub fn with_client(config: DocGeneratorConfig, llm_client: Box<dyn crate::llm::LLMClient>) -> Self {
        Self { config, llm_client }
    }
    
    /// Generate a single document type
    pub async fn generate_single(&self, snapshot: &CodebaseSnapshot, doc_type: &str) -> Result<GeneratedFile> {
        // Create output directory
        fs::create_dir_all(&self.config.output_dir)?;
        
        let project_name = self.config.project_name.clone()
            .unwrap_or_else(|| snapshot.metadata.project_name.clone());
        
        match doc_type {
            "srs" => self.generate_srs(snapshot, &project_name).await,
            "brd" => self.generate_brd(snapshot, &project_name).await,
            "architecture" => self.generate_architecture_overview(snapshot, &project_name).await,
            "api_specs" => self.generate_api_reference(snapshot).await,
            "code_docs" | "modules" => self.generate_modules_doc(snapshot),
            "dependencies" => self.generate_dependency_report(snapshot),
            "index" | "readme" => self.generate_index(snapshot, &project_name),
            _ => Err(crate::errors::RiwaqError::Internal(
                format!("Unknown document type: {}. Use: srs, brd, architecture, api_specs, code_docs, dependencies, index", doc_type)
            ))
        }
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

        // 2. Generate architecture overview (Markdown)
        let arch_file = self.generate_architecture_overview(snapshot, &project_name).await?;
        files.push(arch_file);

        // 2b. Generate architecture overview (PRO HTML)
        if let Ok(html_file) = self.generate_architecture_html(snapshot, &project_name).await {
            files.push(html_file);
        }

        // 3. Generate module documentation
        let modules_file = self.generate_modules_doc(snapshot)?;
        files.push(modules_file);

        // 4. Generate dependency report
        let deps_file = self.generate_dependency_report(snapshot)?;
        files.push(deps_file);

        // 5. Generate API reference (if services found)
        if self.config.generate_api_ref && !snapshot.services.is_empty() {
            let api_file = self.generate_api_reference(snapshot).await?;
            files.push(api_file);
        }

        // 6. Generate ADRs
        if self.config.generate_adrs {
            let adr_dir = self.config.output_dir.join("adrs");
            fs::create_dir_all(&adr_dir)?;
            
            let adr_files = self.generate_adrs(snapshot, &adr_dir).await?;
            files.extend(adr_files);
        }

        // 7. Generate Reverse Engineering Docs (BRD & SRS)
        if self.config.use_llm {
            if let Ok(brd) = self.generate_brd(snapshot, &project_name).await {
                files.push(brd);
            }
            if let Ok(srs) = self.generate_srs(snapshot, &project_name).await {
                files.push(srs);
            }
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
            "[Business Requirements (BRD)](./BUSINESS_REQUIREMENTS.md)",
            "[Software Requirements (SRS)](./SOFTWARE_REQUIREMENTS.md)",
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
            let mermaid_code = mermaid::generate_dependency_diagram(&snapshot.dependency_graph);
            if let Some(svg_path) = self.render_diagram_to_svg(&mermaid_code, "full_dependency_diagram") {
                md.paragraph(&format!("![Dependency Graph]({})", svg_path));
            }
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

    /// Process embedded diagrams in content.
    fn process_embedded_diagrams(&self, content: &str, prefix: &str) -> String {
        let regex = Regex::new(r"(?s)```mermaid\s*(.*?)```").unwrap();
        
        regex.replace_all(content, |caps: &regex::Captures| {
            let mermaid_code = &caps[1];
            let id = uuid::Uuid::new_v4();
            let name = format!("{}_{}", prefix, id);
            
            if let Some(path) = self.render_diagram_to_svg(mermaid_code, &name) {
                format!("\n![Diagram]({})\n", path)
            } else {
                warn!("Failed to render embedded diagram {}", name);
                 format!("\n```mermaid\n{}\n```\n", mermaid_code)
            }
        }).to_string()
    }

    /// Generate BRD using LLM.
    async fn generate_brd(&self, snapshot: &CodebaseSnapshot, project_name: &str) -> Result<GeneratedFile> {
        info!("Generating Business Requirements Document...");
        let content = if self.config.use_llm {
            let context = crate::llm::prompts::truncate_context(&self.generate_context_summary(snapshot), 12000);
            let prompt = crate::llm::prompts::generate_brd_prompt(project_name, &context);
            match self.llm_client.complete(&prompt, Some(crate::llm::prompts::SYSTEM_PROMPT)).await {
                Ok(res) => {
                    info!("BRD LLM response received, processing diagrams...");
                    self.process_embedded_diagrams(&res.content, "brd")
                }
                Err(e) => {
                    warn!("BRD LLM generation failed: {}", e);
                    format!("# Business Requirements Document\n\n## {}\n\n(LLM generation failed: {})\n\n## Fallback Content\n\nThis document should contain the business requirements for the project.", project_name, e)
                }
            }
        } else {
            format!("# Business Requirements Document\n\n## {}\n\n(LLM is disabled. Enable LLM in settings to generate AI-powered BRD.)", project_name)
        };
        self.write_file("BUSINESS_REQUIREMENTS.md", &content, DocType::BusinessRequirements)
    }

    /// Generate SRS using LLM.
    async fn generate_srs(&self, snapshot: &CodebaseSnapshot, project_name: &str) -> Result<GeneratedFile> {
        info!("Generating Software Requirements Specification...");
        let content = if self.config.use_llm {
            let context = crate::llm::prompts::truncate_context(&self.generate_context_summary(snapshot), 12000);
            let prompt = crate::llm::prompts::generate_srs_prompt(project_name, &context);
            match self.llm_client.complete(&prompt, Some(crate::llm::prompts::SYSTEM_PROMPT)).await {
                Ok(res) => {
                    info!("SRS LLM response received, processing diagrams...");
                    self.process_embedded_diagrams(&res.content, "srs")
                }
                Err(e) => {
                    warn!("SRS LLM generation failed: {}", e);
                    format!("# Software Requirements Specification\n\n## {}\n\n(LLM generation failed: {})\n\n## Fallback Content\n\nThis document should contain the software requirements for the project.", project_name, e)
                }
            }
        } else {
            format!("# Software Requirements Specification\n\n## {}\n\n(LLM is disabled. Enable LLM in settings to generate AI-powered SRS.)", project_name)
        };
        self.write_file("SOFTWARE_REQUIREMENTS.md", &content, DocType::SoftwareRequirements)
    }
    
    async fn generate_api_reference(&self, snapshot: &CodebaseSnapshot) -> Result<GeneratedFile> {
        if self.config.use_llm {
             let mut endpoints_desc = String::new();
             for service in &snapshot.services {
                 endpoints_desc.push_str(&format!("Service: {} ({:?})\n", service.name, service.kind));
                 if !service.endpoints.is_empty() {
                      endpoints_desc.push_str("Detected Endpoints:\n");
                      for ep in &service.endpoints {
                          endpoints_desc.push_str(&format!("- {}\n", ep));
                      }
                 }
             }
             
             // Look for schema files
             for file in &snapshot.files {
                 if file.path.extension().map_or(false, |e| e == "graphql" || e == "gql") {
                      endpoints_desc.push_str(&format!("GraphQL Schema found in: {}\n", file.path.display()));
                 }
                 if file.path.extension().map_or(false, |e| e == "proto") {
                      endpoints_desc.push_str(&format!("gRPC Proto found in: {}\n", file.path.display()));
                 }
             }

             let prompt = crate::llm::prompts::api_documentation_prompt(&endpoints_desc);
             let content = match self.llm_client.complete(&prompt, Some(crate::llm::prompts::SYSTEM_PROMPT)).await {
                Ok(c) => c.content,
                Err(e) => {
                    warn!("LLM API Doc generation failed: {}", e);
                    "API Documentation generation failed. Please check logs.".to_string()
                }
             };
             self.write_file("API_REFERENCE.md", &content, DocType::ApiReference)
       } else {
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
    }

    fn generate_context_summary(&self, snapshot: &CodebaseSnapshot) -> String {
        let mut summary = String::new();
        summary.push_str(&format!("Project: {}\n", snapshot.metadata.project_name));
        summary.push_str(&format!("Files: {}\n", snapshot.statistics.total_files));
        summary.push_str(&format!("Modules: {}\n", snapshot.statistics.total_modules));
        
        summary.push_str("\n## Modules\n");
        for module in &snapshot.modules {
            summary.push_str(&format!("- **{}** (`{}`): {} files\n", module.name, module.path, module.files.len()));
        }
        
        summary.push_str("\n## Services\n");
        for service in &snapshot.services {
             summary.push_str(&format!("- **{}** ({:?}): Entry: {}\n", service.name, service.kind, service.entry_file.display()));
        }
        summary
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

    /// Generate HTML architecture overview with premium design.
    async fn generate_architecture_html(
        &self,
        snapshot: &CodebaseSnapshot,
        project_name: &str,
    ) -> Result<GeneratedFile> {
        let stats = &snapshot.statistics;
        
        // Generate Diagram SVG string
        let mermaid_code = mermaid::generate_architecture_diagram(snapshot);
        let diagram_svg = self.render_diagram_string(&mermaid_code).unwrap_or_else(|_| "<!-- Diagram failed to render -->".to_string());
        
        // Premium HTML Template
        let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Architecture Overview - {project_name}</title>
    <style>
        :root {{
            --bg-color: #0f172a;
            --text-color: #e2e8f0;
            --card-bg: rgba(30, 41, 59, 0.7);
            --card-border: rgba(148, 163, 184, 0.1);
            --accent: #6366f1;
            --accent-glow: rgba(99, 102, 241, 0.2);
            --secondary: #94a3b8;
        }}
        body {{
            font-family: 'Inter', system-ui, -apple-system, sans-serif;
            background-color: var(--bg-color);
            color: var(--text-color);
            line-height: 1.6;
            margin: 0;
            padding: 0;
            background-image: radial-gradient(circle at 50% 0%, #1e1b4b 0%, var(--bg-color) 40%);
            min-height: 100vh;
        }}
        .container {{
            max-width: 1400px;
            margin: 0 auto;
            padding: 40px 20px;
        }}
        header {{
            text-align: center;
            margin-bottom: 60px;
            padding-top: 20px;
        }}
        h1 {{
            font-size: 3.5rem;
            font-weight: 800;
            background: linear-gradient(to right, #818cf8, #c084fc);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            margin-bottom: 10px;
            letter-spacing: -1px;
        }}
        .subtitle {{
            color: var(--secondary);
            font-size: 1.2rem;
        }}
        .card {{
            background: var(--card-bg);
            border: 1px solid var(--card-border);
            border-radius: 16px;
            padding: 24px;
            backdrop-filter: blur(12px);
            margin-bottom: 30px;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
        }}
        .diagram-container {{
            background: #111;
            border-radius: 12px;
            border: 1px solid var(--card-border);
            height: 600px;
            overflow: hidden;
            position: relative;
            cursor: grab;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        .diagram-container:active {{
            cursor: grabbing;
        }}
        .diagram-container svg {{
            max-width: none;
            height: auto;
            transition: transform 0.1s ease-out;
            transform-origin: center;
        }}
        /* ... existing styles ... */
        .stats-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 40px;
        }}
        .stat-item {{
            text-align: center;
            padding: 24px;
            background: rgba(255, 255, 255, 0.03);
            border: 1px solid rgba(255, 255, 255, 0.05);
            border-radius: 16px;
            transition: all 0.3s ease;
        }}
        .stat-value {{
            display: block;
            font-size: 3rem;
            font-weight: 800;
            color: #fff;
            margin-bottom: 5px;
            line-height: 1;
        }}
        .stat-label {{
            color: var(--secondary);
            font-size: 0.875rem;
            text-transform: uppercase;
            letter-spacing: 1px;
            font-weight: 600;
        }}
        .controls {{
            position: absolute;
            bottom: 20px;
            right: 20px;
            display: flex;
            gap: 10px;
            z-index: 10;
        }}
        .btn {{
            background: rgba(255, 255, 255, 0.1);
            border: 1px solid rgba(255, 255, 255, 0.2);
            color: white;
            width: 36px;
            height: 36px;
            border-radius: 8px;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.2rem;
            transition: all 0.2s;
        }}
        .btn:hover {{
            background: rgba(255, 255, 255, 0.2);
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>{project_name}</h1>
            <div class="subtitle">Architecture Overview</div>
        </header>

        <!-- Stats Grid -->
        <div class="stats-grid">
            <div class="stat-item">
                <span class="stat-value">{modules}</span>
                <span class="stat-label">Modules</span>
            </div>
            <div class="stat-item">
                <span class="stat-value">{files}</span>
                <span class="stat-label">Files</span>
            </div>
             <div class="stat-item">
                <span class="stat-value">{services}</span>
                <span class="stat-label">Services</span>
            </div>
            <div class="stat-item">
                <span class="stat-value">{lines}</span>
                <span class="stat-label">Total Lines</span>
            </div>
        </div>

        <section class="card">
            <h2>🏗️ System Architecture</h2>
            <div class="diagram-container" id="diagramContainer">
                <div id="diagramContent">
                    {diagram_svg}
                </div>
                <div class="controls">
                    <button class="btn" onclick="zoomIn()">+</button>
                    <button class="btn" onclick="zoomOut()">-</button>
                    <button class="btn" onclick="resetZoom()">↺</button>
                </div>
            </div>
            <p style="text-align: center; color: var(--secondary); margin-top: 10px; font-size: 0.9rem;">
                Scroll to zoom • Drag to pan
            </p>
        </section>

        <section class="card">
            <h2>📦 Modules</h2>
            <div style="overflow-x: auto;">
                <table style="width: 100%; border-collapse: collapse; text-align: left;">
                    <thead>
                        <tr>
                            <th style="padding: 12px; border-bottom: 1px solid var(--card-border); color: var(--secondary);">Name</th>
                            <th style="padding: 12px; border-bottom: 1px solid var(--card-border); color: var(--secondary);">Path</th>
                            <th style="padding: 12px; border-bottom: 1px solid var(--card-border); color: var(--secondary);">Files</th>
                        </tr>
                    </thead>
                    <tbody>
                        {module_rows}
                    </tbody>
                </table>
            </div>
        </section>

        <div class="footer">
            Generated by Riwaq Arch • {date}
        </div>
    </div>

    <script>
        const container = document.getElementById('diagramContainer');
        const content = document.getElementById('diagramContent');
        let scale = 1;
        let pannedX = 0;
        let pannedY = 0;
        let isDragging = false;
        let startX, startY;

        function updateTransform() {{
            content.style.transform = `translate(${{pannedX}}px, ${{pannedY}}px) scale(${{scale}})`;
        }}

        function zoomIn() {{
            scale *= 1.2;
            updateTransform();
        }}

        function zoomOut() {{
            scale /= 1.2;
            updateTransform();
        }}

        function resetZoom() {{
            scale = 1;
            pannedX = 0;
            pannedY = 0;
            updateTransform();
        }}

        container.addEventListener('wheel', (e) => {{
            e.preventDefault();
            const delta = e.deltaY > 0 ? 0.9 : 1.1;
            scale *= delta;
            scale = Math.min(Math.max(0.1, scale), 5);
            updateTransform();
        }});

        container.addEventListener('mousedown', (e) => {{
            if (e.target.closest('.btn')) return;
            isDragging = true;
            startX = e.clientX - pannedX;
            startY = e.clientY - pannedY;
            container.style.cursor = 'grabbing';
        }});

        window.addEventListener('mouseup', () => {{
            isDragging = false;
            container.style.cursor = 'grab';
        }});

        window.addEventListener('mousemove', (e) => {{
            if (!isDragging) return;
            e.preventDefault();
            pannedX = e.clientX - startX;
            pannedY = e.clientY - startY;
            updateTransform();
        }});
        
        // Initial fit
        const svg = content.querySelector('svg');
        if (svg) {{
            svg.setAttribute('width', '100%');
            svg.setAttribute('height', '100%');
        }}
    </script>
</body>
</html>"#,
            project_name = project_name,
            modules = stats.total_modules,
            files = stats.total_files,
            services = snapshot.services.len(),
            lines = stats.total_lines,
            diagram_svg = diagram_svg,
            module_rows = snapshot.modules.iter().take(20).map(|m| format!(
                "<tr><td style='padding: 12px; border-bottom: 1px solid rgba(148, 163, 184, 0.1); font-weight: 500;'>{}</td><td style='padding: 12px; border-bottom: 1px solid rgba(148, 163, 184, 0.1); font-family: monospace; color: #94a3b8;'>{}</td><td style='padding: 12px; border-bottom: 1px solid rgba(148, 163, 184, 0.1);'>{}</td></tr>",
                m.name, m.path, m.files.len()
            )).collect::<Vec<_>>().join(""),
            date = chrono::Local::now().format("%Y-%m-%d %H:%M")
        );

        self.write_file("architecture_overview.html", &html, DocType::ArchitectureOverview)
    }

    /// Helper to render directly to SVG string
    fn render_diagram_string(&self, mermaid_code: &str) -> std::result::Result<String, String> {
        // Use a temp config for rendering
        let config = DiagramRendererConfig {
            output_dir: self.config.output_dir.clone(),
        };
        let renderer = DiagramRenderer::new(config)?;
        renderer.render_to_svg(mermaid_code)
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
