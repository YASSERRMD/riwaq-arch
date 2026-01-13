//! Agentic Document Generation using NAFS-4
//!
//! This module uses the NAFS-4 framework for multi-step, agentic document generation.
//! Each document type requires multiple LLM calls to generate comprehensive content.
//! Uses 2 parallel LLM calls for faster generation.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{info, warn, error};
use nafs_llm::{LLMProvider, ChatMessage, ChatConfig, ChatResponse};
use nafs_core::{Agent, Goal, Result as NafsResult};
use crate::models::snapshot::CodebaseSnapshot;
use crate::errors::Result;
use std::fs;
use tokio::sync::Semaphore;

/// Maximum concurrent LLM calls
const MAX_CONCURRENT_LLM_CALLS: usize = 2;

/// Document types that can be generated
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentType {
    SRS,              // Software Requirements Specification
    UserStories,      // User Stories
    BRD,              // Business Requirements Document
    ArchitectureDiagram,
    CodeDocs,         // Code Documentation
    APISpecs,         // API Specifications
    ReleaseNotes,
    UserGuides,
    Runbook,          // Operations Runbook
}

impl DocumentType {
    pub fn filename(&self) -> &'static str {
        match self {
            Self::SRS => "SOFTWARE_REQUIREMENTS.md",
            Self::UserStories => "USER_STORIES.md",
            Self::BRD => "BUSINESS_REQUIREMENTS.md",
            Self::ArchitectureDiagram => "ARCHITECTURE_DIAGRAMS.md",
            Self::CodeDocs => "CODE_DOCUMENTATION.md",
            Self::APISpecs => "API_SPECIFICATIONS.md",
            Self::ReleaseNotes => "RELEASE_NOTES.md",
            Self::UserGuides => "USER_GUIDES.md",
            Self::Runbook => "OPERATIONS_RUNBOOK.md",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::SRS => "Software Requirements Specification",
            Self::UserStories => "User Stories",
            Self::BRD => "Business Requirements Document",
            Self::ArchitectureDiagram => "Architecture Diagrams",
            Self::CodeDocs => "Code Documentation",
            Self::APISpecs => "API Specifications",
            Self::ReleaseNotes => "Release Notes",
            Self::UserGuides => "User Guides",
            Self::Runbook => "Operations Runbook",
        }
    }

    pub fn all() -> Vec<DocumentType> {
        vec![
            Self::SRS,
            Self::UserStories,
            Self::BRD,
            Self::ArchitectureDiagram,
            Self::CodeDocs,
            Self::APISpecs,
            Self::ReleaseNotes,
            Self::UserGuides,
            Self::Runbook,
        ]
    }
}

/// Agentic Document Generator using NAFS-4
pub struct AgenticDocGenerator {
    llm_provider: Arc<dyn LLMProvider>,
    output_dir: PathBuf,
    project_name: String,
    semaphore: Arc<Semaphore>,
}

