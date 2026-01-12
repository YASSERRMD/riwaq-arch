//! GraphQL Schema Analyzer
//!
//! Parses GraphQL schema files to extract:
//! - Type definitions
//! - Queries, Mutations, Subscriptions
//! - Custom scalars and directives

use super::*;
use regex::Regex;
use std::path::Path;
use tokio::fs;
use walkdir::WalkDir;

pub struct GraphQLAnalyzer;

impl GraphQLAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub async fn analyze(&self, root_path: &Path) -> anyhow::Result<Option<GraphQLSummary>> {
        let mut schema_files = Vec::new();
        let mut all_content = String::new();

        // Find all GraphQL schema files
        for entry in WalkDir::new(root_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            
            if matches!(ext, "graphql" | "gql") {
                let relative_path = path.strip_prefix(root_path)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .to_string();
                
                schema_files.push(relative_path);

                if let Ok(content) = fs::read_to_string(path).await {
                    all_content.push_str(&content);
                    all_content.push('\n');
                }
            }
        }

        if schema_files.is_empty() {
            return Ok(None);
        }

        let types = Self::parse_types(&all_content);
        let queries = Self::parse_operations(&all_content, "Query");
        let mutations = Self::parse_operations(&all_content, "Mutation");
        let subscriptions = Self::parse_operations(&all_content, "Subscription");
        let scalars = Self::parse_scalars(&all_content);
        let directives = Self::parse_directives(&all_content);

        Ok(Some(GraphQLSummary {
            schema_files,
            types,
            queries,
            mutations,
            subscriptions,
            scalars,
            directives,
        }))
    }

    fn parse_types(content: &str) -> Vec<GraphQLType> {
        let mut types = Vec::new();

        // Match type definitions
        let type_regex = Regex::new(
            r#"(?s)(type|input|interface|union|enum)\s+(\w+)(?:\s+implements\s+\w+)?\s*\{([^}]*)\}"#
        ).unwrap();

        for cap in type_regex.captures_iter(content) {
            let kind_str = cap.get(1).map(|m| m.as_str()).unwrap_or("type");
            let name = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
            let body = cap.get(3).map(|m| m.as_str()).unwrap_or("");

            // Skip Query, Mutation, Subscription root types
            if matches!(name.as_str(), "Query" | "Mutation" | "Subscription") {
                continue;
            }

            let kind = match kind_str {
                "type" => GraphQLTypeKind::Object,
                "input" => GraphQLTypeKind::Input,
                "interface" => GraphQLTypeKind::Interface,
                "union" => GraphQLTypeKind::Union,
                "enum" => GraphQLTypeKind::Enum,
                _ => GraphQLTypeKind::Object,
            };

            let fields = Self::parse_fields(body);
            let description = Self::extract_description(content, &name);

            types.push(GraphQLType {
                name,
                kind,
                fields,
                description,
            });
        }

        types
    }

    fn parse_fields(body: &str) -> Vec<GraphQLField> {
        let mut fields = Vec::new();

        // Match field definitions: name(args): Type
        let field_regex = Regex::new(
            r#"(\w+)(?:\(([^)]*)\))?\s*:\s*(\[?\w+!?\]?!?)"#
        ).unwrap();

        for cap in field_regex.captures_iter(body) {
            let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let args_str = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let field_type = cap.get(3).map(|m| m.as_str().to_string()).unwrap_or_default();

            let nullable = !field_type.ends_with('!');
            let args = Self::parse_arguments(args_str);

            fields.push(GraphQLField {
                name,
                field_type,
                nullable,
                args,
                description: None,
            });
        }

        fields
    }

    fn parse_arguments(args_str: &str) -> Vec<GraphQLArg> {
        let mut args = Vec::new();

        if args_str.is_empty() {
            return args;
        }

        // Match argument definitions: name: Type = default
        let arg_regex = Regex::new(
            r#"(\w+)\s*:\s*(\[?\w+!?\]?!?)(?:\s*=\s*([^\s,]+))?"#
        ).unwrap();

        for cap in arg_regex.captures_iter(args_str) {
            let name = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
            let arg_type = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
            let default_value = cap.get(3).map(|m| m.as_str().to_string());

            args.push(GraphQLArg {
                name,
                arg_type,
                default_value,
            });
        }

        args
    }

    fn parse_operations(content: &str, root_type: &str) -> Vec<GraphQLOperation> {
        let mut operations = Vec::new();

        // Find the root type definition
        let pattern = format!(r#"(?s)type\s+{}\s*\{{([^}}]*)\}}"#, root_type);
        let type_regex = Regex::new(&pattern).unwrap();

        if let Some(cap) = type_regex.captures(content) {
            let body = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            
            // Parse each field as an operation
            let field_regex = Regex::new(
                r#"(\w+)(?:\(([^)]*)\))?\s*:\s*(\[?\w+!?\]?!?)"#
            ).unwrap();

            for field_cap in field_regex.captures_iter(body) {
                let name = field_cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
                let args_str = field_cap.get(2).map(|m| m.as_str()).unwrap_or("");
                let return_type = field_cap.get(3).map(|m| m.as_str().to_string()).unwrap_or_default();

                let args = Self::parse_arguments(args_str);

                operations.push(GraphQLOperation {
                    name,
                    return_type,
                    args,
                    description: None,
                });
            }
        }

        operations
    }

    fn parse_scalars(content: &str) -> Vec<String> {
        let scalar_regex = Regex::new(r#"scalar\s+(\w+)"#).unwrap();
        
        scalar_regex.captures_iter(content)
            .filter_map(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }

    fn parse_directives(content: &str) -> Vec<String> {
        let directive_regex = Regex::new(r#"directive\s+@(\w+)"#).unwrap();
        
        directive_regex.captures_iter(content)
            .filter_map(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .collect()
    }

    fn extract_description(content: &str, type_name: &str) -> Option<String> {
        // Look for triple-quoted description before type definition
        let pattern = format!(r#"""""([^"]*)""""\s*(?:type|input|interface)\s+{}"#, type_name);
        let desc_regex = Regex::new(&pattern).ok()?;
        
        desc_regex.captures(content)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().trim().to_string())
    }
}

impl Default for GraphQLAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_types() {
        let schema = r#"
            type User {
                id: ID!
                name: String!
                email: String
                posts: [Post!]!
            }
            
            input CreateUserInput {
                name: String!
                email: String!
            }
        "#;
        
        let types = GraphQLAnalyzer::parse_types(schema);
        assert_eq!(types.len(), 2);
        assert_eq!(types[0].name, "User");
        assert_eq!(types[0].fields.len(), 4);
        assert_eq!(types[1].name, "CreateUserInput");
    }

    #[test]
    fn test_parse_queries() {
        let schema = r#"
            type Query {
                user(id: ID!): User
                users(limit: Int = 10): [User!]!
                posts: [Post!]!
            }
        "#;
        
        let queries = GraphQLAnalyzer::parse_operations(schema, "Query");
        assert_eq!(queries.len(), 3);
        assert_eq!(queries[0].name, "user");
        assert_eq!(queries[0].args.len(), 1);
        assert_eq!(queries[1].args[0].default_value, Some("10".to_string()));
    }

    #[test]
    fn test_parse_scalars() {
        let schema = r#"
            scalar DateTime
            scalar JSON
            scalar Upload
        "#;
        
        let scalars = GraphQLAnalyzer::parse_scalars(schema);
        assert_eq!(scalars.len(), 3);
        assert!(scalars.contains(&"DateTime".to_string()));
    }
}
