//! Workflow & State Machine Analyzer
//!
//! Detects state machines and workflows:
//! - Enum-based state machines
//! - Status fields with transitions
//! - FSM patterns
//! - Conditional logic flows

use super::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use tokio::fs;
use walkdir::WalkDir;

pub struct WorkflowAnalyzer;

impl WorkflowAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub async fn analyze(&self, root_path: &Path) -> anyhow::Result<Vec<Workflow>> {
        let mut workflows = Vec::new();

        for entry in WalkDir::new(root_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if !Self::is_source_file(path) || Self::should_skip_dir(path) {
                continue;
            }

            if let Ok(content) = fs::read_to_string(path).await {
                let relative_path = path.strip_prefix(root_path)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string();
                
                let found = self.detect_workflows(&content, &relative_path);
                workflows.extend(found);
            }
        }

        Ok(workflows)
    }

    fn detect_workflows(&self, content: &str, file: &str) -> Vec<Workflow> {
        let mut workflows = Vec::new();

        // Detect status/state enums
        workflows.extend(self.detect_status_enums(content, file));
        
        // Detect FSM patterns
        workflows.extend(self.detect_fsm_patterns(content, file));
        
        // Detect transition functions
        workflows.extend(self.detect_transitions(content, file));

        workflows
    }

    fn detect_status_enums(&self, content: &str, file: &str) -> Vec<Workflow> {
        let mut workflows = Vec::new();

        // Match enums with status/state-like names
        let patterns = vec![
            r#"(?s)(?:pub\s+)?enum\s+(\w*(?:Status|State|Phase|Stage)\w*)\s*\{([^}]+)\}"#,
            r#"(?s)class\s+(\w*(?:Status|State|Phase|Stage)\w*)\s*\(.*Enum.*\)\s*:\s*((?:[^\n]|\n(?!\s*class\s))+)"#,
            r#"(?s)enum\s+(\w*(?:Status|State|Phase|Stage)\w*)\s*\{([^}]+)\}"#,
        ];

        for pattern in patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for cap in regex.captures_iter(content) {
                    let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
                    let body = cap.get(2).map(|m| m.as_str()).unwrap_or("");
                    
                    let line = content[..cap.get(0).unwrap().start()]
                        .lines().count() + 1;
                    
                    let states = self.parse_enum_states(body);
                    let transitions = self.infer_transitions(&states, content);
                    
                    let initial_state = self.detect_initial_state(&states, content);
                    let terminal_states = self.detect_terminal_states(&states);

                    if states.len() >= 2 {
                        workflows.push(Workflow {
                            name,
                            entity: None,
                            states,
                            transitions,
                            initial_state,
                            terminal_states,
                            file: file.to_string(),
                            line,
                        });
                    }
                }
            }
        }

        workflows
    }

    fn parse_enum_states(&self, body: &str) -> Vec<WorkflowState> {
        let mut states = Vec::new();
        
        // Parse variants/values
        for line in body.lines() {
            let trimmed = line.trim()
                .trim_end_matches(',')
                .trim_end_matches(')')
                .split('(').next()
                .unwrap_or("")
                .split('=').next()
                .unwrap_or("")
                .trim();
            
            if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with('#') {
                continue;
            }

            let is_initial = Self::is_initial_state_name(trimmed);
            let is_terminal = Self::is_terminal_state_name(trimmed);

            states.push(WorkflowState {
                name: trimmed.to_string(),
                description: None,
                is_initial,
                is_terminal,
                entry_actions: Vec::new(),
                exit_actions: Vec::new(),
            });
        }

        states
    }

    fn infer_transitions(&self, states: &[WorkflowState], content: &str) -> Vec<StateTransition> {
        let mut transitions = Vec::new();
        let state_names: HashSet<&str> = states.iter().map(|s| s.name.as_str()).collect();

        // Look for transition patterns in code
        let transition_patterns = vec![
            // Match: status = Status::Processing (from Status::Pending)
            r#"(?:status|state)\s*=\s*\w+::(\w+)"#,
            // Match: .transition_to(OrderStatus::Shipped)
            r#"transition_to\s*\(\s*\w+::(\w+)\s*\)"#,
            // Match: set_status(Status.COMPLETED)
            r#"set_status\s*\(\s*\w+\.(\w+)\s*\)"#,
        ];

        // Also try to infer logical transitions based on state names
        let state_names_vec: Vec<&str> = states.iter().map(|s| s.name.as_str()).collect();
        
        // Common workflow patterns
        let common_flows = vec![
            vec!["Pending", "Processing", "Completed"],
            vec!["Draft", "Published", "Archived"],
            vec!["Created", "Active", "Completed", "Cancelled"],
            vec!["New", "InProgress", "Done"],
            vec!["Submitted", "Approved", "Rejected"],
            vec!["Pending", "Paid", "Fulfilled", "Refunded"],
        ];

        for flow in common_flows {
            let matching: Vec<&str> = flow.iter()
                .filter(|s| state_names_vec.iter().any(|n| n.to_lowercase().contains(&s.to_lowercase())))
                .copied()
                .collect();
            
            if matching.len() >= 2 {
                for window in matching.windows(2) {
                    transitions.push(StateTransition {
                        from_state: window[0].to_string(),
                        to_state: window[1].to_string(),
                        trigger: None,
                        condition: None,
                        actions: Vec::new(),
                    });
                }
            }
        }

        // Detect explicit transitions in code
        for pattern in transition_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                let matches: Vec<&str> = regex.captures_iter(content)
                    .filter_map(|c| c.get(1).map(|m| m.as_str()))
                    .filter(|s| state_names.contains(s))
                    .collect();
                
                // Create transitions from consecutive matches
                for window in matches.windows(2) {
                    if !transitions.iter().any(|t| t.from_state == window[0] && t.to_state == window[1]) {
                        transitions.push(StateTransition {
                            from_state: window[0].to_string(),
                            to_state: window[1].to_string(),
                            trigger: None,
                            condition: None,
                            actions: Vec::new(),
                        });
                    }
                }
            }
        }

        transitions
    }

    fn detect_fsm_patterns(&self, content: &str, file: &str) -> Vec<Workflow> {
        let mut workflows = Vec::new();

        // Look for explicit FSM/StateMachine implementations
        let fsm_patterns = vec![
            r#"(?s)impl\s+(?:State|FSM|StateMachine)\s+for\s+(\w+)"#,
            r#"(?s)struct\s+(\w+StateMachine)\s*\{"#,
            r#"(?s)class\s+(\w+)\s*\(.*StateMachine.*\)"#,
        ];

        for pattern in fsm_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                for cap in regex.captures_iter(content) {
                    let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
                    let line = content[..cap.get(0).unwrap().start()]
                        .lines().count() + 1;

                    workflows.push(Workflow {
                        name,
                        entity: None,
                        states: Vec::new(), // Would need more parsing
                        transitions: Vec::new(),
                        initial_state: None,
                        terminal_states: Vec::new(),
                        file: file.to_string(),
                        line,
                    });
                }
            }
        }

        workflows
    }

    fn detect_transitions(&self, content: &str, file: &str) -> Vec<Workflow> {
        let mut workflows = Vec::new();

        // Look for transition definitions
        let transition_regex = Regex::new(
            r#"(?s)fn\s+(\w*transition\w*|can_\w+|on_\w+)\s*\([^)]*\)"#
        ).ok();

        if let Some(regex) = transition_regex {
            let transition_functions: Vec<&str> = regex.captures_iter(content)
                .filter_map(|c| c.get(1).map(|m| m.as_str()))
                .collect();
            
            if transition_functions.len() >= 2 {
                // Group transitions into a workflow
                let workflow = Workflow {
                    name: "StateTransitions".to_string(),
                    entity: None,
                    states: Vec::new(),
                    transitions: transition_functions.iter().map(|f| {
                        StateTransition {
                            from_state: "any".to_string(),
                            to_state: f.to_string(),
                            trigger: Some(f.to_string()),
                            condition: None,
                            actions: Vec::new(),
                        }
                    }).collect(),
                    initial_state: None,
                    terminal_states: Vec::new(),
                    file: file.to_string(),
                    line: 1,
                };
                workflows.push(workflow);
            }
        }

        workflows
    }

    fn detect_initial_state(&self, states: &[WorkflowState], _content: &str) -> Option<String> {
        states.iter()
            .find(|s| s.is_initial)
            .or_else(|| states.first())
            .map(|s| s.name.clone())
    }

    fn detect_terminal_states(&self, states: &[WorkflowState]) -> Vec<String> {
        states.iter()
            .filter(|s| s.is_terminal)
            .map(|s| s.name.clone())
            .collect()
    }

    fn is_initial_state_name(name: &str) -> bool {
        let lower = name.to_lowercase();
        lower.contains("initial") || lower.contains("pending") || 
        lower.contains("draft") || lower.contains("new") ||
        lower.contains("created") || lower.contains("start")
    }

    fn is_terminal_state_name(name: &str) -> bool {
        let lower = name.to_lowercase();
        lower.contains("completed") || lower.contains("finished") ||
        lower.contains("cancelled") || lower.contains("failed") ||
        lower.contains("done") || lower.contains("closed") ||
        lower.contains("archived") || lower.contains("deleted")
    }

    fn is_source_file(path: &Path) -> bool {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        matches!(ext, "rs" | "py" | "js" | "ts" | "go" | "java")
    }

    fn should_skip_dir(path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        path_str.contains("node_modules") ||
        path_str.contains("target") ||
        path_str.contains(".git") ||
        path_str.contains("vendor")
    }
}

