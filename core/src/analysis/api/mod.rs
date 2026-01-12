//! API Detection Module - Multi-protocol API analyzer
//!
//! Supports:
//! - REST API detection with versioning, auth, rate limiting
//! - gRPC service detection from .proto files
//! - GraphQL schema parsing
//! - Webhook endpoint detection

pub mod rest;
pub mod grpc;
pub mod graphql;
pub mod webhooks;

use serde::{Deserialize, Serialize};
use std::path::Path;

pub use rest::RestAnalyzer;
pub use grpc::GrpcAnalyzer;
pub use graphql::GraphQLAnalyzer;
pub use webhooks::WebhookAnalyzer;

/// Complete API inventory for a codebase
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiInventory {
    /// REST API endpoints
    pub rest: RestApiSummary,
    /// gRPC services
    pub grpc: GrpcSummary,
    /// GraphQL schema
    pub graphql: Option<GraphQLSummary>,
    /// Webhook endpoints
    pub webhooks: Vec<WebhookDefinition>,
    /// Overall API statistics
    pub statistics: ApiStatistics,
}

/// REST API summary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestApiSummary {
    /// Detected endpoints
    pub endpoints: Vec<RestEndpoint>,
    /// Detected API versions
    pub versions: Vec<String>,
    /// Authentication methods detected
    pub auth_methods: Vec<AuthMethod>,
    /// Rate limiting detected
    pub rate_limiting: Option<RateLimitInfo>,
    /// Base paths detected
    pub base_paths: Vec<String>,
}

/// Individual REST endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestEndpoint {
    /// HTTP method
    pub method: String,
    /// Path pattern
    pub path: String,
    /// API version (if detected)
    pub version: Option<String>,
    /// Handler function name
    pub handler: String,
    /// Source file
    pub file: String,
    /// Line number
    pub line: usize,
    /// Description from doc comments
    pub description: Option<String>,
    /// Request parameters
    pub parameters: Vec<ApiParameter>,
    /// Request body schema
    pub request_body: Option<SchemaRef>,
    /// Response schemas
    pub responses: Vec<ApiResponse>,
    /// Authentication required
    pub auth_required: bool,
    /// Deprecated flag
    pub deprecated: bool,
    /// Tags/categories
    pub tags: Vec<String>,
}

/// API parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiParameter {
    pub name: String,
    pub location: ParameterLocation,
    pub param_type: String,
    pub required: bool,
    pub description: Option<String>,
    pub default_value: Option<String>,
}

/// Parameter location
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterLocation {
    Path,
    Query,
    Header,
    Cookie,
    Body,
}

/// API response definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub status_code: u16,
    pub description: Option<String>,
    pub content_type: String,
    pub schema: Option<SchemaRef>,
}

/// Schema reference
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaRef {
    pub name: String,
    pub schema_type: String,
    pub properties: Vec<SchemaProperty>,
}

/// Schema property
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaProperty {
    pub name: String,
    pub property_type: String,
    pub required: bool,
    pub description: Option<String>,
}

/// Authentication method
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AuthMethod {
    Bearer,
    ApiKey { header: String },
    OAuth2 { flows: Vec<String> },
    Basic,
    Jwt,
    Custom { name: String },
}

/// Rate limiting information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitInfo {
    pub requests_per_period: u32,
    pub period_seconds: u32,
    pub header_names: Vec<String>,
}

/// gRPC summary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrpcSummary {
    /// Proto files found
    pub proto_files: Vec<String>,
    /// Services defined
    pub services: Vec<GrpcService>,
    /// Messages defined
    pub messages: Vec<GrpcMessage>,
}

/// gRPC service definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrpcService {
    pub name: String,
    pub package: String,
    pub methods: Vec<GrpcMethod>,
    pub file: String,
}

/// gRPC method
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrpcMethod {
    pub name: String,
    pub input_type: String,
    pub output_type: String,
    pub client_streaming: bool,
    pub server_streaming: bool,
    pub description: Option<String>,
}

/// gRPC message type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrpcMessage {
    pub name: String,
    pub fields: Vec<GrpcField>,
}

