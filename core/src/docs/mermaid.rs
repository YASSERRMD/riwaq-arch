//! Enhanced Mermaid diagram generation utilities.
//!
//! This module provides functions to generate professional-quality Mermaid diagrams:
//! - Architecture diagrams
//! - Component diagrams  
//! - Sequence diagrams
//! - State diagrams
//! - Class diagrams
//! - ER diagrams

use crate::models::module::DependencyGraph;
use crate::models::snapshot::CodebaseSnapshot;
use crate::analysis::business::{Workflow, Entity, EntityRelationship, RelationshipType};

/// Generate a module dependency diagram in Mermaid syntax.
pub fn generate_dependency_diagram(graph: &DependencyGraph) -> String {
    let mut mermaid = String::from("```mermaid\ngraph TD\n");
    
    // Add styling
    mermaid.push_str("    classDef module fill:#4f46e5,stroke:#3730a3,color:#fff\n");
    mermaid.push_str("    classDef circular fill:#ef4444,stroke:#dc2626,color:#fff\n\n");

    // Generate nodes and edges
    for (source, edges) in &graph.edges {
        let source_id = sanitize_id(source);
        
        for edge in edges {
            let target_id = sanitize_id(&edge.target);
            let label = if edge.weight > 1 {
                format!(" -- {} refs -->", edge.weight)
            } else {
                " -->".to_string()
            };
            
            mermaid.push_str(&format!("    {}[{}]{} {}[{}]\n", 
                source_id, source, label, target_id, &edge.target));
        }
    }

    // Highlight circular dependencies
    for cycle in &graph.circular_deps {
        if cycle.len() >= 2 {
            mermaid.push_str(&format!(
                "    class {} circular\n",
                sanitize_id(&cycle[0])
            ));
        }
    }

    mermaid.push_str("```\n");
    mermaid
}

/// Generate a high-level architecture diagram with enhanced styling.
pub fn generate_architecture_diagram(snapshot: &CodebaseSnapshot) -> String {
    let mut mermaid = String::from("```mermaid\ngraph TB\n");

    // Enhanced styling
    mermaid.push_str("    classDef service fill:#7c3aed,stroke:#5b21b6,color:#fff,stroke-width:2px\n");
    mermaid.push_str("    classDef module fill:#4f46e5,stroke:#3730a3,color:#fff\n");
    mermaid.push_str("    classDef external fill:#6366f1,stroke:#4f46e5,color:#fff,stroke-dasharray: 5 5\n\n");

    // Group modules by top-level namespace
    let mut namespaces: std::collections::HashMap<String, Vec<&str>> = std::collections::HashMap::new();

    for module in &snapshot.modules {
        let namespace = module.path.split(&[':', '.', '/'][..])
            .next()
            .unwrap_or(&module.name);
        
        namespaces
            .entry(namespace.to_string())
            .or_default()
            .push(&module.name);
    }

    // Add service entry points first
    if !snapshot.services.is_empty() {
        mermaid.push_str("    subgraph SERVICES[\"🚀 Services\"]\n");
        mermaid.push_str("        direction TB\n");
        for service in &snapshot.services {
            let icon = match service.kind {
                crate::models::snapshot::ServiceKind::HttpServer => "🌐",
                crate::models::snapshot::ServiceKind::GrpcServer => "⚡",
                crate::models::snapshot::ServiceKind::Cli => "💻",
                _ => "📦",
            };
            mermaid.push_str(&format!("        {}[\"{} {}\"]\n", 
                sanitize_id(&service.name), icon, service.name));
        }
        mermaid.push_str("    end\n\n");
    }

    // Create subgraphs for each namespace
    for (namespace, modules) in &namespaces {
        if modules.len() > 1 {
            mermaid.push_str(&format!("    subgraph {}[\"📦 {}\"]\n", sanitize_id(namespace), namespace));
            mermaid.push_str("        direction TB\n");
            for module in modules.iter().take(10) {
                mermaid.push_str(&format!("        {}[\"{}\"]\n", sanitize_id(module), module));
            }
            if modules.len() > 10 {
                mermaid.push_str(&format!("        {}more[\"... +{} more\"]\n", sanitize_id(namespace), modules.len() - 10));
            }
            mermaid.push_str("    end\n");
        } else if !modules.is_empty() {
            mermaid.push_str(&format!("    {}[\"{}\"]\n", sanitize_id(modules[0]), modules[0]));
        }
    }

    // Apply styles
    mermaid.push('\n');
    for service in &snapshot.services {
        mermaid.push_str(&format!("    class {} service\n", sanitize_id(&service.name)));
    }

    mermaid.push_str("```\n");
    mermaid
}

