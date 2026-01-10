//! Mermaid diagram generation utilities.
//!
//! This module provides functions to generate Mermaid diagrams
//! for visualizing architecture, dependencies, and data flow.

use crate::models::module::DependencyGraph;
use crate::models::snapshot::CodebaseSnapshot;

/// Generate a module dependency diagram in Mermaid syntax.
pub fn generate_dependency_diagram(graph: &DependencyGraph) -> String {
    let mut mermaid = String::from("```mermaid\ngraph TD\n");

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
                "    style {} fill:#f96\n",
                sanitize_id(&cycle[0])
            ));
        }
    }

    mermaid.push_str("```\n");
    mermaid
}

/// Generate a high-level architecture diagram.
pub fn generate_architecture_diagram(snapshot: &CodebaseSnapshot) -> String {
    let mut mermaid = String::from("```mermaid\ngraph TB\n");

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

    // Create subgraphs for each namespace
    for (namespace, modules) in &namespaces {
        if modules.len() > 1 {
            mermaid.push_str(&format!("    subgraph {} [{}]\n", sanitize_id(namespace), namespace));
            for module in modules {
                mermaid.push_str(&format!("        {}[{}]\n", sanitize_id(module), module));
            }
            mermaid.push_str("    end\n");
        } else if !modules.is_empty() {
            mermaid.push_str(&format!("    {}[{}]\n", sanitize_id(modules[0]), modules[0]));
        }
    }

    // Add service entry points
    if !snapshot.services.is_empty() {
        mermaid.push_str("    subgraph Services\n");
        for service in &snapshot.services {
            let icon = match service.kind {
                crate::models::snapshot::ServiceKind::HttpServer => "🌐",
                crate::models::snapshot::ServiceKind::GrpcServer => "⚡",
                crate::models::snapshot::ServiceKind::Cli => "💻",
                _ => "📦",
            };
            mermaid.push_str(&format!("        {}[{} {}]\n", 
                sanitize_id(&service.name), icon, service.name));
        }
        mermaid.push_str("    end\n");
    }

    mermaid.push_str("```\n");
    mermaid
}

/// Generate a data flow diagram.
pub fn generate_dataflow_diagram(snapshot: &CodebaseSnapshot) -> String {
    let mut mermaid = String::from("```mermaid\nflowchart LR\n");

    // Extract main entry points and their dependencies
    for service in &snapshot.services {
        let service_id = sanitize_id(&service.name);
        mermaid.push_str(&format!("    {}(({})):::service\n", service_id, service.name));

        // Find modules this service depends on
        for module in &snapshot.modules {
            if service.entry_file.to_string_lossy().contains(&module.name) {
                let module_id = sanitize_id(&module.name);
                mermaid.push_str(&format!("    {} --> {}\n", service_id, module_id));
            }
        }
    }

    // Add style classes
    mermaid.push_str("    classDef service fill:#f9f,stroke:#333\n");
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

/// Sanitize a name for use as a Mermaid node ID.
fn sanitize_id(name: &str) -> String {
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
}
