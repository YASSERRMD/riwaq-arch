//! Business Logic Extraction Module
//!
//! Intelligent reverse-engineering of business logic:
//! - Entity/Data Model detection
//! - Workflow & State Machine detection
//! - Business Rule extraction
//! - Process & Integration flow detection

pub mod entities;
pub mod workflows;
pub mod rules;
pub mod integrations;

use serde::{Deserialize, Serialize};
use std::path::Path;

pub use entities::EntityAnalyzer;
pub use workflows::WorkflowAnalyzer;
pub use rules::RuleAnalyzer;
pub use integrations::IntegrationAnalyzer;

/// Complete business logic inventory
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessLogicInventory {
    /// Data model entities
    pub entities: Vec<Entity>,
    /// Entity relationships
    pub relationships: Vec<EntityRelationship>,
    /// State machines / workflows
    pub workflows: Vec<Workflow>,
    /// Business rules
    pub rules: Vec<BusinessRule>,
    /// External integrations
    pub integrations: Vec<Integration>,
    /// Statistics
    pub statistics: BusinessStatistics,
}

/// Database/domain entity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    /// Entity name (e.g., "User", "Order")
    pub name: String,
    /// Entity type (model, enum, value object)
    pub entity_type: EntityType,
    /// Fields/properties
    pub fields: Vec<EntityField>,
    /// Validation rules
    pub validations: Vec<Validation>,
    /// Primary key field
    pub primary_key: Option<String>,
    /// Indexes
    pub indexes: Vec<String>,
    /// Source file
    pub file: String,
    /// Line number
    pub line: usize,
    /// Description
    pub description: Option<String>,
}

/// Entity type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntityType {
    Model,
    Enum,
    ValueObject,
    Aggregate,
    View,
}

/// Entity field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityField {
    pub name: String,
    pub field_type: String,
    pub nullable: bool,
    pub unique: bool,
    pub indexed: bool,
    pub default_value: Option<String>,
    pub description: Option<String>,
}

/// Validation rule on a field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Validation {
    pub field: String,
    pub rule_type: String,
    pub parameters: String,
}

/// Relationship between entities
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityRelationship {
    pub from_entity: String,
    pub to_entity: String,
    pub relationship_type: RelationshipType,
    pub field_name: String,
    pub cascade: bool,
}

/// Relationship type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

/// Workflow / State Machine
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workflow {
    pub name: String,
    pub entity: Option<String>,
    pub states: Vec<WorkflowState>,
    pub transitions: Vec<StateTransition>,
    pub initial_state: Option<String>,
    pub terminal_states: Vec<String>,
    pub file: String,
    pub line: usize,
}

/// Workflow state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowState {
    pub name: String,
    pub description: Option<String>,
    pub is_initial: bool,
    pub is_terminal: bool,
    pub entry_actions: Vec<String>,
    pub exit_actions: Vec<String>,
}

/// State transition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateTransition {
    pub from_state: String,
    pub to_state: String,
    pub trigger: Option<String>,
    pub condition: Option<String>,
    pub actions: Vec<String>,
}

/// Business rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessRule {
    pub name: String,
    pub description: String,
    pub rule_type: BusinessRuleType,
    pub condition: String,
    pub action: Option<String>,
    pub entities: Vec<String>,
    pub file: String,
    pub line: usize,
    pub severity: RuleSeverity,
}

/// Business rule type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BusinessRuleType {
    Validation,
    Authorization,
    Calculation,
    Constraint,
    Trigger,
    Policy,
}

/// Rule severity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuleSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// External integration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Integration {
    pub name: String,
    pub integration_type: IntegrationType,
    pub purpose: String,
    pub endpoints: Vec<String>,
    pub auth_method: Option<String>,
    pub file: String,
}

/// Integration type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IntegrationType {
    Payment,
    Email,
    Authentication,
    Storage,
    Analytics,
    Messaging,
    Database,
    Cache,
    Search,
    Other,
}

/// Business logic statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessStatistics {
    pub total_entities: usize,
    pub total_relationships: usize,
    pub total_workflows: usize,
    pub total_states: usize,
    pub total_rules: usize,
    pub total_integrations: usize,
    pub critical_rules: usize,
}

/// Main business logic analyzer
pub struct BusinessLogicAnalyzer {
    entity_analyzer: EntityAnalyzer,
    workflow_analyzer: WorkflowAnalyzer,
    rule_analyzer: RuleAnalyzer,
    integration_analyzer: IntegrationAnalyzer,
}

impl BusinessLogicAnalyzer {
    pub fn new() -> Self {
        Self {
            entity_analyzer: EntityAnalyzer::new(),
            workflow_analyzer: WorkflowAnalyzer::new(),
            rule_analyzer: RuleAnalyzer::new(),
            integration_analyzer: IntegrationAnalyzer::new(),
        }
    }

    pub async fn analyze(&self, root_path: &Path) -> anyhow::Result<BusinessLogicInventory> {
        let (entities, relationships) = self.entity_analyzer.analyze(root_path).await?;
        let workflows = self.workflow_analyzer.analyze(root_path).await?;
        let rules = self.rule_analyzer.analyze(root_path).await?;
        let integrations = self.integration_analyzer.analyze(root_path).await?;

        let statistics = BusinessStatistics {
            total_entities: entities.len(),
            total_relationships: relationships.len(),
            total_workflows: workflows.len(),
            total_states: workflows.iter().map(|w| w.states.len()).sum(),
            total_rules: rules.len(),
            total_integrations: integrations.len(),
            critical_rules: rules.iter()
                .filter(|r| matches!(r.severity, RuleSeverity::Critical))
                .count(),
        };

        Ok(BusinessLogicInventory {
            entities,
            relationships,
            workflows,
            rules,
            integrations,
            statistics,
        })
    }
}

impl Default for BusinessLogicAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