/// gRPC field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrpcField {
    pub name: String,
    pub field_type: String,
    pub number: u32,
    pub repeated: bool,
    pub optional: bool,
}

/// GraphQL summary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLSummary {
    /// Schema files found
    pub schema_files: Vec<String>,
    /// Types defined
    pub types: Vec<GraphQLType>,
    /// Queries
    pub queries: Vec<GraphQLOperation>,
    /// Mutations
    pub mutations: Vec<GraphQLOperation>,
    /// Subscriptions
    pub subscriptions: Vec<GraphQLOperation>,
    /// Custom scalars
    pub scalars: Vec<String>,
    /// Directives
    pub directives: Vec<String>,
}

/// GraphQL type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLType {
    pub name: String,
    pub kind: GraphQLTypeKind,
    pub fields: Vec<GraphQLField>,
    pub description: Option<String>,
}

/// GraphQL type kind
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GraphQLTypeKind {
    Object,
    Input,
    Interface,
    Union,
    Enum,
    Scalar,
}

/// GraphQL field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLField {
    pub name: String,
    pub field_type: String,
    pub nullable: bool,
    pub args: Vec<GraphQLArg>,
    pub description: Option<String>,
}

/// GraphQL argument
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLArg {
    pub name: String,
    pub arg_type: String,
    pub default_value: Option<String>,
}

/// GraphQL operation (query/mutation/subscription)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLOperation {
    pub name: String,
    pub return_type: String,
    pub args: Vec<GraphQLArg>,
    pub description: Option<String>,
}

/// Webhook definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookDefinition {
    pub event_type: String,
    pub endpoint: Option<String>,
    pub payload_schema: Option<SchemaRef>,
    pub retry_policy: Option<RetryPolicy>,
    pub file: String,
    pub line: usize,
}

/// Retry policy for webhooks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub backoff_ms: u32,
}

/// API statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiStatistics {
    pub total_rest_endpoints: usize,
    pub total_grpc_services: usize,
    pub total_grpc_methods: usize,
    pub total_graphql_operations: usize,
    pub total_webhooks: usize,
    pub auth_methods_count: usize,
    pub has_versioning: bool,
    pub has_rate_limiting: bool,
}

/// Main API analyzer that coordinates all protocol analyzers
pub struct ApiAnalyzer {
    rest_analyzer: RestAnalyzer,
    grpc_analyzer: GrpcAnalyzer,
    graphql_analyzer: GraphQLAnalyzer,
    webhook_analyzer: WebhookAnalyzer,
}

impl ApiAnalyzer {
    pub fn new() -> Self {
        Self {
            rest_analyzer: RestAnalyzer::new(),
            grpc_analyzer: GrpcAnalyzer::new(),
            graphql_analyzer: GraphQLAnalyzer::new(),
            webhook_analyzer: WebhookAnalyzer::new(),
        }
    }

    /// Analyze all APIs in a codebase
    pub async fn analyze(&self, root_path: &Path) -> anyhow::Result<ApiInventory> {
        let rest = self.rest_analyzer.analyze(root_path).await?;
        let grpc = self.grpc_analyzer.analyze(root_path).await?;
        let graphql = self.graphql_analyzer.analyze(root_path).await?;
        let webhooks = self.webhook_analyzer.analyze(root_path).await?;

        let statistics = ApiStatistics {
            total_rest_endpoints: rest.endpoints.len(),
            total_grpc_services: grpc.services.len(),
            total_grpc_methods: grpc.services.iter().map(|s| s.methods.len()).sum(),
            total_graphql_operations: graphql.as_ref().map(|g| {
                g.queries.len() + g.mutations.len() + g.subscriptions.len()
            }).unwrap_or(0),
            total_webhooks: webhooks.len(),
            auth_methods_count: rest.auth_methods.len(),
            has_versioning: !rest.versions.is_empty(),
            has_rate_limiting: rest.rate_limiting.is_some(),
        };

        Ok(ApiInventory {
            rest,
            grpc,
            graphql,
            webhooks,
            statistics,
        })
    }
}

impl Default for ApiAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
