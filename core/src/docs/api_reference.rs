//! Enhanced API Reference Generator
//!
//! Generates comprehensive API documentation with:
//! - Multi-protocol support (REST, gRPC, GraphQL)
//! - Authentication details
//! - Rate limiting info
//! - Request/response examples

use crate::analysis::api::*;
use std::io::Write;

pub struct ApiReferenceGenerator;

impl ApiReferenceGenerator {
    pub fn generate(inventory: &ApiInventory) -> String {
        let mut output = String::new();

        output.push_str("# API Reference\n\n");
        output.push_str("*Auto-generated API documentation*\n\n");

        // Summary section
        output.push_str("## Overview\n\n");
        output.push_str(&Self::generate_summary(inventory));
        output.push('\n');

        // REST API section
        if !inventory.rest.endpoints.is_empty() {
            output.push_str(&Self::generate_rest_section(&inventory.rest));
        }

        // gRPC section
        if !inventory.grpc.services.is_empty() {
            output.push_str(&Self::generate_grpc_section(&inventory.grpc));
        }

        // GraphQL section
        if let Some(graphql) = &inventory.graphql {
            output.push_str(&Self::generate_graphql_section(graphql));
        }

        // Webhooks section
        if !inventory.webhooks.is_empty() {
            output.push_str(&Self::generate_webhooks_section(&inventory.webhooks));
        }

        output
    }

    fn generate_summary(inventory: &ApiInventory) -> String {
        let mut output = String::new();

        output.push_str("| Protocol | Count | Status |\n");
        output.push_str("|----------|-------|--------|\n");

        if inventory.statistics.total_rest_endpoints > 0 {
            output.push_str(&format!(
                "| REST | {} endpoints | ✅ Detected |\n",
                inventory.statistics.total_rest_endpoints
            ));
        }

        if inventory.statistics.total_grpc_services > 0 {
            output.push_str(&format!(
                "| gRPC | {} services, {} methods | ✅ Detected |\n",
                inventory.statistics.total_grpc_services,
                inventory.statistics.total_grpc_methods
            ));
        }

        if inventory.statistics.total_graphql_operations > 0 {
            output.push_str(&format!(
                "| GraphQL | {} operations | ✅ Detected |\n",
                inventory.statistics.total_graphql_operations
            ));
        }

        if inventory.statistics.total_webhooks > 0 {
            output.push_str(&format!(
                "| Webhooks | {} events | ✅ Detected |\n",
                inventory.statistics.total_webhooks
            ));
        }

        output.push('\n');

        // Authentication summary
        if !inventory.rest.auth_methods.is_empty() {
            output.push_str("### Authentication\n\n");
            for auth in &inventory.rest.auth_methods {
                let auth_str = match auth {
                    AuthMethod::Bearer => "Bearer Token (JWT)",
                    AuthMethod::ApiKey { header } => &format!("API Key ({})", header),
                    AuthMethod::OAuth2 { flows } => &format!("OAuth 2.0 ({})", flows.join(", ")),
                    AuthMethod::Basic => "Basic Authentication",
                    AuthMethod::Jwt => "JWT Authentication",
                    AuthMethod::Custom { name } => name,
                };
                output.push_str(&format!("- {}\n", auth_str));
            }
            output.push('\n');
        }

        // Rate limiting
        if let Some(rate_limit) = &inventory.rest.rate_limiting {
            output.push_str("### Rate Limiting\n\n");
            output.push_str(&format!(
                "- **Limit**: {} requests per {} seconds\n",
                rate_limit.requests_per_period,
                rate_limit.period_seconds
            ));
            output.push_str(&format!(
                "- **Headers**: {}\n\n",
                rate_limit.header_names.join(", ")
            ));
        }

        // Versions
        if !inventory.rest.versions.is_empty() {
            output.push_str("### API Versions\n\n");
            for version in &inventory.rest.versions {
                output.push_str(&format!("- `{}`\n", version));
            }
            output.push('\n');
        }

        output
    }