impl Default for WorkflowAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_enum_states() {
        let body = r#"
            Pending,
            Processing,
            Completed,
            Cancelled,
        "#;
        
        let analyzer = WorkflowAnalyzer::new();
        let states = analyzer.parse_enum_states(body);
        
        assert_eq!(states.len(), 4);
        assert!(states[0].is_initial); // Pending
        assert!(states[2].is_terminal); // Completed
        assert!(states[3].is_terminal); // Cancelled
    }

    #[test]
    fn test_infer_transitions() {
        let states = vec![
            WorkflowState {
                name: "Pending".to_string(),
                description: None,
                is_initial: true,
                is_terminal: false,
                entry_actions: Vec::new(),
                exit_actions: Vec::new(),
            },
            WorkflowState {
                name: "Processing".to_string(),
                description: None,
                is_initial: false,
                is_terminal: false,
                entry_actions: Vec::new(),
                exit_actions: Vec::new(),
            },
            WorkflowState {
                name: "Completed".to_string(),
                description: None,
                is_initial: false,
                is_terminal: true,
                entry_actions: Vec::new(),
                exit_actions: Vec::new(),
            },
        ];
        
        let analyzer = WorkflowAnalyzer::new();
        let transitions = analyzer.infer_transitions(&states, "");
        
        assert!(!transitions.is_empty());
    }
}
