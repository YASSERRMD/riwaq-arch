//! Business Model Documentation Generator
//!
//! Generates comprehensive business documentation:
//! - Entity/Data model diagrams
//! - Workflow state diagrams
//! - Business rules documentation
//! - Integration mappings

use crate::analysis::business::*;

pub struct BusinessModelGenerator;

impl BusinessModelGenerator {
    pub fn generate(inventory: &BusinessLogicInventory) -> String {
        let mut output = String::new();

        output.push_str("# Business Model Documentation\n\n");
        output.push_str("*Auto-generated business logic documentation*\n\n");

        // Summary
        output.push_str("## Overview\n\n");
        output.push_str(&Self::generate_summary(&inventory.statistics));
        output.push('\n');

        // Data Model
        if !inventory.entities.is_empty() {
            output.push_str(&Self::generate_data_model(&inventory.entities, &inventory.relationships));
        }

        // Workflows
        if !inventory.workflows.is_empty() {
            output.push_str(&Self::generate_workflows(&inventory.workflows));
        }

        // Business Rules
        if !inventory.rules.is_empty() {
            output.push_str(&Self::generate_rules(&inventory.rules));
        }

        // Integrations
        if !inventory.integrations.is_empty() {
            output.push_str(&Self::generate_integrations(&inventory.integrations));
        }

        output
    }

    fn generate_summary(stats: &BusinessStatistics) -> String {
        let mut output = String::new();

        output.push_str("| Category | Count |\n");
        output.push_str("|----------|-------|\n");
        output.push_str(&format!("| Entities/Models | {} |\n", stats.total_entities));
        output.push_str(&format!("| Relationships | {} |\n", stats.total_relationships));
        output.push_str(&format!("| Workflows | {} |\n", stats.total_workflows));
        output.push_str(&format!("| Business Rules | {} |\n", stats.total_rules));
        output.push_str(&format!("| External Integrations | {} |\n", stats.total_integrations));

        if stats.critical_rules > 0 {
            output.push_str(&format!("\n⚠️ **Critical Rules**: {} rules marked as critical\n", stats.critical_rules));
        }

        output
    }

    fn generate_data_model(entities: &[Entity], relationships: &[EntityRelationship]) -> String {
        let mut output = String::new();

        output.push_str("## Data Model\n\n");

        // ER Diagram in Mermaid
        output.push_str("### Entity Relationship Diagram\n\n");
        output.push_str("```mermaid\n");
        output.push_str("erDiagram\n");

        for entity in entities {
            output.push_str(&format!("    {} {{\n", entity.name));
            for field in &entity.fields {
                let pk = if Some(&field.name) == entity.primary_key.as_ref() { " PK" } else { "" };
                let unique = if field.unique { " UK" } else { "" };
                output.push_str(&format!(
                    "        {} {}{}{}\n",
                    field.field_type.replace('<', "_").replace('>', "_"),
                    field.name,
                    pk,
                    unique
                ));
            }
            output.push_str("    }\n");
        }

        // Relationships
        for rel in relationships {
            let rel_symbol = match rel.relationship_type {
                RelationshipType::OneToOne => "||--||",
                RelationshipType::OneToMany => "||--o{",
                RelationshipType::ManyToOne => "}o--||",
                RelationshipType::ManyToMany => "}o--o{",
            };
            output.push_str(&format!(
                "    {} {} {} : {}\n",
                rel.from_entity,
                rel_symbol,
                rel.to_entity,
                rel.field_name
            ));
        }

        output.push_str("```\n\n");

        // Entity details
        output.push_str("### Entity Details\n\n");

        for entity in entities {
            let type_badge = match entity.entity_type {
                EntityType::Model => "📦 Model",
                EntityType::Enum => "🔢 Enum",
                EntityType::ValueObject => "💎 Value Object",
                EntityType::Aggregate => "🏛️ Aggregate",
                EntityType::View => "👁️ View",
            };

            output.push_str(&format!("#### {} `{}`\n\n", type_badge, entity.name));
            output.push_str(&format!("**File**: `{}`:{}\n\n", entity.file, entity.line));

            if !entity.fields.is_empty() {
                output.push_str("| Field | Type | Nullable | Constraints |\n");
                output.push_str("|-------|------|----------|-------------|\n");

                for field in &entity.fields {
                    let mut constraints = Vec::new();
                    if Some(&field.name) == entity.primary_key.as_ref() {
                        constraints.push("PK");
                    }
                    if field.unique {
                        constraints.push("Unique");
                    }
                    if field.indexed {
                        constraints.push("Indexed");
                    }

                    output.push_str(&format!(
                        "| `{}` | `{}` | {} | {} |\n",
                        field.name,
                        field.field_type,
                        if field.nullable { "Yes" } else { "No" },
                        constraints.join(", ")
                    ));
                }
                output.push('\n');
            }

            if !entity.validations.is_empty() {
                output.push_str("**Validations**:\n");
                for validation in &entity.validations {
                    output.push_str(&format!(
                        "- `{}`: {} ({})\n",
                        validation.field,
                        validation.rule_type,
                        validation.parameters
                    ));
                }
                output.push('\n');
            }
        }

        output
    }

