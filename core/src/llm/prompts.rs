//! Prompt templates for LLM interactions.
//!
//! This module contains carefully crafted prompts for different
//! documentation generation tasks.

/// System prompt for all LLM interactions.
pub const SYSTEM_PROMPT: &str = r#"You are Riwaq, an expert software architect assistant. Your role is to analyze codebases and generate comprehensive, accurate documentation.

Guidelines:
1. Be precise and technical, but clear
2. Use Markdown formatting for all outputs
3. Include code references when relevant (file paths, function names)
4. Identify patterns, best practices, and potential issues
5. Generate documentation that would help a new developer onboard quickly
6. When generating diagrams, use Mermaid syntax
7. Be objective and fact-based - don't make assumptions beyond what the code shows

Always structure your responses clearly with headers and sections."#;

/// Generate prompt for architecture overview.
pub fn architecture_overview_prompt(context: &str) -> String {
    format!(
        r#"Based on the following codebase analysis, generate a comprehensive Architecture Overview document.

## Codebase Analysis

{context}

## Required Output

Generate a Markdown document with the following sections:

### 1. Executive Summary
A brief (2-3 paragraph) overview of what this system does, its purpose, and key architectural decisions.

### 2. System Architecture
Describe the high-level architecture. Include:
- Architectural style/pattern (e.g., microservices, monolith, layered, hexagonal)
- Main components and their responsibilities
- How components interact

### 3. Component Diagram
Create a Mermaid diagram showing the main components and their relationships:
```mermaid
graph TD
    A[Component A] --> B[Component B]
```

### 4. Data Flow
Describe how data flows through the system for key operations.

### 5. Technology Stack
List the main technologies, frameworks, and libraries used.

### 6. Key Design Decisions
Highlight important architectural decisions visible in the code structure.

### 7. Areas of Note
- Potential technical debt
- Circular dependencies (if any)
- Hot spots (frequently changing code)

Be thorough but concise. Focus on what a new team member needs to understand the system."#
    )
}

/// Generate prompt for component documentation.
pub fn component_doc_prompt(name: &str, path: &str, public_functions: &[String]) -> String {
    let functions_list = if public_functions.is_empty() {
        "No public functions found".to_string()
    } else {
        public_functions
            .iter()
            .map(|f| format!("- `{}`", f))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        r#"Generate documentation for the following module/component:

## Module Information
- **Name**: {name}
- **Path**: {path}
- **Public Functions**:
{functions_list}

## Required Output

Generate a Markdown document with:

### 1. Purpose
What is this module's primary responsibility? (1-2 sentences)

### 2. Overview
A paragraph describing what this module does and how it fits into the larger system.

### 3. Public API
For each public function, provide:
- Brief description
- Parameters
- Return value
- Example usage (if appropriate)

### 4. Dependencies
What other modules/external libraries does this depend on?

### 5. Usage Example
A simple code example showing typical usage.

Be concise but complete."#
    )
}

/// Generate prompt for ADR generation.
pub fn adr_generation_prompt(context: &str) -> String {
    format!(
        r#"Based on the following codebase analysis, identify and generate Architecture Decision Records (ADRs) for the key architectural decisions evident in this codebase.

## Codebase Analysis

{context}

## Required Output

Generate 3-5 ADRs for the most significant architectural decisions you can infer from the code structure. Each ADR should follow this format:

---

# ADR-XXX: [Title]

## Status
[Accepted/Proposed/Deprecated]

## Context
What is the issue that we're seeing that is motivating this decision or change?

## Decision
What is the change that we're proposing and/or doing?

## Consequences
What becomes easier or more difficult to do because of this change?

---

Focus on decisions that are:
1. Visible from the code structure (technology choices, patterns used)
2. Impactful for the system's architecture
3. Useful for future maintainers to understand

Infer the reasoning behind the decisions based on common software engineering practices."#
    )
}

/// Generate prompt for answering questions about the codebase.
pub fn question_answer_prompt(context: &str, question: &str) -> String {
    format!(
        r#"Based on the following codebase analysis, answer the user's question.

## Codebase Analysis

{context}

## User Question

{question}

## Instructions

1. Answer the question directly and accurately based on the codebase information provided
2. Reference specific files, modules, or functions when relevant
3. If the question cannot be fully answered from the available information, say so
4. If you need to make assumptions, state them clearly
5. Provide code snippets or examples if they would help clarify the answer

Format your answer in clear Markdown with appropriate headers if needed."#
    )
}

/// Generate prompt for Business Requirements Document (BRD).
pub fn generate_brd_prompt(project_name: &str, context: &str) -> String {
    format!(
        r#"Act as a Senior Business Analyst. Reverse-engineer a Business Requirements Document (BRD) based on this codebase.

Project: {project_name}

## Codebase Context
{context}

## Required Output (Markdown)

# Business Requirements Document (BRD)

## 1. Executive Summary
- High-level overview of the solution.
- Business problems it solves (inferred from features).

## 2. Business Objectives
- Key goals (e.g., "Enable real-time collaboration", "Secure data storage", "Automate workflows").

## 3. User Personas (Inferred)
- Who are the likely users? (e.g., Admin, Customer, System).

## 4. Functional Requirements (Business View)
- List high-level features mapped to business needs.
- Format: "The system shall [do X] to enable [Business Value Y]."

## 5. Non-Functional Requirements
- Performance, Security, Scalability constraints observed or implied.

## 6. Glossary
- Key domain terms found in the code.

Focus on 'WHAT' and 'WHY', not 'HOW'."#
    )
}