    fn generate_rest_section(rest: &RestApiSummary) -> String {
        let mut output = String::new();

        output.push_str("## REST API\n\n");

        // Group endpoints by base path
        let mut grouped: std::collections::HashMap<String, Vec<&RestEndpoint>> = std::collections::HashMap::new();
        
        for endpoint in &rest.endpoints {
            let base = endpoint.path.split('/').nth(1)
                .map(|s| format!("/{}", s))
                .unwrap_or_else(|| "/".to_string());
            grouped.entry(base).or_default().push(endpoint);
        }

        for (base_path, endpoints) in grouped.iter() {
            output.push_str(&format!("### {}\n\n", base_path));
            output.push_str("| Method | Path | Handler | Auth |\n");
            output.push_str("|--------|------|---------|------|\n");

            for endpoint in endpoints {
                let auth_badge = if endpoint.auth_required { "🔒" } else { "🔓" };
                let deprecated = if endpoint.deprecated { " ⚠️" } else { "" };
                
                output.push_str(&format!(
                    "| `{}` | `{}` | `{}`{} | {} |\n",
                    endpoint.method,
                    endpoint.path,
                    endpoint.handler,
                    deprecated,
                    auth_badge
                ));
            }
            output.push('\n');
        }

        // Detailed endpoint documentation
        output.push_str("### Endpoint Details\n\n");

        for endpoint in &rest.endpoints {
            output.push_str(&format!(
                "#### {} `{}`\n\n",
                endpoint.method,
                endpoint.path
            ));

            if let Some(desc) = &endpoint.description {
                output.push_str(&format!("{}\n\n", desc));
            }

            output.push_str(&format!(
                "- **Handler**: `{}`\n",
                endpoint.handler
            ));
            output.push_str(&format!(
                "- **File**: `{}`:{}\n",
                endpoint.file,
                endpoint.line
            ));
            
            if endpoint.auth_required {
                output.push_str("- **Authentication**: Required\n");
            }

            if endpoint.deprecated {
                output.push_str("- **Status**: ⚠️ Deprecated\n");
            }

            output.push('\n');
        }

        output
    }

    fn generate_grpc_section(grpc: &GrpcSummary) -> String {
        let mut output = String::new();

        output.push_str("## gRPC API\n\n");

        output.push_str("### Proto Files\n\n");
        for proto in &grpc.proto_files {
            output.push_str(&format!("- `{}`\n", proto));
        }
        output.push('\n');

        for service in &grpc.services {
            output.push_str(&format!("### Service: `{}`\n\n", service.name));
            
            if !service.package.is_empty() {
                output.push_str(&format!("**Package**: `{}`\n\n", service.package));
            }

            output.push_str("| Method | Type | Request | Response |\n");
            output.push_str("|--------|------|---------|----------|\n");

            for method in &service.methods {
                let stream_type = match (method.client_streaming, method.server_streaming) {
                    (false, false) => "Unary",
                    (true, false) => "Client Streaming",
                    (false, true) => "Server Streaming",
                    (true, true) => "Bidirectional",
                };

                output.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` |\n",
                    method.name,
                    stream_type,
                    method.input_type,
                    method.output_type
                ));
            }
            output.push('\n');
        }

        // Message types
        if !grpc.messages.is_empty() {
            output.push_str("### Message Types\n\n");
            
            for message in &grpc.messages {
                output.push_str(&format!("#### `{}`\n\n", message.name));
                output.push_str("```protobuf\n");
                output.push_str(&format!("message {} {{\n", message.name));
                
                for field in &message.fields {
                    let modifier = if field.repeated { "repeated " } else { "" };
                    output.push_str(&format!(
                        "    {}{} {} = {};\n",
                        modifier,
                        field.field_type,
                        field.name,
                        field.number
                    ));
                }
                
                output.push_str("}\n```\n\n");
            }
        }

        output
    }

