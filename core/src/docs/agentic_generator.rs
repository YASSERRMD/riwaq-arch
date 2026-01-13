//! Agentic Document Generation
//!
//! This module provides multi-step document generation using the internal LLM client.
//! Each document type requires multiple LLM calls to generate comprehensive content.

use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, warn};
use crate::llm::{LLMClient, HttpLlmClient, LLMResponse};
use crate::llm::client::{ArchitectureOverview, ComponentSummary, Adr, AnswerWithRefs};
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

const SYSTEM_PROMPT: &str = "You are an expert technical writer creating professional software documentation. Be concise but comprehensive. Use proper markdown formatting.";

/// Agentic Document Generator using internal LLM client
pub struct AgenticDocGenerator {
    llm_client: Arc<dyn LLMClient>,
    output_dir: PathBuf,
    project_name: String,
}

impl AgenticDocGenerator {
    /// Create a new agentic document generator using RIWAQ_LLM_API_KEY
    pub fn new(output_dir: PathBuf, project_name: String) -> Self {
        let llm_client: Arc<dyn LLMClient> = match HttpLlmClient::with_defaults() {
            Ok(client) => Arc::new(client),
            Err(e) => {
                warn!("Failed to create LLM client: {:?}, using mock", e);
                Arc::new(MockLlmClient)
            }
        };
        Self {
            llm_client,
            output_dir,
            project_name,
        }
    }
    
    /// Create with an external LLM client (use this when you have a configured client from AppState)
    pub fn with_client(llm_client: Arc<dyn LLMClient>, output_dir: PathBuf, project_name: String) -> Self {
        Self {
            llm_client,
            output_dir,
            project_name,
        }
    }
}

/// Mock LLM client for when the real client fails
struct MockLlmClient;

#[async_trait::async_trait]
impl LLMClient for MockLlmClient {
    async fn generate_architecture_overview(
        &self,
        _snapshot: &CodebaseSnapshot,
    ) -> Result<ArchitectureOverview> {
        Ok(ArchitectureOverview {
            content: "Mock architecture overview".to_string(),
            patterns: vec![],
            components: vec![],
        })
    }

    async fn generate_component_docs(
        &self,
        _snapshot: &CodebaseSnapshot,
    ) -> Result<Vec<ComponentSummary>> {
        Ok(vec![])
    }

    async fn generate_adrs(&self, _snapshot: &CodebaseSnapshot) -> Result<Vec<Adr>> {
        Ok(vec![])
    }

    async fn answer_question(
        &self,
        _snapshot: &CodebaseSnapshot,
        _question: &str,
    ) -> Result<AnswerWithRefs> {
        Ok(AnswerWithRefs {
            answer: "Mock answer".to_string(),
            file_refs: vec![],
            confidence: 0.0,
        })
    }

    async fn complete(&self, _prompt: &str, _system_prompt: Option<&str>) -> Result<LLMResponse> {
        Ok(LLMResponse {
            content: "LLM not configured. Set RIWAQ_LLM_API_KEY, RIWAQ_LLM_ENDPOINT, and RIWAQ_LLM_MODEL environment variables.".to_string(),
            prompt_tokens: None,
            completion_tokens: None,
            model: "mock".to_string(),
            streamed: false,
            finish_reason: None,
        })
    }

    async fn complete_stream(
        &self,
        _prompt: &str,
        _system_prompt: Option<&str>,
        _callback: Box<dyn Fn(String) + Send>,
    ) -> Result<LLMResponse> {
        self.complete("", None).await
    }
}

impl AgenticDocGenerator {

    /// Generate a single document type (public API for on-demand generation)
    pub async fn generate_document(
        &self,
        doc_type: DocumentType,
        _snapshot: &CodebaseSnapshot,
        context: &str,
    ) -> Result<PathBuf> {
        info!("Generating single document: {}", doc_type.title());
        
        let content = match doc_type {
            DocumentType::SRS => self.generate_srs(context).await?,
            DocumentType::UserStories => self.generate_user_stories(context).await?,
            DocumentType::BRD => self.generate_brd(context).await?,
            DocumentType::ArchitectureDiagram => self.generate_architecture(context).await?,
            DocumentType::CodeDocs => self.generate_code_docs(context).await?,
            DocumentType::APISpecs => self.generate_api_specs(context).await?,
            DocumentType::ReleaseNotes => self.generate_release_notes(context).await?,
            DocumentType::UserGuides => self.generate_user_guides(context).await?,
            DocumentType::Runbook => self.generate_runbook(context).await?,
        };
        
        let path = self.output_dir.join(doc_type.filename());
        fs::write(&path, &content)?;
        info!("Generated: {}", path.display());
        Ok(path)
    }

