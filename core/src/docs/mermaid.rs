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

/// Generate a high-level architecture diagram using Layered Architecture pattern.
pub fn generate_architecture_diagram(snapshot: &CodebaseSnapshot) -> String {
    let mut mermaid = String::from("```mermaid\ngraph TD\n");

    // Enhanced styling
    mermaid.push_str("    classDef layerC fill:#1e293b,stroke:#475569,color:#cbd5e1,stroke-width:2px,stroke-dasharray: 5 5\n");
    mermaid.push_str("    classDef presentation fill:#0891b2,stroke:#0e7490,color:#fff,rx:5,ry:5\n");
    mermaid.push_str("    classDef interface fill:#7c3aed,stroke:#6d28d9,color:#fff,rx:5,ry:5\n");
    mermaid.push_str("    classDef business fill:#059669,stroke:#047857,color:#fff,rx:5,ry:5\n");
    mermaid.push_str("    classDef data fill:#b91c1c,stroke:#991b1b,color:#fff,rx:5,ry:5\n");
    mermaid.push_str("    classDef common fill:#475569,stroke:#334155,color:#fff,rx:5,ry:5\n");
    mermaid.push_str("    classDef service fill:#db2777,stroke:#be185d,color:#fff,shape:hexagon\n\n");

    // 1. Identify Layers & Group Directories
    let mut layer_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    let mut dir_deps: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    
    // Helper to classify directories into layers
    fn classify_layer(name: &str) -> &'static str {
        match name.to_lowercase().as_str() {
            "ui" | "frontend" | "web" | "pages" | "view" | "views" | "components" | "app" | "client" | "cli" => "Presentation",
            "api" | "server" | "controllers" | "routes" | "handlers" | "graphql" | "grpc" | "interfaces" | "gateway" => "Interface",
            "core" | "domain" | "business" | "services" | "logic" | "usecases" | "workflows" | "jobs" | "workers" | "orchestrator" => "Business",
            "data" | "models" | "db" | "database" | "repositories" | "store" | "sql" | "entities" | "migrations" => "Data",
            "utils" | "common" | "lib" | "shared" | "helpers" | "config" | "constants" | "types" | "infrastructure" | "infra" => "Common",
            _ => "Business", // Default to Business for unrecognized top-level folders
        }
    }

    // Collect Directories and organize into layers
    let mut dir_to_layer: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for module in &snapshot.modules {
        let path_parts: Vec<&str> = module.path.split(&['/', '\\'][..]).collect();
        let top_dir = if !path_parts.is_empty() { path_parts[0] } else { "root" };
        
        let layer = classify_layer(top_dir);
        let dir_key = top_dir.to_string();

        dir_to_layer.insert(dir_key.clone(), layer.to_string());
        
        // Add unique directories to the layer map
        let entry = layer_map.entry(layer.to_string()).or_default();
        if !entry.contains(&dir_key) {
            entry.push(dir_key.clone());
        }

        // Collect cross-directory dependencies
        // (Similar to previous logic but simpler)
        for dep in &module.dependencies {
             if let Some(target_module) = snapshot.modules.iter().find(|m| m.name == dep.target) {
                 let target_parts: Vec<&str> = target_module.path.split(&['/', '\\'][..]).collect();
                 let target_dir = if !target_parts.is_empty() { target_parts[0] } else { "root" };
                 
                 if top_dir != target_dir {
                     dir_deps.insert((top_dir.to_string(), target_dir.to_string()));
                 }
             }
        }
    }

    // 2. Render Layers in Order (Top Down)
    let ordered_layers = vec!["Presentation", "Interface", "Business", "Data", "Common"];
    
    for layer in ordered_layers {
        if let Some(dirs) = layer_map.get(layer) {
            mermaid.push_str(&format!("    subgraph {}_Layer[\" \"]\n", layer));
            mermaid.push_str("        direction LR\n"); // Items inside a layer flow Left-Right
            
            for dir in dirs {
                let id = sanitize_id(dir);
                let label = match layer {
                    "Presentation" => format!("🖥️ {}", dir),
                    "Interface" => format!("🔌 {}", dir),
                    "Business" => format!("⚙️ {}", dir),
                    "Data" => format!("💾 {}", dir),
                    "Common" => format!("📚 {}", dir),
                    _ => dir.clone(),
                };
                
                // Styling based on layer
                let style_class = layer.to_lowercase();
                mermaid.push_str(&format!("        {}[\"{}\"]\n", id, label));
                mermaid.push_str(&format!("        class {} {}\n", id, style_class));
            }
            mermaid.push_str("    end\n");
            // Style the container
            mermaid.push_str(&format!("    class {}_Layer layerC\n\n", layer));
        }
    }

    // 3. Render Services Entry Points (Separate from layers or attached)
    if !snapshot.services.is_empty() {
        for service in &snapshot.services {
             // Heuristic: Connect service to the dir containing its entry file
             let entry = service.entry_file.to_string_lossy();
             let parts: Vec<&str> = entry.split(&['/', '\\'][..]).collect();
             if !parts.is_empty() {
                 let dir = parts[0];
                 let service_id = sanitize_id(&service.name);
                 mermaid.push_str(&format!("    {}[\"🚀 {}\"]\n", service_id, service.name));
                 mermaid.push_str(&format!("    class {} service\n", service_id));
                 mermaid.push_str(&format!("    {} --> {}\n", service_id, sanitize_id(dir)));
             }
        }
    }

    // 4. Render Aggregated Edges (Layer to Layer mostly)
    for (src, dst) in dir_deps {
        let src_id = sanitize_id(&src);
        let dst_id = sanitize_id(&dst);
        
        let src_layer = dir_to_layer.get(&src).map(|s| s.as_str()).unwrap_or("Other");
        let dst_layer = dir_to_layer.get(&dst).map(|s| s.as_str()).unwrap_or("Other");

        // Filter: Don't show Common -> * dependencies (too noisy), only * -> Common
        if src_layer == "Common" { continue; }

        mermaid.push_str(&format!("    {} --> {}\n", src_id, dst_id));
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