    fn generate_graphql_section(graphql: &GraphQLSummary) -> String {
        let mut output = String::new();

        output.push_str("## GraphQL API\n\n");

        output.push_str("### Schema Files\n\n");
        for file in &graphql.schema_files {
            output.push_str(&format!("- `{}`\n", file));
        }
        output.push('\n');

        // Queries
        if !graphql.queries.is_empty() {
            output.push_str("### Queries\n\n");
            output.push_str("```graphql\n");
            output.push_str("type Query {\n");
            for query in &graphql.queries {
                let args = if query.args.is_empty() {
                    String::new()
                } else {
                    let args_str: Vec<String> = query.args.iter()
                        .map(|a| format!("{}: {}", a.name, a.arg_type))
                        .collect();
                    format!("({})", args_str.join(", "))
                };
                output.push_str(&format!("    {}{}: {}\n", query.name, args, query.return_type));
            }
            output.push_str("}\n```\n\n");
        }

        // Mutations
        if !graphql.mutations.is_empty() {
            output.push_str("### Mutations\n\n");
            output.push_str("```graphql\n");
            output.push_str("type Mutation {\n");
            for mutation in &graphql.mutations {
                let args = if mutation.args.is_empty() {
                    String::new()
                } else {
                    let args_str: Vec<String> = mutation.args.iter()
                        .map(|a| format!("{}: {}", a.name, a.arg_type))
                        .collect();
                    format!("({})", args_str.join(", "))
                };
                output.push_str(&format!("    {}{}: {}\n", mutation.name, args, mutation.return_type));
            }
            output.push_str("}\n```\n\n");
        }

        // Subscriptions
        if !graphql.subscriptions.is_empty() {
            output.push_str("### Subscriptions\n\n");
            for sub in &graphql.subscriptions {
                output.push_str(&format!("- `{}`: `{}`\n", sub.name, sub.return_type));
            }
            output.push('\n');
        }

        // Types
        if !graphql.types.is_empty() {
            output.push_str("### Types\n\n");
            for gql_type in &graphql.types {
                output.push_str(&format!("#### `{}`\n\n", gql_type.name));
                output.push_str("```graphql\n");
                output.push_str(&format!("type {} {{\n", gql_type.name));
                for field in &gql_type.fields {
                    output.push_str(&format!("    {}: {}\n", field.name, field.field_type));
                }
                output.push_str("}\n```\n\n");
            }
        }

        // Custom scalars
        if !graphql.scalars.is_empty() {
            output.push_str("### Custom Scalars\n\n");
            for scalar in &graphql.scalars {
                output.push_str(&format!("- `{}`\n", scalar));
            }
            output.push('\n');
        }

        output
    }

    fn generate_webhooks_section(webhooks: &[WebhookDefinition]) -> String {
        let mut output = String::new();

        output.push_str("## Webhooks\n\n");

        output.push_str("| Event Type | Endpoint | File |\n");
        output.push_str("|------------|----------|------|\n");

        for webhook in webhooks {
            let endpoint = webhook.endpoint.as_deref().unwrap_or("-");
            output.push_str(&format!(
                "| `{}` | `{}` | `{}`:{} |\n",
                webhook.event_type,
                endpoint,
                webhook.file,
                webhook.line
            ));
        }
        output.push('\n');

        // Retry policies
        let with_retry: Vec<_> = webhooks.iter()
            .filter(|w| w.retry_policy.is_some())
            .collect();
        
        if !with_retry.is_empty() {
            output.push_str("### Retry Policies\n\n");
            for webhook in with_retry {
                if let Some(policy) = &webhook.retry_policy {
                    output.push_str(&format!(
                        "- `{}`: {} retries, {}ms backoff\n",
                        webhook.event_type,
                        policy.max_retries,
                        policy.backoff_ms
                    ));
                }
            }
            output.push('\n');
        }

        output
    }
}