    /// Build context summary for LLM (public for use by handlers)
    pub fn build_context_summary(&self, snapshot: &CodebaseSnapshot) -> String {
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

    /// Make an LLM call using the internal client
    async fn llm_call(&self, prompt: &str) -> String {
        match self.llm_client.complete(prompt, Some(SYSTEM_PROMPT)).await {
            Ok(response) => response.content,
            Err(e) => {
                warn!("LLM call failed: {:?}", e);
                format!("*Content generation failed: {:?}*", e)
            }
        }
    }

    /// SRS generation
    async fn generate_srs(&self, context: &str) -> Result<String> {
        let mut content = format!("# Software Requirements Specification\n\n## {}\n\n", self.project_name);

        // Section 1: Executive Summary
        info!("SRS: Generating Executive Summary...");
        let summary = self.llm_call(&format!("Based on this codebase analysis, write an executive summary for an SRS document. Write 2-3 paragraphs summarizing the software's purpose and scope.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## 1. Executive Summary\n\n");
        content.push_str(&summary);
        content.push_str("\n\n");

        // Section 2: Functional Requirements
        info!("SRS: Generating Functional Requirements...");
        let functional = self.llm_call(&format!("Based on this codebase, list the functional requirements. Format as numbered requirements with IDs like FR-001. Provide at least 10 functional requirements.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## 2. Functional Requirements\n\n");
        content.push_str(&functional);
        content.push_str("\n\n");

        // Section 3: Non-Functional Requirements
        info!("SRS: Generating Non-Functional Requirements...");
        let nonfunctional = self.llm_call(&format!("Based on this codebase, list non-functional requirements (performance, security, scalability, reliability). Format as NFR-001, NFR-002, etc. Provide at least 8 non-functional requirements.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## 3. Non-Functional Requirements\n\n");
        content.push_str(&nonfunctional);
        content.push_str("\n\n");

        // Section 4: System Interfaces
        info!("SRS: Generating System Interfaces...");
        let interfaces = self.llm_call(&format!("Describe the system interfaces (APIs, external integrations, user interfaces) based on this codebase. Provide detailed interface descriptions.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## 4. System Interfaces\n\n");
        content.push_str(&interfaces);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// User Stories generation
    async fn generate_user_stories(&self, context: &str) -> Result<String> {
        let mut content = format!("# User Stories\n\n## {}\n\n", self.project_name);

        info!("User Stories: Generating Personas...");
        let personas = self.llm_call(&format!("Based on this codebase, identify the user personas/actors who would use this system. Provide 3-5 personas with descriptions.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## User Personas\n\n");
        content.push_str(&personas);
        content.push_str("\n\n");

        info!("User Stories: Generating Epic Stories...");
        let epics = self.llm_call(&format!("Based on this codebase, create epic-level user stories. Format: As a [persona], I want [goal] so that [benefit]. Create 5-8 epics.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## Epic Stories\n\n");
        content.push_str(&epics);
        content.push_str("\n\n");

        info!("User Stories: Generating Detailed Stories...");
        let stories = self.llm_call(&format!("Create detailed user stories with acceptance criteria. Format each story with ID (US-001), story statement, and acceptance criteria. Create 10-15 stories.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## Detailed User Stories\n\n");
        content.push_str(&stories);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// BRD generation
    async fn generate_brd(&self, context: &str) -> Result<String> {
        let mut content = format!("# Business Requirements Document\n\n## {}\n\n", self.project_name);

        info!("BRD: Generating Business Overview...");
        let overview = self.llm_call(&format!("Write a business overview section for this software project. Include business context, objectives, and value proposition.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## 1. Business Overview\n\n");
        content.push_str(&overview);
        content.push_str("\n\n");

        info!("BRD: Generating Business Objectives...");
        let objectives = self.llm_call(&format!("Define SMART business objectives for this project. Include measurable goals and success criteria.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## 2. Business Objectives\n\n");
        content.push_str(&objectives);
        content.push_str("\n\n");

        info!("BRD: Generating Stakeholder Analysis...");
        let stakeholders = self.llm_call(&format!("Identify stakeholders for this project. Include their interests, influence, and engagement strategy.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## 3. Stakeholder Analysis\n\n");
        content.push_str(&stakeholders);
        content.push_str("\n\n");

        info!("BRD: Generating Business Requirements...");
        let requirements = self.llm_call(&format!("List business requirements with IDs (BR-001). Include priority (High/Medium/Low) and rationale.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## 4. Business Requirements\n\n");
        content.push_str(&requirements);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Architecture docs
    async fn generate_architecture(&self, context: &str) -> Result<String> {
        let mut content = format!("# Architecture Documentation\n\n## {}\n\n", self.project_name);

        info!("Architecture: Generating System Overview...");
        let overview = self.llm_call(&format!("Write a system architecture overview. Describe the high-level design, architectural style, and key components.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## 1. System Overview\n\n");
        content.push_str(&overview);
        content.push_str("\n\n");

        info!("Architecture: Generating Component Architecture...");
        let components = self.llm_call(&format!("Describe the component architecture. List each major component, its responsibility, and interfaces.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## 2. Component Architecture\n\n");
        content.push_str(&components);
        content.push_str("\n\n");

        info!("Architecture: Generating Data Architecture...");
        let data = self.llm_call(&format!("Describe the data architecture including data stores, data flow, and data models.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## 3. Data Architecture\n\n");
        content.push_str(&data);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Code docs
    async fn generate_code_docs(&self, context: &str) -> Result<String> {
        let mut content = format!("# Code Documentation\n\n## {}\n\n", self.project_name);

        info!("Code Docs: Generating Getting Started...");
        let getting_started = self.llm_call(&format!("Write a getting started guide for developers. Include setup, prerequisites, and first steps.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## Getting Started\n\n");
        content.push_str(&getting_started);
        content.push_str("\n\n");

        info!("Code Docs: Generating Module Documentation...");
        let modules = self.llm_call(&format!("Document each major module. Include purpose, public interfaces, and usage examples.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## Module Documentation\n\n");
        content.push_str(&modules);
        content.push_str("\n\n");

        info!("Code Docs: Generating Testing Guide...");
        let testing = self.llm_call(&format!("Write a testing guide. Include how to run tests, write new tests, and testing best practices.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## Testing Guide\n\n");
        content.push_str(&testing);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// API specs
    async fn generate_api_specs(&self, context: &str) -> Result<String> {
        let mut content = format!("# API Specifications\n\n## {}\n\n", self.project_name);

        info!("API Specs: Generating API Overview...");
        let overview = self.llm_call(&format!("Write an API overview including authentication, base URLs, versioning, and general conventions.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## API Overview\n\n");
        content.push_str(&overview);
        content.push_str("\n\n");

        info!("API Specs: Generating REST Endpoints...");
        let rest = self.llm_call(&format!("Document REST API endpoints. Include method, path, request/response schemas, and examples.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## REST API Endpoints\n\n");
        content.push_str(&rest);
        content.push_str("\n\n");

        info!("API Specs: Generating Error Handling...");
        let errors = self.llm_call(&format!("Document API error handling including error codes, error response format, and troubleshooting.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## Error Handling\n\n");
        content.push_str(&errors);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Release notes
    async fn generate_release_notes(&self, context: &str) -> Result<String> {
        let mut content = format!("# Release Notes\n\n## {}\n\n", self.project_name);

        info!("Release Notes: Generating Current Release...");
        let current = self.llm_call(&format!("Based on the codebase, write release notes for what appears to be the current version. Include new features, improvements, and bug fixes.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## Current Release\n\n");
        content.push_str(&current);
        content.push_str("\n\n");

        info!("Release Notes: Generating Breaking Changes...");
        let breaking = self.llm_call(&format!("Identify any potential breaking changes based on the codebase architecture. Include migration guides.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## Breaking Changes\n\n");
        content.push_str(&breaking);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// User guides
    async fn generate_user_guides(&self, context: &str) -> Result<String> {
        let mut content = format!("# User Guides\n\n## {}\n\n", self.project_name);

        info!("User Guides: Generating Quick Start...");
        let quickstart = self.llm_call(&format!("Write a quick start guide for end users. Keep it simple and focused on getting started in 5 minutes.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## Quick Start Guide\n\n");
        content.push_str(&quickstart);
        content.push_str("\n\n");

        info!("User Guides: Generating Features Guide...");
        let features = self.llm_call(&format!("Write a comprehensive features guide explaining each major feature and how to use it.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## Features Guide\n\n");
        content.push_str(&features);
        content.push_str("\n\n");

        info!("User Guides: Generating FAQ...");
        let faq = self.llm_call(&format!("Write a FAQ section with 10-15 frequently asked questions and answers.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## FAQ\n\n");
        content.push_str(&faq);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
    }

    /// Runbook
    async fn generate_runbook(&self, context: &str) -> Result<String> {
        let mut content = format!("# Operations Runbook\n\n## {}\n\n", self.project_name);

        info!("Runbook: Generating Deployment Procedures...");
        let deployment = self.llm_call(&format!("Write deployment procedures including step-by-step deployment, rollback procedures, and verification steps.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## Deployment Procedures\n\n");
        content.push_str(&deployment);
        content.push_str("\n\n");

        info!("Runbook: Generating Monitoring Guidelines...");
        let monitoring = self.llm_call(&format!("Write monitoring and alerting guidelines. Include key metrics, dashboards, and alert thresholds.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## Monitoring & Alerting\n\n");
        content.push_str(&monitoring);
        content.push_str("\n\n");

        info!("Runbook: Generating Incident Response...");
        let incidents = self.llm_call(&format!("Write incident response procedures. Include severity levels, escalation paths, and remediation steps.\n\nCodebase Context:\n{}", context)).await;
        content.push_str("## Incident Response\n\n");
        content.push_str(&incidents);
        content.push_str("\n\n");

        content.push_str(&format!("\n---\n*Generated by Riwaq Arch on {}*\n", chrono::Local::now().format("%Y-%m-%d")));
        Ok(content)
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