    fn generate_workflows(workflows: &[Workflow]) -> String {
        let mut output = String::new();

        output.push_str("## Workflows & State Machines\n\n");

        for workflow in workflows {
            output.push_str(&format!("### {}\n\n", workflow.name));
            output.push_str(&format!("**File**: `{}`:{}\n\n", workflow.file, workflow.line));

            if !workflow.states.is_empty() {
                // State diagram in Mermaid
                output.push_str("```mermaid\n");
                output.push_str("stateDiagram-v2\n");

                // Initial state
                if let Some(initial) = &workflow.initial_state {
                    output.push_str(&format!("    [*] --> {}\n", initial));
                }

                // States
                for state in &workflow.states {
                    if state.is_terminal {
                        output.push_str(&format!("    {} --> [*]\n", state.name));
                    }
                }

                // Transitions
                for transition in &workflow.transitions {
                    let label = transition.trigger.as_deref().unwrap_or("");
                    if label.is_empty() {
                        output.push_str(&format!(
                            "    {} --> {}\n",
                            transition.from_state,
                            transition.to_state
                        ));
                    } else {
                        output.push_str(&format!(
                            "    {} --> {} : {}\n",
                            transition.from_state,
                            transition.to_state,
                            label
                        ));
                    }
                }

                output.push_str("```\n\n");

                // States table
                output.push_str("#### States\n\n");
                output.push_str("| State | Type | Description |\n");
                output.push_str("|-------|------|-------------|\n");

                for state in &workflow.states {
                    let state_type = if state.is_initial {
                        "🟢 Initial"
                    } else if state.is_terminal {
                        "🔴 Terminal"
                    } else {
                        "⚪ Normal"
                    };

                    output.push_str(&format!(
                        "| `{}` | {} | {} |\n",
                        state.name,
                        state_type,
                        state.description.as_deref().unwrap_or("-")
                    ));
                }
                output.push('\n');
            }
        }

        output
    }

    fn generate_rules(rules: &[BusinessRule]) -> String {
        let mut output = String::new();

        output.push_str("## Business Rules\n\n");

        // Group by severity
        let critical: Vec<_> = rules.iter()
            .filter(|r| matches!(r.severity, RuleSeverity::Critical))
            .collect();
        let high: Vec<_> = rules.iter()
            .filter(|r| matches!(r.severity, RuleSeverity::High))
            .collect();
        let other: Vec<_> = rules.iter()
            .filter(|r| !matches!(r.severity, RuleSeverity::Critical | RuleSeverity::High))
            .collect();

        if !critical.is_empty() {
            output.push_str("### 🚨 Critical Rules\n\n");
            for rule in critical {
                output.push_str(&Self::format_rule(rule));
            }
        }

        if !high.is_empty() {
            output.push_str("### ⚠️ High Priority Rules\n\n");
            for rule in high {
                output.push_str(&Self::format_rule(rule));
            }
        }

        if !other.is_empty() {
            output.push_str("### 📋 Other Rules\n\n");
            output.push_str("| Rule | Type | Description | Location |\n");
            output.push_str("|------|------|-------------|----------|\n");
            for rule in other {
                let rule_type = match rule.rule_type {
                    BusinessRuleType::Validation => "Validation",
                    BusinessRuleType::Authorization => "Auth",
                    BusinessRuleType::Calculation => "Calc",
                    BusinessRuleType::Constraint => "Constraint",
                    BusinessRuleType::Trigger => "Trigger",
                    BusinessRuleType::Policy => "Policy",
                };
                output.push_str(&format!(
                    "| {} | {} | {} | `{}`:{} |\n",
                    rule.name,
                    rule_type,
                    rule.description,
                    rule.file,
                    rule.line
                ));
            }
            output.push('\n');
        }

        output
    }

    fn format_rule(rule: &BusinessRule) -> String {
        let mut output = String::new();

        let rule_type = match rule.rule_type {
            BusinessRuleType::Validation => "Validation",
            BusinessRuleType::Authorization => "Authorization",
            BusinessRuleType::Calculation => "Calculation",
            BusinessRuleType::Constraint => "Constraint",
            BusinessRuleType::Trigger => "Trigger",
            BusinessRuleType::Policy => "Policy",
        };

        output.push_str(&format!("#### {}\n\n", rule.name));
        output.push_str(&format!("- **Type**: {}\n", rule_type));
        output.push_str(&format!("- **Description**: {}\n", rule.description));
        output.push_str(&format!("- **Condition**: `{}`\n", rule.condition));
        
        if let Some(action) = &rule.action {
            output.push_str(&format!("- **Action**: {}\n", action));
        }
        
        output.push_str(&format!("- **Location**: `{}`:{}\n\n", rule.file, rule.line));

        output
    }

    fn generate_integrations(integrations: &[Integration]) -> String {
        let mut output = String::new();

        output.push_str("## External Integrations\n\n");

        // Group by type
        let mut by_type: std::collections::HashMap<&str, Vec<&Integration>> = std::collections::HashMap::new();
        
        for integration in integrations {
            let type_name = match integration.integration_type {
                IntegrationType::Payment => "Payment",
                IntegrationType::Email => "Email",
                IntegrationType::Authentication => "Authentication",
                IntegrationType::Storage => "Storage",
                IntegrationType::Analytics => "Analytics",
                IntegrationType::Messaging => "Messaging",
                IntegrationType::Database => "Database",
                IntegrationType::Cache => "Cache",
                IntegrationType::Search => "Search",
                IntegrationType::Other => "Other",
            };
            by_type.entry(type_name).or_default().push(integration);
        }

        for (type_name, integrations) in by_type.iter() {
            output.push_str(&format!("### {} Services\n\n", type_name));
            
            for integration in integrations.iter() {
                output.push_str(&format!("#### {}\n\n", integration.name));
                output.push_str(&format!("- **Purpose**: {}\n", integration.purpose));
                output.push_str(&format!("- **File**: `{}`\n", integration.file));
                
                if let Some(auth) = &integration.auth_method {
                    output.push_str(&format!("- **Auth**: {}\n", auth));
                }

                if !integration.endpoints.is_empty() {
                    output.push_str("- **Endpoints**:\n");
                    for endpoint in &integration.endpoints {
                        output.push_str(&format!("  - `{}`\n", endpoint));
                    }
                }
                output.push('\n');
            }
        }

        output
    }
}