/// Generate prompt for Software Requirements Specification (SRS).
pub fn generate_srs_prompt(project_name: &str, context: &str) -> String {
    format!(
        r#"Act as a Lead Systems Architect. Reverse-engineer a Software Requirements Specification (SRS) based on this codebase.

Project: {project_name}

## Codebase Context
{context}

## Required Output (Markdown)

# Software Requirements Specification (SRS)

## 1. Introduction
- Purpose of this document.
- Scope of the software.

## 2. System Overview
- High-level architecture summary.
- Core technologies and their justification.

## 3. Detailed Functional Requirements
- Break down features into specific technical requirements.
- **API Capabilities**: Summary of REST/gRPC/GraphQL capabilities.
- **Data Processing**: Key validation and processing rules.

## 4. Interface Requirements
- External System Integrations (Databases, 3rd Party APIs).
- User Interfaces (CLI, Web).

## 5. System Attributes
- Reliability points (retries, error handling).
- Security measures (Auth, encryption).
- Maintainability patterns.

## 6. Data Model Requirements
- Key entities and relationships.

Be technical and precise. This document is for developers/integrators."#
    )
}

/// Generate prompt for API documentation.
pub fn api_documentation_prompt(endpoints: &str) -> String {
    format!(
        r#"Generate comprehensive API documentation intended for external integrators.
The documentation must cover all detected interfaces including REST, gRPC, and GraphQL.

## Endpoints/Services Analysis

{endpoints}

## Required Output

Generate a Markdown API Reference with the following structure:

### 1. Introduction
- Overview of available interfaces (REST/gRPC/GraphQL).
- Base URLs and Environments.
- Authentication mechanisms (Bearer Token, API Key, etc.).

### 2. REST API Reference (if applicable)
For each endpoint:
- **`[METHOD] /path`**
- **Description**: Clear explanation of purpose.
- **Parameters**: Header, Path, Query, Body.
- **Response**: Success (200) and Error codes (4xx, 5xx) with JSON examples.
- **Example**: `curl` command.

### 3. gRPC Service Definition (if applicable)
For each service:
- **Service Name**
- **RPC Methods**: Input message type, Output message type.
- **Protobuf Snippet**: Show the `.proto` definition.

### 4. GraphQL Schema (if applicable)
- **Queries**: Available data fetching operations.
- **Mutations**: Data modification operations.
- **Types**: Key data structures.

Be strictly technical and precise. Use professional formatting."#
    )
}

/// Generate prompt for dependency analysis.
pub fn dependency_analysis_prompt(context: &str) -> String {
    format!(
        r#"Analyze the dependencies and architecture of this codebase:

## Codebase Analysis

{context}

## Required Output

Generate a Markdown document covering:

### 1. Dependency Overview
List and categorize the main dependencies:
- Runtime dependencies
- Development dependencies
- Optional dependencies

### 2. Dependency Graph
Create a Mermaid diagram showing module dependencies:
```mermaid
graph LR
    subgraph Core
        A --> B
    end
```

### 3. Coupling Analysis
- Identify tightly coupled components
- Suggest potential improvements

### 4. Circular Dependencies
List any circular dependencies and their impact.

### 5. Risk Assessment
Identify:
- Single points of failure
- Outdated or risky dependencies
- Security considerations

### 6. Recommendations
Suggest improvements for the dependency structure."#
    )
}

/// Truncate context to fit within token limits.
pub fn truncate_context(context: &str, max_chars: usize) -> String {
    if context.len() <= max_chars {
        return context.to_string();
    }

    // Try to truncate at a sensible boundary
    let truncated = &context[..max_chars];
    
    // Find the last complete section
    if let Some(pos) = truncated.rfind("\n##") {
        return format!("{}\n\n... (truncated)", &truncated[..pos]);
    }
    
    // Fall back to truncating at a newline
    if let Some(pos) = truncated.rfind('\n') {
        return format!("{}\n\n... (truncated)", &truncated[..pos]);
    }
    
    format!("{}... (truncated)", truncated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_overview_prompt() {
        let context = "# Project\nSome context here";
        let prompt = architecture_overview_prompt(context);
        
        assert!(prompt.contains("Architecture Overview"));
        assert!(prompt.contains(context));
        assert!(prompt.contains("Mermaid"));
    }

    #[test]
    fn test_component_doc_prompt() {
        let prompt = component_doc_prompt(
            "auth",
            "src/auth",
            &["login".to_string(), "logout".to_string()],
        );
        
        assert!(prompt.contains("auth"));
        assert!(prompt.contains("`login`"));
        assert!(prompt.contains("`logout`"));
    }

    #[test]
    fn test_truncate_context() {
        let context = "## Section 1\nContent 1\n## Section 2\nContent 2";
        let truncated = truncate_context(context, 30);
        
        assert!(truncated.len() <= 50); // 30 + some overhead
        assert!(truncated.contains("truncated"));
    }

    #[test]
    fn test_question_answer_prompt() {
        let prompt = question_answer_prompt("context here", "How does auth work?");
        
        assert!(prompt.contains("How does auth work?"));
        assert!(prompt.contains("context here"));
    }
}
