# Riwaq Arch v2.0 Implementation Plan

## Phase 1: Analysis Engine (Changes 1-3)

### Change 1: Intelligent API Detection Engine

#### 1.1 New Rust Modules
- `core/src/analysis/api/mod.rs` - API detection coordinator
- `core/src/analysis/api/rest.rs` - REST API analyzer
- `core/src/analysis/api/grpc.rs` - gRPC/protobuf parser
- `core/src/analysis/api/graphql.rs` - GraphQL schema parser
- `core/src/analysis/api/webhooks.rs` - Webhook detector

#### 1.2 New Data Structures
```rust
pub struct ApiInventory {
    pub rest_endpoints: Vec<RestEndpoint>,
    pub grpc_services: Vec<GrpcService>,
    pub graphql_schema: Option<GraphQLSchema>,
    pub webhooks: Vec<WebhookDefinition>,
}
```

### Change 2: Business Logic Extraction

#### 2.1 New Modules
- `core/src/analysis/business/mod.rs` - Business logic coordinator
- `core/src/analysis/business/entities.rs` - Entity/model detection
- `core/src/analysis/business/workflows.rs` - State machine detection
- `core/src/analysis/business/rules.rs` - Business rule extraction

### Change 3: SDK Documentation Enhancement

#### 3.1 New Modules
- `core/src/docs/sdk.rs` - SDK documentation generator
- Language-specific templates

---

## Implementation Status

- [ ] Phase 1: API Detection Engine
- [ ] Phase 2: Business Logic Extraction  
- [ ] Phase 3: SDK Documentation
- [ ] Phase 4: Enhanced Diagrams
- [ ] Phase 5: LLM Improvements
- [ ] Phase 6: Caching & Performance
- [ ] Phase 7: Error Handling
- [ ] Phase 8: Security
- [ ] Phase 9: Monitoring
