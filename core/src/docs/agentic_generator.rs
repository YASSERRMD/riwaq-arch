//! Agentic Document Generation using NAFS-4
//!
//! This module uses the NAFS-4 framework for multi-step, agentic document generation.
//! Each document type requires multiple LLM calls to generate comprehensive content.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{info, warn, error};
use nafs_llm::{LLMProvider, ChatMessage, ChatConfig, ChatResponse};
use nafs_core::{Agent, Goal, Result as NafsResult};
use crate::models::snapshot::CodebaseSnapshot;
use crate::errors::Result;
use std::fs;

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
        }
    }

    /// Generate all documents
    pub async fn generate_all(&self, snapshot: &CodebaseSnapshot) -> Result<Vec<PathBuf>> {
        info!("Starting agentic document generation for {} documents", DocumentType::all().len());
        
        let mut generated_files = Vec::new();
        let context = self.build_context_summary(snapshot);

        for doc_type in DocumentType::all() {
            info!("Generating: {}", doc_type.title());
            match self.generate_document(doc_type, snapshot, &context).await {
                Ok(path) => {
                    info!("Successfully generated: {}", path.display());
                    generated_files.push(path);
                }
                Err(e) => {
                    error!("Failed to generate {}: {}", doc_type.title(), e);
                }
            }
        }

        Ok(generated_files)
    }

    /// Generate a single document using multi-step agentic approach
    pub async fn generate_document(
        &self,
        doc_type: DocumentType,
        snapshot: &CodebaseSnapshot,
        context: &str,
    ) -> Result<PathBuf> {
        let content = match doc_type {
            DocumentType::SRS => self.generate_srs(snapshot, context).await?,
            DocumentType::UserStories => self.generate_user_stories(snapshot, context).await?,
            DocumentType::BRD => self.generate_brd(snapshot, context).await?,
            DocumentType::ArchitectureDiagram => self.generate_architecture_docs(snapshot, context).await?,
            DocumentType::CodeDocs => self.generate_code_docs(snapshot, context).await?,
            DocumentType::APISpecs => self.generate_api_specs(snapshot, context).await?,
            DocumentType::ReleaseNotes => self.generate_release_notes(snapshot, context).await?,
            DocumentType::UserGuides => self.generate_user_guides(snapshot, context).await?,
            DocumentType::Runbook => self.generate_runbook(snapshot, context).await?,
        };

        let path = self.output_dir.join(doc_type.filename());
        fs::write(&path, &content)?;
        Ok(path)
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

    /// Multi-step SRS generation
    async fn generate_srs(&self, snapshot: &CodebaseSnapshot, context: &str) -> Result<String> {
        let mut content = format!("# Software Requirements Specification\n\n## {}\n\n", self.project_name);

        // Step 1: Executive Summary
        info!("SRS Step 1/6: Executive Summary");
        let summary = self.llm_call(&format!(
            "Based on this codebase analysis, write an executive summary for an SRS document.\n\n{}\n\nWrite 2-3 paragraphs summarizing the software's purpose and scope.",
            context
        )).await?;
        content.push_str("## 1. Executive Summary\n\n");
        content.push_str(&summary);
        content.push_str("\n\n");

        // Step 2: Functional Requirements
        info!("SRS Step 2/6: Functional Requirements");
        let functional = self.llm_call(&format!(
            "Based on this codebase, list the functional requirements. Format as numbered requirements with IDs like FR-001.\n\n{}\n\nProvide at least 10 functional requirements.",
            context
        )).await?;
        content.push_str("## 2. Functional Requirements\n\n");
        content.push_str(&functional);
        content.push_str("\n\n");

        // Step 3: Non-Functional Requirements
        info!("SRS Step 3/6: Non-Functional Requirements");
        let nonfunctional = self.llm_call(&format!(
            "Based on this codebase, list non-functional requirements (performance, security, scalability, reliability). Format as NFR-001, NFR-002, etc.\n\n{}\n\nProvide at least 8 non-functional requirements.",
            context
        )).await?;
        content.push_str("## 3. Non-Functional Requirements\n\n");
        content.push_str(&nonfunctional);
        content.push_str("\n\n");

        // Step 4: System Interfaces
        info!("SRS Step 4/6: System Interfaces");
        let interfaces = self.llm_call(&format!(
            "Describe the system interfaces (APIs, external integrations, user interfaces) based on this codebase.\n\n{}\n\nProvide detailed interface descriptions.",
            context
        )).await?;
        content.push_str("## 4. System Interfaces\n\n");
        content.push_str(&interfaces);
        content.push_str("\n\n");

        // Step 5: Data Requirements
        info!("SRS Step 5/6: Data Requirements");
        let data = self.llm_call(&format!(
            "Describe the data requirements, data models, and data flow based on this codebase.\n\n{}\n\nInclude data entities, relationships, and storage requirements.",
            context
        )).await?;
        content.push_str("## 5. Data Requirements\n\n");
        content.push_str(&data);
        content.push_str("\n\n");

        // Step 6: Constraints & Assumptions
        info!("SRS Step 6/6: Constraints & Assumptions");
        let constraints = self.llm_call(&format!(
            "List the constraints, assumptions, and dependencies for this software system.\n\n{}\n\nProvide concrete constraints and assumptions.",
            context
        )).await?;
        content.push_str("## 6. Constraints and Assumptions\n\n");
        content.push_str(&constraints);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Multi-step User Stories generation
    async fn generate_user_stories(&self, snapshot: &CodebaseSnapshot, context: &str) -> Result<String> {
        let mut content = format!("# User Stories\n\n## {}\n\n", self.project_name);

        // Step 1: Identify Personas
        info!("User Stories Step 1/4: Identifying Personas");
        let personas = self.llm_call(&format!(
            "Based on this codebase, identify the user personas/actors who would use this system. Provide 3-5 personas with descriptions.\n\n{}",
            context
        )).await?;
        content.push_str("## User Personas\n\n");
        content.push_str(&personas);
        content.push_str("\n\n");

        // Step 2: Epic Stories
        info!("User Stories Step 2/4: Epic Stories");
        let epics = self.llm_call(&format!(
            "Based on this codebase, create epic-level user stories. Format: As a [persona], I want [goal] so that [benefit]. Create 5-8 epics.\n\n{}",
            context
        )).await?;
        content.push_str("## Epic Stories\n\n");
        content.push_str(&epics);
        content.push_str("\n\n");

        // Step 3: Detailed Stories
        info!("User Stories Step 3/4: Detailed Stories");
        let stories = self.llm_call(&format!(
            "Create detailed user stories with acceptance criteria. Format each story with ID (US-001), story statement, and acceptance criteria. Create 10-15 stories.\n\n{}",
            context
        )).await?;
        content.push_str("## Detailed User Stories\n\n");
        content.push_str(&stories);
        content.push_str("\n\n");

        // Step 4: Story Map
        info!("User Stories Step 4/4: Story Map");
        let map = self.llm_call(&format!(
            "Create a story map organizing the user stories by user journey phases. Show the flow from discovery to completion.\n\n{}",
            context
        )).await?;
        content.push_str("## Story Map\n\n");
        content.push_str(&map);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Multi-step BRD generation
    async fn generate_brd(&self, snapshot: &CodebaseSnapshot, context: &str) -> Result<String> {
        let mut content = format!("# Business Requirements Document\n\n## {}\n\n", self.project_name);

        // Step 1: Business Overview
        info!("BRD Step 1/5: Business Overview");
        let overview = self.llm_call(&format!(
            "Write a business overview section for this software project. Include business context, objectives, and value proposition.\n\n{}",
            context
        )).await?;
        content.push_str("## 1. Business Overview\n\n");
        content.push_str(&overview);
        content.push_str("\n\n");

        // Step 2: Business Objectives
        info!("BRD Step 2/5: Business Objectives");
        let objectives = self.llm_call(&format!(
            "Define SMART business objectives for this project. Include measurable goals and success criteria.\n\n{}",
            context
        )).await?;
        content.push_str("## 2. Business Objectives\n\n");
        content.push_str(&objectives);
        content.push_str("\n\n");

        // Step 3: Stakeholder Analysis
        info!("BRD Step 3/5: Stakeholder Analysis");
        let stakeholders = self.llm_call(&format!(
            "Identify stakeholders for this project. Include their interests, influence, and engagement strategy.\n\n{}",
            context
        )).await?;
        content.push_str("## 3. Stakeholder Analysis\n\n");
        content.push_str(&stakeholders);
        content.push_str("\n\n");

        // Step 4: Business Requirements
        info!("BRD Step 4/5: Business Requirements");
        let requirements = self.llm_call(&format!(
            "List business requirements with IDs (BR-001). Include priority (High/Medium/Low) and rationale.\n\n{}",
            context
        )).await?;
        content.push_str("## 4. Business Requirements\n\n");
        content.push_str(&requirements);
        content.push_str("\n\n");

        // Step 5: ROI & Business Case
        info!("BRD Step 5/5: ROI & Business Case");
        let roi = self.llm_call(&format!(
            "Create a business case section including expected benefits, costs considerations, and ROI analysis framework.\n\n{}",
            context
        )).await?;
        content.push_str("## 5. Business Case & ROI\n\n");
        content.push_str(&roi);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Architecture documentation with diagrams
    async fn generate_architecture_docs(&self, snapshot: &CodebaseSnapshot, context: &str) -> Result<String> {
        let mut content = format!("# Architecture Documentation\n\n## {}\n\n", self.project_name);

        // Step 1: System Overview
        info!("Architecture Step 1/5: System Overview");
        let overview = self.llm_call(&format!(
            "Write a system architecture overview. Describe the high-level design, architectural style, and key components.\n\n{}",
            context
        )).await?;
        content.push_str("## 1. System Overview\n\n");
        content.push_str(&overview);
        content.push_str("\n\n");

        // Step 2: Component Architecture
        info!("Architecture Step 2/5: Component Architecture");
        let components = self.llm_call(&format!(
            "Describe the component architecture. List each major component, its responsibility, and interfaces.\n\n{}",
            context
        )).await?;
        content.push_str("## 2. Component Architecture\n\n");
        content.push_str(&components);
        content.push_str("\n\n");

        // Step 3: Data Architecture
        info!("Architecture Step 3/5: Data Architecture");
        let data = self.llm_call(&format!(
            "Describe the data architecture including data stores, data flow, and data models.\n\n{}",
            context
        )).await?;
        content.push_str("## 3. Data Architecture\n\n");
        content.push_str(&data);
        content.push_str("\n\n");

        // Step 4: Integration Architecture
        info!("Architecture Step 4/5: Integration Architecture");
        let integration = self.llm_call(&format!(
            "Describe how components integrate with each other and external systems. Include APIs, messaging, etc.\n\n{}",
            context
        )).await?;
        content.push_str("## 4. Integration Architecture\n\n");
        content.push_str(&integration);
        content.push_str("\n\n");

        // Step 5: Deployment Architecture
        info!("Architecture Step 5/5: Deployment Architecture");
        let deployment = self.llm_call(&format!(
            "Describe the deployment architecture including infrastructure, scaling, and availability.\n\n{}",
            context
        )).await?;
        content.push_str("## 5. Deployment Architecture\n\n");
        content.push_str(&deployment);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Code documentation
    async fn generate_code_docs(&self, snapshot: &CodebaseSnapshot, context: &str) -> Result<String> {
        let mut content = format!("# Code Documentation\n\n## {}\n\n", self.project_name);

        // Step 1: Getting Started
        info!("Code Docs Step 1/4: Getting Started");
        let getting_started = self.llm_call(&format!(
            "Write a getting started guide for developers. Include setup, prerequisites, and first steps.\n\n{}",
            context
        )).await?;
        content.push_str("## Getting Started\n\n");
        content.push_str(&getting_started);
        content.push_str("\n\n");

        // Step 2: Module Documentation
        info!("Code Docs Step 2/4: Module Documentation");
        let modules = self.llm_call(&format!(
            "Document each major module. Include purpose, public interfaces, and usage examples.\n\n{}",
            context
        )).await?;
        content.push_str("## Module Documentation\n\n");
        content.push_str(&modules);
        content.push_str("\n\n");

        // Step 3: Code Patterns
        info!("Code Docs Step 3/4: Code Patterns");
        let patterns = self.llm_call(&format!(
            "Document the coding patterns and conventions used in this codebase. Include examples.\n\n{}",
            context
        )).await?;
        content.push_str("## Coding Patterns & Conventions\n\n");
        content.push_str(&patterns);
        content.push_str("\n\n");

        // Step 4: Testing Guide
        info!("Code Docs Step 4/4: Testing Guide");
        let testing = self.llm_call(&format!(
            "Write a testing guide. Include how to run tests, write new tests, and testing best practices.\n\n{}",
            context
        )).await?;
        content.push_str("## Testing Guide\n\n");
        content.push_str(&testing);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// API Specifications
    async fn generate_api_specs(&self, snapshot: &CodebaseSnapshot, context: &str) -> Result<String> {
        let mut content = format!("# API Specifications\n\n## {}\n\n", self.project_name);

        // Step 1: API Overview
        info!("API Specs Step 1/4: API Overview");
        let overview = self.llm_call(&format!(
            "Write an API overview including authentication, base URLs, versioning, and general conventions.\n\n{}",
            context
        )).await?;
        content.push_str("## API Overview\n\n");
        content.push_str(&overview);
        content.push_str("\n\n");

        // Step 2: REST Endpoints
        info!("API Specs Step 2/4: REST Endpoints");
        let rest = self.llm_call(&format!(
            "Document REST API endpoints. Include method, path, request/response schemas, and examples.\n\n{}",
            context
        )).await?;
        content.push_str("## REST API Endpoints\n\n");
        content.push_str(&rest);
        content.push_str("\n\n");

        // Step 3: gRPC/GraphQL if applicable
        info!("API Specs Step 3/4: gRPC/GraphQL");
        let other = self.llm_call(&format!(
            "Document any gRPC services or GraphQL schemas if present in the codebase. If not present, note that.\n\n{}",
            context
        )).await?;
        content.push_str("## Other API Interfaces\n\n");
        content.push_str(&other);
        content.push_str("\n\n");

        // Step 4: Error Handling
        info!("API Specs Step 4/4: Error Handling");
        let errors = self.llm_call(&format!(
            "Document API error handling including error codes, error response format, and troubleshooting.\n\n{}",
            context
        )).await?;
        content.push_str("## Error Handling\n\n");
        content.push_str(&errors);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Release Notes
    async fn generate_release_notes(&self, snapshot: &CodebaseSnapshot, context: &str) -> Result<String> {
        let mut content = format!("# Release Notes\n\n## {}\n\n", self.project_name);

        // Step 1: Current Release
        info!("Release Notes Step 1/3: Current Release");
        let current = self.llm_call(&format!(
            "Based on the codebase, write release notes for what appears to be the current version. Include new features, improvements, and bug fixes.\n\n{}",
            context
        )).await?;
        content.push_str("## Current Release\n\n");
        content.push_str(&current);
        content.push_str("\n\n");

        // Step 2: Breaking Changes
        info!("Release Notes Step 2/3: Breaking Changes");
        let breaking = self.llm_call(&format!(
            "Identify any potential breaking changes based on the codebase architecture. Include migration guides.\n\n{}",
            context
        )).await?;
        content.push_str("## Breaking Changes\n\n");
        content.push_str(&breaking);
        content.push_str("\n\n");

        // Step 3: Deprecations & Future
        info!("Release Notes Step 3/3: Roadmap");
        let future = self.llm_call(&format!(
            "Suggest deprecations and future improvements based on the codebase analysis.\n\n{}",
            context
        )).await?;
        content.push_str("## Deprecations & Future Roadmap\n\n");
        content.push_str(&future);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// User Guides
    async fn generate_user_guides(&self, snapshot: &CodebaseSnapshot, context: &str) -> Result<String> {
        let mut content = format!("# User Guides\n\n## {}\n\n", self.project_name);

        // Step 1: Quick Start
        info!("User Guides Step 1/4: Quick Start");
        let quickstart = self.llm_call(&format!(
            "Write a quick start guide for end users. Keep it simple and focused on getting started in 5 minutes.\n\n{}",
            context
        )).await?;
        content.push_str("## Quick Start Guide\n\n");
        content.push_str(&quickstart);
        content.push_str("\n\n");

        // Step 2: Features Guide
        info!("User Guides Step 2/4: Features Guide");
        let features = self.llm_call(&format!(
            "Write a comprehensive features guide explaining each major feature and how to use it.\n\n{}",
            context
        )).await?;
        content.push_str("## Features Guide\n\n");
        content.push_str(&features);
        content.push_str("\n\n");

        // Step 3: Troubleshooting
        info!("User Guides Step 3/4: Troubleshooting");
        let troubleshooting = self.llm_call(&format!(
            "Write a troubleshooting guide with common issues and their solutions.\n\n{}",
            context
        )).await?;
        content.push_str("## Troubleshooting\n\n");
        content.push_str(&troubleshooting);
        content.push_str("\n\n");

        // Step 4: FAQ
        info!("User Guides Step 4/4: FAQ");
        let faq = self.llm_call(&format!(
            "Write a FAQ section with 10-15 frequently asked questions and answers.\n\n{}",
            context
        )).await?;
        content.push_str("## FAQ\n\n");
        content.push_str(&faq);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Operations Runbook
    async fn generate_runbook(&self, snapshot: &CodebaseSnapshot, context: &str) -> Result<String> {
        let mut content = format!("# Operations Runbook\n\n## {}\n\n", self.project_name);

        // Step 1: Deployment Procedures
        info!("Runbook Step 1/5: Deployment");
        let deployment = self.llm_call(&format!(
            "Write deployment procedures including step-by-step deployment, rollback procedures, and verification steps.\n\n{}",
            context
        )).await?;
        content.push_str("## Deployment Procedures\n\n");
        content.push_str(&deployment);
        content.push_str("\n\n");

        // Step 2: Monitoring & Alerting
        info!("Runbook Step 2/5: Monitoring");
        let monitoring = self.llm_call(&format!(
            "Write monitoring and alerting guidelines. Include key metrics, dashboards, and alert thresholds.\n\n{}",
            context
        )).await?;
        content.push_str("## Monitoring & Alerting\n\n");
        content.push_str(&monitoring);
        content.push_str("\n\n");

        // Step 3: Incident Response
        info!("Runbook Step 3/5: Incident Response");
        let incidents = self.llm_call(&format!(
            "Write incident response procedures. Include severity levels, escalation paths, and remediation steps.\n\n{}",
            context
        )).await?;
        content.push_str("## Incident Response\n\n");
        content.push_str(&incidents);
        content.push_str("\n\n");

        // Step 4: Backup & Recovery
        info!("Runbook Step 4/5: Backup & Recovery");
        let backup = self.llm_call(&format!(
            "Write backup and disaster recovery procedures. Include RTO, RPO, and recovery steps.\n\n{}",
            context
        )).await?;
        content.push_str("## Backup & Disaster Recovery\n\n");
        content.push_str(&backup);
        content.push_str("\n\n");

        // Step 5: Maintenance Tasks
        info!("Runbook Step 5/5: Maintenance");
        let maintenance = self.llm_call(&format!(
            "Document routine maintenance tasks including health checks, log rotation, and performance tuning.\n\n{}",
            context
        )).await?;
        content.push_str("## Routine Maintenance\n\n");
        content.push_str(&maintenance);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Make an LLM call using NAFS-4
    async fn llm_call(&self, prompt: &str) -> Result<String> {
        let messages = vec![
            ChatMessage::system("You are an expert technical writer creating professional software documentation. Be concise but comprehensive. Use proper markdown formatting."),
            ChatMessage::user(prompt),
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
