//! Module-level data models.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Summary of a module or package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleSummary {
    /// Module name.
    pub name: String,

    /// Full module path (e.g., "myproject::utils::helpers").
    pub path: String,

    /// Source file paths that belong to this module.
    pub files: Vec<PathBuf>,

    /// Sub-modules.
    pub submodules: Vec<String>,

    /// Description (from doc comments or generated).
    pub description: Option<String>,

    /// Public functions exported by this module.
    pub public_functions: Vec<String>,

    /// Public types exported by this module.
    pub public_types: Vec<String>,

    /// Dependencies on other modules.
    pub dependencies: Vec<ModuleDependency>,

    /// Metrics for this module.
    pub metrics: ModuleMetrics,
}

impl ModuleSummary {
    /// Create a new module summary.
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            path,
            files: Vec::new(),
            submodules: Vec::new(),
            description: None,
            public_functions: Vec::new(),
            public_types: Vec::new(),
            dependencies: Vec::new(),
            metrics: ModuleMetrics::default(),
        }
    }

    /// Check if this is a leaf module (no submodules).
    pub fn is_leaf(&self) -> bool {
        self.submodules.is_empty()
    }

    /// Get the number of public items.
    pub fn public_item_count(&self) -> usize {
        self.public_functions.len() + self.public_types.len()
    }
}

/// Dependency on another module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDependency {
    /// Target module path.
    pub target: String,

    /// Kind of dependency.
    pub kind: DependencyKind,

    /// Number of references.
    pub reference_count: usize,
}

/// Kind of dependency between modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DependencyKind {
    /// Type usage.
    Type,
    /// Function call.
    Function,
    /// Trait implementation.
    Trait,
    /// Module re-export.
    Reexport,
    /// Unknown dependency.
    Unknown,
}

/// Metrics for a module.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModuleMetrics {
    /// Total lines of code.
    pub lines_of_code: usize,

    /// Number of files.
    pub file_count: usize,

    /// Number of functions.
    pub function_count: usize,

    /// Number of types.
    pub type_count: usize,

    /// Average function complexity.
    pub avg_complexity: f32,

    /// Coupling score (0-1, higher = more coupled).
    pub coupling_score: f32,

    /// Cohesion score (0-1, higher = more cohesive).
    pub cohesion_score: f32,
}

impl ModuleMetrics {
    /// Check if the module has high coupling.
    pub fn has_high_coupling(&self) -> bool {
        self.coupling_score > 0.7
    }

    /// Check if the module has low cohesion.
    pub fn has_low_cohesion(&self) -> bool {
        self.cohesion_score < 0.3
    }
}

/// Dependency graph representing relationships between modules.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependencyGraph {
    /// Nodes in the graph (module paths).
    pub nodes: Vec<String>,

    /// Edges in the graph (from -> [to]).
    pub edges: HashMap<String, Vec<DependencyEdge>>,

    /// Circular dependencies detected.
    pub circular_deps: Vec<Vec<String>>,
}

/// Edge in the dependency graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    /// Target module.
    pub target: String,

    /// Weight (number of references).
    pub weight: usize,

    /// Kind of dependency.
    pub kind: DependencyKind,
}

impl DependencyGraph {
    /// Create a new empty dependency graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a node to the graph.
    pub fn add_node(&mut self, node: String) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node.clone());
            self.edges.insert(node, Vec::new());
        }
    }

    /// Add an edge to the graph.
    pub fn add_edge(&mut self, from: &str, to: &str, kind: DependencyKind) {
        if let Some(edges) = self.edges.get_mut(from) {
            if let Some(edge) = edges.iter_mut().find(|e| e.target == to) {
                edge.weight += 1;
            } else {
                edges.push(DependencyEdge {
                    target: to.to_string(),
                    weight: 1,
                    kind,
                });
            }
        }
    }

    /// Get dependencies for a module.
    pub fn get_dependencies(&self, module: &str) -> Vec<&DependencyEdge> {
        self.edges
            .get(module)
            .map(|edges| edges.iter().collect())
            .unwrap_or_default()
    }

    /// Get dependents of a module (modules that depend on it).
    pub fn get_dependents(&self, module: &str) -> Vec<(&str, &DependencyEdge)> {
        let mut dependents = Vec::new();
        for (from, edges) in &self.edges {
            for edge in edges {
                if edge.target == module {
                    dependents.push((from.as_str(), edge));
                }
            }
        }
        dependents
    }

    /// Find all circular dependencies using Tarjan's algorithm.
    pub fn find_circular_dependencies(&mut self) {
        // Simple implementation: DFS to find back edges
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = Vec::new();
        let mut cycles = Vec::new();

        for node in &self.nodes.clone() {
            if !visited.contains(node) {
                self.dfs_find_cycles(node, &mut visited, &mut rec_stack, &mut cycles);
            }
        }

        self.circular_deps = cycles;
    }

    fn dfs_find_cycles(
        &self,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string());
        rec_stack.push(node.to_string());

        if let Some(edges) = self.edges.get(node) {
            for edge in edges {
                if !visited.contains(&edge.target) {
                    self.dfs_find_cycles(&edge.target, visited, rec_stack, cycles);
                } else if rec_stack.contains(&edge.target) {
                    // Found a cycle
                    let start_idx = rec_stack.iter().position(|n| n == &edge.target).unwrap();
                    let cycle: Vec<String> = rec_stack[start_idx..].to_vec();
                    if !cycles.iter().any(|c| c == &cycle) {
                        cycles.push(cycle);
                    }
                }
            }
        }

        rec_stack.pop();
    }

    /// Generate a Mermaid diagram for the dependency graph.
    pub fn to_mermaid(&self) -> String {
        let mut output = String::from("graph LR\n");

        for (from, edges) in &self.edges {
            for edge in edges {
                let arrow = match edge.kind {
                    DependencyKind::Type => "-->",
                    DependencyKind::Function => "-..->",
                    DependencyKind::Trait => "===>",
                    _ => "-->",
                };
                output.push_str(&format!(
                    "    {}[{}] {} {}[{}]\n",
                    sanitize_mermaid_id(from),
                    from,
                    arrow,
                    sanitize_mermaid_id(&edge.target),
                    edge.target
                ));
            }
        }

        output
    }
}

fn sanitize_mermaid_id(s: &str) -> String {
    s.replace("::", "_").replace('-', "_").replace('.', "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_summary_creation() {
        let module = ModuleSummary::new("utils".to_string(), "myproject::utils".to_string());
        assert!(module.is_leaf());
        assert_eq!(module.public_item_count(), 0);
    }

    #[test]
    fn test_dependency_graph_add_edge() {
        let mut graph = DependencyGraph::new();
        graph.add_node("module_a".to_string());
        graph.add_node("module_b".to_string());
        graph.add_edge("module_a", "module_b", DependencyKind::Function);

        let deps = graph.get_dependencies("module_a");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].target, "module_b");
    }

    #[test]
    fn test_find_circular_dependencies() {
        let mut graph = DependencyGraph::new();
        graph.add_node("a".to_string());
        graph.add_node("b".to_string());
        graph.add_node("c".to_string());
        graph.add_edge("a", "b", DependencyKind::Function);
        graph.add_edge("b", "c", DependencyKind::Function);
        graph.add_edge("c", "a", DependencyKind::Function);

        graph.find_circular_dependencies();
        assert_eq!(graph.circular_deps.len(), 1);
    }
}
