//! Enhanced Context Builder for LLM Interactions
//!
//! This module provides intelligent context building for LLM queries:
//! - Semantic file relevance matching
//! - Related code discovery
//! - Entity-aware context
//! - Workflow-aware responses

use crate::models::snapshot::CodebaseSnapshot;
use crate::analysis::api::ApiInventory;
use crate::analysis::business::BusinessLogicInventory;
use std::collections::{HashMap, HashSet};

/// Enhanced context for LLM queries
#[derive(Debug, Clone)]
pub struct EnhancedContext {
    /// Main context text
    pub text: String,
    /// Relevant files with snippets
    pub relevant_files: Vec<RelevantFile>,
    /// Related entities
    pub related_entities: Vec<String>,
    /// Related API endpoints
    pub related_endpoints: Vec<String>,
    /// Related workflows
    pub related_workflows: Vec<String>,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
}

/// A relevant file with code snippets
#[derive(Debug, Clone)]
pub struct RelevantFile {
    pub path: String,
    pub relevance_score: f32,
    pub snippet: Option<String>,
    pub line_start: Option<usize>,
    pub line_end: Option<usize>,
    pub reason: String,
}

/// Context builder for enhanced LLM interactions
pub struct ContextBuilder {
    /// Maximum context length in characters
    max_context_chars: usize,
    /// Maximum number of relevant files
    max_relevant_files: usize,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            max_context_chars: 32000, // ~8K tokens
            max_relevant_files: 10,
        }
    }

    pub fn with_max_chars(mut self, max_chars: usize) -> Self {
        self.max_context_chars = max_chars;
        self
    }

    /// Build enhanced context for a question
    pub fn build_for_question(
        &self,
        question: &str,
        snapshot: &CodebaseSnapshot,
        api_inventory: Option<&ApiInventory>,
        business_inventory: Option<&BusinessLogicInventory>,
    ) -> EnhancedContext {
        let keywords = self.extract_keywords(question);
        let intent = self.detect_intent(question);
        
        let mut relevant_files = Vec::new();
        let mut related_entities = Vec::new();
        let mut related_endpoints = Vec::new();
        let mut related_workflows = Vec::new();

        // Find relevant files based on keywords
        relevant_files.extend(
            self.find_relevant_files(snapshot, &keywords, &intent)
        );

        // Find related API endpoints
        if let Some(api) = api_inventory {
            related_endpoints.extend(
                self.find_related_endpoints(api, &keywords)
            );
        }

        // Find related business entities and workflows
        if let Some(business) = business_inventory {
            related_entities.extend(
                self.find_related_entities(business, &keywords)
            );
            related_workflows.extend(
                self.find_related_workflows(business, &keywords)
            );
        }

        // Build the context text
        let text = self.build_context_text(
            snapshot,
            &relevant_files,
            &related_entities,
            &related_endpoints,
            &related_workflows,
        );

        // Calculate confidence
        let confidence = self.calculate_confidence(
            &keywords,
            &relevant_files,
            &related_entities,
        );

        EnhancedContext {
            text,
            relevant_files,
            related_entities,
            related_endpoints,
            related_workflows,
            confidence,
        }
    }

    /// Extract keywords from a question
    fn extract_keywords(&self, question: &str) -> Vec<String> {
        let stop_words: HashSet<&str> = [
            "the", "a", "an", "is", "are", "was", "were", "be", "been", "being",
            "have", "has", "had", "do", "does", "did", "will", "would", "could",
            "should", "may", "might", "can", "this", "that", "these", "those",
            "i", "you", "he", "she", "it", "we", "they", "what", "which", "who",
            "where", "when", "why", "how", "all", "each", "every", "both",
            "few", "more", "most", "other", "some", "such", "no", "nor", "not",
            "only", "own", "same", "so", "than", "too", "very", "just", "or",
            "and", "but", "if", "or", "because", "as", "until", "while", "of",
            "at", "by", "for", "with", "about", "between", "into", "through",
            "during", "before", "after", "above", "below", "to", "from", "up",
            "down", "in", "out", "on", "off", "over", "under", "again", "then",
        ].into_iter().collect();

        question
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|word| word.len() > 2 && !stop_words.contains(word))
            .map(|s| s.to_string())
            .collect()
    }

    /// Detect the intent of the question
    fn detect_intent(&self, question: &str) -> QuestionIntent {
        let q = question.to_lowercase();

        if q.contains("how") && (q.contains("work") || q.contains("implement")) {
            QuestionIntent::HowItWorks
        } else if q.contains("where") || q.contains("find") || q.contains("locate") {
            QuestionIntent::FindCode
        } else if q.contains("why") {
            QuestionIntent::Reasoning
        } else if q.contains("what") && q.contains("api") {
            QuestionIntent::ApiReference
        } else if q.contains("flow") || q.contains("process") || q.contains("workflow") {
            QuestionIntent::WorkflowExplanation
        } else if q.contains("database") || q.contains("model") || q.contains("entity") {
            QuestionIntent::DataModel
        } else if q.contains("auth") || q.contains("permission") || q.contains("security") {
            QuestionIntent::Security
        } else if q.contains("config") || q.contains("setting") || q.contains("environment") {
            QuestionIntent::Configuration
        } else {
            QuestionIntent::General
        }
    }

    /// Find files relevant to the keywords
    fn find_relevant_files(
        &self,
        snapshot: &CodebaseSnapshot,
        keywords: &[String],
        intent: &QuestionIntent,
    ) -> Vec<RelevantFile> {
        let mut scored_files: Vec<(f32, RelevantFile)> = Vec::new();

        for file in &snapshot.files {
            let path_lower = file.path.to_string_lossy().to_lowercase();
            let mut score: f32 = 0.0;
            let mut reasons = Vec::new();

            // Keyword matching in path
            for keyword in keywords {
                if path_lower.contains(keyword) {
                    score += 2.0;
                    reasons.push(format!("path contains '{}'", keyword));
                }
            }

            // Keyword matching in function names
            for func in &file.functions {
                let func_lower = func.name.to_lowercase();
                for keyword in keywords {
                    if func_lower.contains(keyword) {
                        score += 1.5;
                        reasons.push(format!("function '{}' matches", func.name));
                    }
                }
            }

            // Keyword matching in type names
            for typ in &file.types {
                let type_lower = typ.name.to_lowercase();
                for keyword in keywords {
                    if type_lower.contains(keyword) {
                        score += 1.5;
                        reasons.push(format!("type '{}' matches", typ.name));
                    }
                }
            }

            // Intent-based boosting
            match intent {
                QuestionIntent::ApiReference => {
                    if path_lower.contains("route") || path_lower.contains("handler") 
                        || path_lower.contains("api") || path_lower.contains("endpoint") {
                        score += 3.0;
                    }
                }
                QuestionIntent::DataModel => {
                    if path_lower.contains("model") || path_lower.contains("entity")
                        || path_lower.contains("schema") {
                        score += 3.0;
                    }
                }
                QuestionIntent::Security => {
                    if path_lower.contains("auth") || path_lower.contains("security")
                        || path_lower.contains("permission") {
                        score += 3.0;
                    }
                }
                QuestionIntent::Configuration => {
                    if path_lower.contains("config") || path_lower.contains("settings")
                        || path_lower.ends_with(".toml") || path_lower.ends_with(".yaml") {
                        score += 3.0;
                    }
                }
                _ => {}
            }

            if score > 0.0 {
                scored_files.push((score, RelevantFile {
                    path: file.path.to_string_lossy().to_string(),
                    relevance_score: score,
                    snippet: None,
                    line_start: None,
                    line_end: None,
                    reason: reasons.join(", "),
                }));
            }
        }

        // Sort by score and take top N
        scored_files.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored_files
            .into_iter()
            .take(self.max_relevant_files)
            .map(|(_, f)| f)
            .collect()
    }

    /// Find related API endpoints
    fn find_related_endpoints(&self, api: &ApiInventory, keywords: &[String]) -> Vec<String> {
        let mut endpoints = Vec::new();

        for endpoint in &api.rest.endpoints {
            let path_lower = endpoint.path.to_lowercase();
            let handler_lower = endpoint.handler.to_lowercase();

            for keyword in keywords {
                if path_lower.contains(keyword) || handler_lower.contains(keyword) {
                    endpoints.push(format!("{} {}", endpoint.method, endpoint.path));
                    break;
                }
            }
        }

        endpoints.truncate(5);
        endpoints
    }

    /// Find related business entities
    fn find_related_entities(&self, business: &BusinessLogicInventory, keywords: &[String]) -> Vec<String> {
        let mut entities = Vec::new();

        for entity in &business.entities {
            let name_lower = entity.name.to_lowercase();

            for keyword in keywords {
                if name_lower.contains(keyword) {
                    entities.push(entity.name.clone());
                    break;
                }
            }

            // Also check fields
            for field in &entity.fields {
                let field_lower = field.name.to_lowercase();
                for keyword in keywords {
                    if field_lower.contains(keyword) {
                        entities.push(entity.name.clone());
                        break;
                    }
                }
            }
        }

        entities.sort();
        entities.dedup();
        entities.truncate(5);
        entities
    }

    /// Find related workflows
    fn find_related_workflows(&self, business: &BusinessLogicInventory, keywords: &[String]) -> Vec<String> {
        let mut workflows = Vec::new();

        for workflow in &business.workflows {
            let name_lower = workflow.name.to_lowercase();

            for keyword in keywords {
                if name_lower.contains(keyword) {
                    workflows.push(workflow.name.clone());
                    break;
                }
            }

            // Check states
            for state in &workflow.states {
                let state_lower = state.name.to_lowercase();
                for keyword in keywords {
                    if state_lower.contains(keyword) {
                        workflows.push(workflow.name.clone());
                        break;
                    }
                }
            }
        }

        workflows.sort();
        workflows.dedup();
        workflows.truncate(3);
        workflows
    }

    /// Build the context text
    fn build_context_text(
        &self,
        snapshot: &CodebaseSnapshot,
        relevant_files: &[RelevantFile],
        related_entities: &[String],
        related_endpoints: &[String],
        related_workflows: &[String],
    ) -> String {
        let mut context = String::new();

        // Project overview
        context.push_str(&format!(
            "Project: {}\n",
            snapshot.metadata.project_name
        ));
        context.push_str(&format!(
            "Languages: {}\n",
            snapshot.statistics.languages
                .iter()
                .map(|l| format!("{} ({})", l.language, l.file_count))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        context.push_str(&format!(
            "Stats: {} files, {} modules, {} functions\n\n",
            snapshot.statistics.total_files,
            snapshot.statistics.total_modules,
            snapshot.statistics.total_functions
        ));

        // Relevant files
        if !relevant_files.is_empty() {
            context.push_str("## Relevant Files\n\n");
            for file in relevant_files {
                context.push_str(&format!(
                    "- `{}` (score: {:.1}) - {}\n",
                    file.path,
                    file.relevance_score,
                    file.reason
                ));
            }
            context.push('\n');
        }

        // Related API endpoints
        if !related_endpoints.is_empty() {
            context.push_str("## Related API Endpoints\n\n");
            for endpoint in related_endpoints {
                context.push_str(&format!("- `{}`\n", endpoint));
            }
            context.push('\n');
        }

        // Related entities
        if !related_entities.is_empty() {
            context.push_str("## Related Data Entities\n\n");
            for entity in related_entities {
                context.push_str(&format!("- `{}`\n", entity));
            }
            context.push('\n');
        }

        // Related workflows
        if !related_workflows.is_empty() {
            context.push_str("## Related Workflows\n\n");
            for workflow in related_workflows {
                context.push_str(&format!("- `{}`\n", workflow));
            }
            context.push('\n');
        }

        // Module structure
        context.push_str("## Module Structure\n\n");
        for module in snapshot.modules.iter().take(15) {
            context.push_str(&format!(
                "- **{}**: {} files, {} public functions\n",
                module.name,
                module.files.len(),
                module.public_functions.len()
            ));
        }

        // Truncate if needed
        if context.len() > self.max_context_chars {
            context.truncate(self.max_context_chars - 100);
            context.push_str("\n\n[Context truncated for token limit]");
        }

        context
    }

    /// Calculate confidence score
    fn calculate_confidence(
        &self,
        keywords: &[String],
        relevant_files: &[RelevantFile],
        related_entities: &[String],
    ) -> f32 {
        if keywords.is_empty() {
            return 0.3;
        }

        let file_score = if relevant_files.is_empty() {
            0.0
        } else {
            (relevant_files.len() as f32 / keywords.len() as f32).min(1.0)
        };

        let entity_score = if related_entities.is_empty() {
            0.0
        } else {
            0.3
        };

        let base_confidence = 0.4;
        (base_confidence + file_score * 0.4 + entity_score).min(1.0)
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Question intent classification
#[derive(Debug, Clone, PartialEq)]
pub enum QuestionIntent {
    /// How does X work?
    HowItWorks,
    /// Where is X?
    FindCode,
    /// Why is X done this way?
    Reasoning,
    /// What APIs are available?
    ApiReference,
    /// Explain a workflow/process
    WorkflowExplanation,
    /// Data model questions
    DataModel,
    /// Security/auth questions
    Security,
    /// Configuration questions
    Configuration,
    /// General question
    General,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_keywords() {
        let builder = ContextBuilder::new();
        let keywords = builder.extract_keywords("How does authentication work in this project?");
        
        assert!(keywords.contains(&"authentication".to_string()));
        assert!(keywords.contains(&"work".to_string()));
        assert!(keywords.contains(&"project".to_string()));
        // Stop words should be filtered
        assert!(!keywords.contains(&"how".to_string()));
        assert!(!keywords.contains(&"does".to_string()));
    }

    #[test]
    fn test_detect_intent() {
        let builder = ContextBuilder::new();
        
        assert_eq!(
            builder.detect_intent("How does payment processing work?"),
            QuestionIntent::HowItWorks
        );
        assert_eq!(
            builder.detect_intent("Where is the authentication code?"),
            QuestionIntent::FindCode
        );
        assert_eq!(
            builder.detect_intent("What API endpoints are available?"),
            QuestionIntent::ApiReference
        );
    }
}