impl AgenticDocGenerator {
    /// Create a new agentic document generator
    pub fn new(
        llm_provider: Arc<dyn LLMProvider>,
        output_dir: PathBuf,
        project_name: String,
    ) -> Self {
        Self {
            llm_provider,
            output_dir,
            project_name,
            semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_LLM_CALLS)),
        }
    }

    /// Generate all documents with 2 parallel document generation
    pub async fn generate_all(&self, snapshot: &CodebaseSnapshot) -> Result<Vec<PathBuf>> {
        info!("Starting agentic document generation for {} documents (2 parallel)", DocumentType::all().len());
        
        let context = Arc::new(self.build_context_summary(snapshot));
        let doc_types = DocumentType::all();
        
        // Process documents in pairs (2 at a time)
        let mut generated_files = Vec::new();
        for chunk in doc_types.chunks(2) {
            let mut handles = Vec::new();
            
            for doc_type in chunk {
                let doc_type = *doc_type;
                let ctx = Arc::clone(&context);
                let llm = Arc::clone(&self.llm_provider);
                let output_dir = self.output_dir.clone();
                let project_name = self.project_name.clone();
                let sem = Arc::clone(&self.semaphore);
                
                let handle = tokio::spawn(async move {
                    let _permit = sem.acquire().await.unwrap();
                    info!("Generating: {}", doc_type.title());
                    
                    let gen = AgenticDocGenerator {
                        llm_provider: llm,
                        output_dir: output_dir.clone(),
                        project_name: project_name.clone(),
                        semaphore: Arc::new(Semaphore::new(2)),
                    };
                    
                    let content = match doc_type {
                        DocumentType::SRS => gen.generate_srs_concurrent(&ctx).await,
                        DocumentType::UserStories => gen.generate_user_stories_concurrent(&ctx).await,
                        DocumentType::BRD => gen.generate_brd_concurrent(&ctx).await,
                        DocumentType::ArchitectureDiagram => gen.generate_architecture_concurrent(&ctx).await,
                        DocumentType::CodeDocs => gen.generate_code_docs_concurrent(&ctx).await,
                        DocumentType::APISpecs => gen.generate_api_specs_concurrent(&ctx).await,
                        DocumentType::ReleaseNotes => gen.generate_release_notes_concurrent(&ctx).await,
                        DocumentType::UserGuides => gen.generate_user_guides_concurrent(&ctx).await,
                        DocumentType::Runbook => gen.generate_runbook_concurrent(&ctx).await,
                    };
                    
                    match content {
                        Ok(content) => {
                            let path = output_dir.join(doc_type.filename());
                            if let Err(e) = fs::write(&path, &content) {
                                error!("Failed to write {}: {}", doc_type.filename(), e);
                                None
                            } else {
                                info!("Successfully generated: {}", path.display());
                                Some(path)
                            }
                        }
                        Err(e) => {
                            error!("Failed to generate {}: {}", doc_type.title(), e);
                            None
                        }
                    }
                });
                handles.push(handle);
            }
            
            // Wait for this batch to complete
            for handle in handles {
                if let Ok(Some(path)) = handle.await {
                    generated_files.push(path);
                }
            }
        }

        Ok(generated_files)
    }

    /// Build context summary for LLM
    fn build_context_summary(&self, snapshot: &CodebaseSnapshot) -> String {
        let stats = &snapshot.statistics;
        let mut summary = format!(
            "Project: {}\nFiles: {}\nModules: {}\nFunctions: {}\nTypes: {}\nLines: {}\n\n",
            self.project_name,
            stats.total_files,
            stats.total_modules,
            stats.total_functions,
            stats.total_types,
            stats.total_lines
        );

        summary.push_str("## Languages\n");
        for lang in &stats.languages {
            summary.push_str(&format!("- {}: {} files, {} lines\n", lang.language, lang.file_count, lang.line_count));
        }

        summary.push_str("\n## Modules\n");
        for module in snapshot.modules.iter().take(20) {
            summary.push_str(&format!("- **{}** (`{}`): {} files\n", module.name, module.path, module.files.len()));
        }

        summary.push_str("\n## Services\n");
        for service in &snapshot.services {
            summary.push_str(&format!("- **{}** ({:?}): Entry: {}\n", service.name, service.kind, service.entry_file.display()));
        }

        summary
    }

    /// SRS generation with 2 parallel sections
    async fn generate_srs_concurrent(&self, context: &str) -> Result<String> {
        let mut content = format!("# Software Requirements Specification\n\n## {}\n\n", self.project_name);

        // Batch 1: Executive Summary + Functional Requirements
        let (summary, functional) = tokio::join!(
            self.llm_call("Based on this codebase analysis, write an executive summary for an SRS document. Write 2-3 paragraphs summarizing the software's purpose and scope.", context),
            self.llm_call("Based on this codebase, list the functional requirements. Format as numbered requirements with IDs like FR-001. Provide at least 10 functional requirements.", context)
        );
        content.push_str("## 1. Executive Summary\n\n");
        content.push_str(&summary.unwrap_or_default());
        content.push_str("\n\n## 2. Functional Requirements\n\n");
        content.push_str(&functional.unwrap_or_default());
        content.push_str("\n\n");

        // Batch 2: Non-Functional + System Interfaces
        let (nonfunctional, interfaces) = tokio::join!(
            self.llm_call("Based on this codebase, list non-functional requirements (performance, security, scalability, reliability). Format as NFR-001, NFR-002, etc. Provide at least 8 non-functional requirements.", context),
            self.llm_call("Describe the system interfaces (APIs, external integrations, user interfaces) based on this codebase. Provide detailed interface descriptions.", context)
        );
        content.push_str("## 3. Non-Functional Requirements\n\n");
        content.push_str(&nonfunctional.unwrap_or_default());
        content.push_str("\n\n## 4. System Interfaces\n\n");
        content.push_str(&interfaces.unwrap_or_default());
        content.push_str("\n\n");

        // Batch 3: Data Requirements + Constraints
        let (data, constraints) = tokio::join!(
            self.llm_call("Describe the data requirements, data models, and data flow based on this codebase. Include data entities, relationships, and storage requirements.", context),
            self.llm_call("List the constraints, assumptions, and dependencies for this software system. Provide concrete constraints and assumptions.", context)
        );
        content.push_str("## 5. Data Requirements\n\n");
        content.push_str(&data.unwrap_or_default());
        content.push_str("\n\n## 6. Constraints and Assumptions\n\n");
        content.push_str(&constraints.unwrap_or_default());
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// User Stories with 2 parallel sections
    async fn generate_user_stories_concurrent(&self, context: &str) -> Result<String> {
        let mut content = format!("# User Stories\n\n## {}\n\n", self.project_name);

        // Batch 1: Personas + Epics
        let (personas, epics) = tokio::join!(
            self.llm_call("Based on this codebase, identify the user personas/actors who would use this system. Provide 3-5 personas with descriptions.", context),
            self.llm_call("Based on this codebase, create epic-level user stories. Format: As a [persona], I want [goal] so that [benefit]. Create 5-8 epics.", context)
        );
        content.push_str("## User Personas\n\n");
        content.push_str(&personas.unwrap_or_default());
        content.push_str("\n\n## Epic Stories\n\n");
        content.push_str(&epics.unwrap_or_default());
        content.push_str("\n\n");

        // Batch 2: Detailed Stories + Story Map
        let (stories, map) = tokio::join!(
            self.llm_call("Create detailed user stories with acceptance criteria. Format each story with ID (US-001), story statement, and acceptance criteria. Create 10-15 stories.", context),
            self.llm_call("Create a story map organizing the user stories by user journey phases. Show the flow from discovery to completion.", context)
        );
        content.push_str("## Detailed User Stories\n\n");
        content.push_str(&stories.unwrap_or_default());
        content.push_str("\n\n## Story Map\n\n");
        content.push_str(&map.unwrap_or_default());
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// BRD with 2 parallel sections
    async fn generate_brd_concurrent(&self, context: &str) -> Result<String> {
        let mut content = format!("# Business Requirements Document\n\n## {}\n\n", self.project_name);

        // Batch 1: Overview + Objectives
        let (overview, objectives) = tokio::join!(
            self.llm_call("Write a business overview section for this software project. Include business context, objectives, and value proposition.", context),
            self.llm_call("Define SMART business objectives for this project. Include measurable goals and success criteria.", context)
        );
        content.push_str("## 1. Business Overview\n\n");
        content.push_str(&overview.unwrap_or_default());
        content.push_str("\n\n## 2. Business Objectives\n\n");
        content.push_str(&objectives.unwrap_or_default());
        content.push_str("\n\n");

        // Batch 2: Stakeholders + Requirements
        let (stakeholders, requirements) = tokio::join!(
            self.llm_call("Identify stakeholders for this project. Include their interests, influence, and engagement strategy.", context),
            self.llm_call("List business requirements with IDs (BR-001). Include priority (High/Medium/Low) and rationale.", context)
        );
        content.push_str("## 3. Stakeholder Analysis\n\n");
        content.push_str(&stakeholders.unwrap_or_default());
        content.push_str("\n\n## 4. Business Requirements\n\n");
        content.push_str(&requirements.unwrap_or_default());
        content.push_str("\n\n");

        // Batch 3: ROI (single)
        let roi = self.llm_call("Create a business case section including expected benefits, costs considerations, and ROI analysis framework.", context).await?;
        content.push_str("## 5. Business Case & ROI\n\n");
        content.push_str(&roi);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Architecture docs with 2 parallel sections
    async fn generate_architecture_concurrent(&self, context: &str) -> Result<String> {
        let mut content = format!("# Architecture Documentation\n\n## {}\n\n", self.project_name);

        // Batch 1: Overview + Components
        let (overview, components) = tokio::join!(
            self.llm_call("Write a system architecture overview. Describe the high-level design, architectural style, and key components.", context),
            self.llm_call("Describe the component architecture. List each major component, its responsibility, and interfaces.", context)
        );
        content.push_str("## 1. System Overview\n\n");
        content.push_str(&overview.unwrap_or_default());
        content.push_str("\n\n## 2. Component Architecture\n\n");
        content.push_str(&components.unwrap_or_default());
        content.push_str("\n\n");

        // Batch 2: Data + Integration
        let (data, integration) = tokio::join!(
            self.llm_call("Describe the data architecture including data stores, data flow, and data models.", context),
            self.llm_call("Describe how components integrate with each other and external systems. Include APIs, messaging, etc.", context)
        );
        content.push_str("## 3. Data Architecture\n\n");
        content.push_str(&data.unwrap_or_default());
        content.push_str("\n\n## 4. Integration Architecture\n\n");
        content.push_str(&integration.unwrap_or_default());
        content.push_str("\n\n");

        // Batch 3: Deployment (single)
        let deployment = self.llm_call("Describe the deployment architecture including infrastructure, scaling, and availability.", context).await?;
        content.push_str("## 5. Deployment Architecture\n\n");
        content.push_str(&deployment);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Code docs with 2 parallel sections
    async fn generate_code_docs_concurrent(&self, context: &str) -> Result<String> {
        let mut content = format!("# Code Documentation\n\n## {}\n\n", self.project_name);

        // Batch 1: Getting Started + Modules
        let (getting_started, modules) = tokio::join!(
            self.llm_call("Write a getting started guide for developers. Include setup, prerequisites, and first steps.", context),
            self.llm_call("Document each major module. Include purpose, public interfaces, and usage examples.", context)
        );
        content.push_str("## Getting Started\n\n");
        content.push_str(&getting_started.unwrap_or_default());
        content.push_str("\n\n## Module Documentation\n\n");
        content.push_str(&modules.unwrap_or_default());
        content.push_str("\n\n");

        // Batch 2: Patterns + Testing
        let (patterns, testing) = tokio::join!(
            self.llm_call("Document the coding patterns and conventions used in this codebase. Include examples.", context),
            self.llm_call("Write a testing guide. Include how to run tests, write new tests, and testing best practices.", context)
        );
        content.push_str("## Coding Patterns & Conventions\n\n");
        content.push_str(&patterns.unwrap_or_default());
        content.push_str("\n\n## Testing Guide\n\n");
        content.push_str(&testing.unwrap_or_default());
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// API specs with 2 parallel sections
    async fn generate_api_specs_concurrent(&self, context: &str) -> Result<String> {
        let mut content = format!("# API Specifications\n\n## {}\n\n", self.project_name);

        // Batch 1: Overview + REST
        let (overview, rest) = tokio::join!(
            self.llm_call("Write an API overview including authentication, base URLs, versioning, and general conventions.", context),
            self.llm_call("Document REST API endpoints. Include method, path, request/response schemas, and examples.", context)
        );
        content.push_str("## API Overview\n\n");
        content.push_str(&overview.unwrap_or_default());
        content.push_str("\n\n## REST API Endpoints\n\n");
        content.push_str(&rest.unwrap_or_default());
        content.push_str("\n\n");

        // Batch 2: Other APIs + Errors
        let (other, errors) = tokio::join!(
            self.llm_call("Document any gRPC services or GraphQL schemas if present in the codebase. If not present, note that.", context),
            self.llm_call("Document API error handling including error codes, error response format, and troubleshooting.", context)
        );
        content.push_str("## Other API Interfaces\n\n");
        content.push_str(&other.unwrap_or_default());
        content.push_str("\n\n## Error Handling\n\n");
        content.push_str(&errors.unwrap_or_default());
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Release notes with parallel sections
    async fn generate_release_notes_concurrent(&self, context: &str) -> Result<String> {
        let mut content = format!("# Release Notes\n\n## {}\n\n", self.project_name);

        // Batch 1: Current + Breaking (2 parallel)
        let (current, breaking) = tokio::join!(
            self.llm_call("Based on the codebase, write release notes for what appears to be the current version. Include new features, improvements, and bug fixes.", context),
            self.llm_call("Identify any potential breaking changes based on the codebase architecture. Include migration guides.", context)
        );
        content.push_str("## Current Release\n\n");
        content.push_str(&current.unwrap_or_default());
        content.push_str("\n\n## Breaking Changes\n\n");
        content.push_str(&breaking.unwrap_or_default());
        content.push_str("\n\n");

        // Single: Roadmap
        let future = self.llm_call("Suggest deprecations and future improvements based on the codebase analysis.", context).await?;
        content.push_str("## Deprecations & Future Roadmap\n\n");
        content.push_str(&future);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// User guides with 2 parallel sections
    async fn generate_user_guides_concurrent(&self, context: &str) -> Result<String> {
        let mut content = format!("# User Guides\n\n## {}\n\n", self.project_name);

        // Batch 1: Quick Start + Features
        let (quickstart, features) = tokio::join!(
            self.llm_call("Write a quick start guide for end users. Keep it simple and focused on getting started in 5 minutes.", context),
            self.llm_call("Write a comprehensive features guide explaining each major feature and how to use it.", context)
        );
        content.push_str("## Quick Start Guide\n\n");
        content.push_str(&quickstart.unwrap_or_default());
        content.push_str("\n\n## Features Guide\n\n");
        content.push_str(&features.unwrap_or_default());
        content.push_str("\n\n");

        // Batch 2: Troubleshooting + FAQ
        let (troubleshooting, faq) = tokio::join!(
            self.llm_call("Write a troubleshooting guide with common issues and their solutions.", context),
            self.llm_call("Write a FAQ section with 10-15 frequently asked questions and answers.", context)
        );
        content.push_str("## Troubleshooting\n\n");
        content.push_str(&troubleshooting.unwrap_or_default());
        content.push_str("\n\n## FAQ\n\n");
        content.push_str(&faq.unwrap_or_default());
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Runbook with 2 parallel sections
    async fn generate_runbook_concurrent(&self, context: &str) -> Result<String> {
        let mut content = format!("# Operations Runbook\n\n## {}\n\n", self.project_name);

        // Batch 1: Deployment + Monitoring
        let (deployment, monitoring) = tokio::join!(
            self.llm_call("Write deployment procedures including step-by-step deployment, rollback procedures, and verification steps.", context),
            self.llm_call("Write monitoring and alerting guidelines. Include key metrics, dashboards, and alert thresholds.", context)
        );
        content.push_str("## Deployment Procedures\n\n");
        content.push_str(&deployment.unwrap_or_default());
        content.push_str("\n\n## Monitoring & Alerting\n\n");
        content.push_str(&monitoring.unwrap_or_default());
        content.push_str("\n\n");

        // Batch 2: Incidents + Backup
        let (incidents, backup) = tokio::join!(
            self.llm_call("Write incident response procedures. Include severity levels, escalation paths, and remediation steps.", context),
            self.llm_call("Write backup and disaster recovery procedures. Include RTO, RPO, and recovery steps.", context)
        );
        content.push_str("## Incident Response\n\n");
        content.push_str(&incidents.unwrap_or_default());
        content.push_str("\n\n## Backup & Disaster Recovery\n\n");
        content.push_str(&backup.unwrap_or_default());
        content.push_str("\n\n");

        // Single: Maintenance
        let maintenance = self.llm_call("Document routine maintenance tasks including health checks, log rotation, and performance tuning.", context).await?;
        content.push_str("## Routine Maintenance\n\n");
        content.push_str(&maintenance);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Make an LLM call using NAFS-4
    async fn llm_call(&self, prompt: &str, context: &str) -> Result<String> {
        let full_prompt = format!("{}\n\nCodebase Context:\n{}", prompt, context);
        let messages = vec![
            ChatMessage::system("You are an expert technical writer creating professional software documentation. Be concise but comprehensive. Use proper markdown formatting."),
            ChatMessage::user(&full_prompt),
        ];

        let config = ChatConfig::default()
            .with_temperature(0.3)
            .with_max_tokens(2000);

        match self.llm_provider.chat(&messages, &config).await {
            Ok(response) => Ok(response.content),
            Err(e) => {
                warn!("LLM call failed: {:?}", e);
                Ok(format!("*Content generation failed: {:?}*", e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_types() {
        let all = DocumentType::all();
        assert_eq!(all.len(), 9);
        assert_eq!(DocumentType::SRS.filename(), "SOFTWARE_REQUIREMENTS.md");
    }
}