/// Generate a data flow diagram.
pub fn generate_dataflow_diagram(snapshot: &CodebaseSnapshot) -> String {
    let mut mermaid = String::from("```mermaid\nflowchart LR\n");

    // Enhanced styling
    mermaid.push_str("    classDef service fill:#7c3aed,stroke:#5b21b6,color:#fff\n");
    mermaid.push_str("    classDef data fill:#10b981,stroke:#059669,color:#fff\n");
    mermaid.push_str("    classDef external fill:#f59e0b,stroke:#d97706,color:#fff\n\n");

    // Extract main entry points and their dependencies
    for service in &snapshot.services {
        let service_id = sanitize_id(&service.name);
        mermaid.push_str(&format!("    {}((\"{}\"))\n", service_id, service.name));

        // Find modules this service depends on
        for module in &snapshot.modules {
            if service.entry_file.to_string_lossy().contains(&module.name) {
                let module_id = sanitize_id(&module.name);
                mermaid.push_str(&format!("    {} --> {}\n", service_id, module_id));
            }
        }
    }

    // Apply styles
    mermaid.push('\n');
    for service in &snapshot.services {
        mermaid.push_str(&format!("    class {} service\n", sanitize_id(&service.name)));
    }

    mermaid.push_str("```\n");
    mermaid
}

/// Generate a class/type diagram for a module.
pub fn generate_type_diagram(snapshot: &CodebaseSnapshot, module_name: &str) -> String {
    let mut mermaid = String::from("```mermaid\nclassDiagram\n");

    // Find files in this module
    for file in &snapshot.files {
        if file.path.to_string_lossy().contains(module_name) {
            for typ in &file.types {
                mermaid.push_str(&format!("    class {} {{\n", sanitize_id(&typ.name)));
                
                // Add fields
                for field in &typ.fields {
                    let visibility = match field.visibility {
                        crate::models::file::Visibility::Public => "+",
                        crate::models::file::Visibility::Private => "-",
                        _ => "#",
                    };
                    let field_type = field.type_annotation.as_deref().unwrap_or("unknown");
                    mermaid.push_str(&format!("        {}{}: {}\n", visibility, field.name, field_type));
                }
                
                // Add methods
                for method in &typ.methods {
                    let visibility = match method.visibility {
                        crate::models::file::Visibility::Public => "+",
                        crate::models::file::Visibility::Private => "-",
                        _ => "#",
                    };
                    mermaid.push_str(&format!("        {}{}()\n", visibility, method.name));
                }
                
                mermaid.push_str("    }\n");
            }
        }
    }

    mermaid.push_str("```\n");
    mermaid
}

/// Generate a state diagram from a workflow.
pub fn generate_state_diagram(workflow: &Workflow) -> String {
    let mut mermaid = String::from("```mermaid\nstateDiagram-v2\n");

    // Add title
    mermaid.push_str(&format!("    [*] --> {}\n", 
        workflow.initial_state.as_deref().unwrap_or("Start")));

    // Add states with descriptions
    for state in &workflow.states {
        if let Some(desc) = &state.description {
            mermaid.push_str(&format!("    {} : {}\n", state.name, desc));
        }
    }

    // Add transitions
    for transition in &workflow.transitions {
        if let Some(trigger) = &transition.trigger {
            mermaid.push_str(&format!("    {} --> {} : {}\n", 
                transition.from_state, transition.to_state, trigger));
        } else {
            mermaid.push_str(&format!("    {} --> {}\n", 
                transition.from_state, transition.to_state));
        }
    }

    // Add terminal states
    for terminal in &workflow.terminal_states {
        mermaid.push_str(&format!("    {} --> [*]\n", terminal));
    }

    mermaid.push_str("```\n");
    mermaid
}

/// Generate an ER diagram from entities.
pub fn generate_er_diagram(entities: &[Entity], relationships: &[EntityRelationship]) -> String {
    let mut mermaid = String::from("```mermaid\nerDiagram\n");

    // Add entities
    for entity in entities {
        mermaid.push_str(&format!("    {} {{\n", sanitize_id(&entity.name)));
        for field in &entity.fields {
            let pk = if Some(&field.name) == entity.primary_key.as_ref() { " PK" } else { "" };
            let fk = if field.name.ends_with("_id") && field.name != "id" { " FK" } else { "" };
            mermaid.push_str(&format!("        {} {}{}{}\n", 
                sanitize_id(&field.field_type), 
                sanitize_id(&field.name),
                pk,
                fk));
        }
        mermaid.push_str("    }\n");
    }

    // Add relationships
    for rel in relationships {
        let cardinality = match rel.relationship_type {
            RelationshipType::OneToOne => "||--||",
            RelationshipType::OneToMany => "||--o{",
            RelationshipType::ManyToOne => "}o--||",
            RelationshipType::ManyToMany => "}o--o{",
        };
        mermaid.push_str(&format!("    {} {} {} : \"{}\"\n", 
            sanitize_id(&rel.from_entity),
            cardinality,
            sanitize_id(&rel.to_entity),
            rel.field_name));
    }

    mermaid.push_str("```\n");
    mermaid
}

/// Generate a sequence diagram for a flow.
pub fn generate_sequence_diagram(
    title: &str,
    actors: &[(&str, &str)],  // (id, label)
    messages: &[(&str, &str, &str)],  // (from, to, message)
) -> String {
    let mut mermaid = String::from("```mermaid\nsequenceDiagram\n");

    // Add title
    mermaid.push_str(&format!("    title {}\n", title));

    // Add participants
    for (id, label) in actors {
        mermaid.push_str(&format!("    participant {} as {}\n", id, label));
    }

    mermaid.push('\n');

    // Add messages
    for (from, to, message) in messages {
        mermaid.push_str(&format!("    {} ->> {}: {}\n", from, to, message));
    }

    mermaid.push_str("```\n");
    mermaid
}

/// Generate a component diagram.
pub fn generate_component_diagram(
    components: &[(&str, &[&str])],  // (component_name, dependencies)
) -> String {
    let mut mermaid = String::from("```mermaid\nflowchart TB\n");

    // Styling
    mermaid.push_str("    classDef component fill:#4f46e5,stroke:#3730a3,color:#fff,stroke-width:2px\n");
    mermaid.push_str("    classDef interface fill:#10b981,stroke:#059669,color:#fff\n\n");

    // Add components
    for (name, deps) in components {
        let id = sanitize_id(name);
        mermaid.push_str(&format!("    {}[\"📦 {}\"]\n", id, name));
        
        for dep in *deps {
            let dep_id = sanitize_id(dep);
            mermaid.push_str(&format!("    {} --> {}\n", id, dep_id));
        }
    }

    mermaid.push_str("```\n");
    mermaid
}

/// Sanitize a name for use as a Mermaid node ID.
pub fn sanitize_id(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_id() {
        assert_eq!(sanitize_id("hello::world"), "hello__world");
        assert_eq!(sanitize_id("my-module"), "my_module");
        assert_eq!(sanitize_id("MyClass"), "MyClass");
    }

    #[test]
    fn test_dependency_diagram() {
        let mut graph = DependencyGraph::new();
        graph.add_node("module_a".to_string());
        graph.add_node("module_b".to_string());
        graph.add_edge("module_a", "module_b", crate::models::module::DependencyKind::Function);

        let diagram = generate_dependency_diagram(&graph);
        assert!(diagram.contains("mermaid"));
        assert!(diagram.contains("module_a"));
        assert!(diagram.contains("module_b"));
    }

    #[test]
    fn test_sequence_diagram() {
        let actors = vec![
            ("client", "Client"),
            ("server", "Server"),
            ("db", "Database"),
        ];
        let messages = vec![
            ("client", "server", "Request"),
            ("server", "db", "Query"),
            ("db", "server", "Result"),
            ("server", "client", "Response"),
        ];
        
        let diagram = generate_sequence_diagram("API Flow", &actors, &messages);
        assert!(diagram.contains("sequenceDiagram"));
        assert!(diagram.contains("API Flow"));
    }
}
