# Riwaq Core Library

The core Rust library powering Riwaq Arch's codebase analysis and documentation generation.

## Overview

`riwaq-core` provides:

- **Static Code Analysis**: Multi-language parsing using tree-sitter
- **Dependency Graph Construction**: Module and function-level dependency tracking
- **Git History Analysis**: Commit patterns, file churn, and contributor insights
- **LLM Integration**: OpenAI-compatible API client for documentation generation
- **Documentation Engine**: Markdown and Mermaid diagram generation
- **HTTP Server**: REST API for IDE integration

## Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
riwaq-core = { git = "https://github.com/YASSERRMD/riwaq-arch", branch = "main" }
```

### Basic Analysis

```rust
use riwaq_core::{CodebaseAnalyzer, CodebaseSnapshot};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create analyzer
    let analyzer = CodebaseAnalyzer::new(PathBuf::from("./my-project"))
        .with_max_commits(500)
        .with_include_tests(false);

    // Run analysis
    let snapshot: CodebaseSnapshot = analyzer.analyze().await?;

    // Access results
    println!("Project: {}", snapshot.metadata.project_name);
    println!("Files: {}", snapshot.statistics.total_files);
    println!("Modules: {}", snapshot.statistics.total_modules);
    println!("Functions: {}", snapshot.statistics.total_functions);
    
    // Check for issues
    if !snapshot.dependency_graph.circular_deps.is_empty() {
        println!("Warning: {} circular dependencies found", 
            snapshot.dependency_graph.circular_deps.len());
    }

    Ok(())
}
```

### Documentation Generation

```rust
use riwaq_core::{DocGenerator, DocGeneratorConfig, CodebaseSnapshot};
use std::path::PathBuf;

async fn generate_docs(snapshot: &CodebaseSnapshot) -> anyhow::Result<()> {
    let config = DocGeneratorConfig::new("./docs")
        .with_project_name("My Project")
        .with_llm(true);

    let generator = DocGenerator::new(config);
    let result = generator.generate(snapshot).await?;

    println!("Generated {} files in {}ms", 
        result.files.len(), 
        result.generation_time_ms);

    Ok(())
}
```

### LLM Integration

```rust
use riwaq_core::{HttpLlmClient, LLMClient, LLMConfig, CodebaseSnapshot};

async fn ask_about_code(snapshot: &CodebaseSnapshot) -> anyhow::Result<()> {
    let config = LLMConfig::default()
        .with_model("glm-4")
        .with_api_key("your-api-key");

    let client = HttpLlmClient::new(config)?;

    // Ask a question
    let response = client.answer_question(
        snapshot, 
        "How does the authentication module work?"
    ).await?;

    println!("Answer: {}", response.answer);
    println!("Confidence: {:.0}%", response.confidence * 100.0);

    // Generate architecture overview
    let overview = client.generate_architecture_overview(snapshot).await?;
    println!("{}", overview.content);

    Ok(())
}
```

### HTTP Server

```rust
use riwaq_core::{create_router, AppState, LLMConfig};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let state = AppState::new(LLMConfig::default());
    let app = create_router(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9527").await?;
    axum::serve(listener, app).await
}
```

## Module Reference

### `analysis`

Core code analysis functionality.

| Component | Description |
|-----------|-------------|
| `CodebaseAnalyzer` | Main orchestrator for codebase analysis |
| `FsScanner` | File system discovery with gitignore support |
| `CodeParser` | Multi-language parsing using tree-sitter |
| `GitAnalyzer` | Git history and contributor analysis |

### `docs`

Documentation generation.

| Component | Description |
|-----------|-------------|
| `DocGenerator` | Main documentation generator |
| `MarkdownBuilder` | Fluent API for Markdown construction |
| `mermaid` | Mermaid diagram generators |

### `llm`

LLM client and prompts.

| Component | Description |
|-----------|-------------|
| `LLMClient` | Trait for LLM provider abstraction |
| `HttpLlmClient` | OpenAI-compatible HTTP client |
| `LLMConfig` | Configuration with builder pattern |
| `prompts` | Prompt templates for various tasks |

### `models`

Data structures.

| Model | Description |
|-------|-------------|
| `CodebaseSnapshot` | Complete analysis result |
| `FileSummary` | Individual file analysis |
| `ModuleSummary` | Module with metrics and dependencies |
| `FunctionSummary` | Function metadata |
| `TypeSummary` | Type/class information |
| `GitInsights` | Repository history analysis |

### `server`

HTTP API server.

| Component | Description |
|-----------|-------------|
| `AppState` | Shared application state |
| `handlers` | HTTP request handlers |
| `routes` | Router configuration |

## Data Models

### CodebaseSnapshot

```rust
pub struct CodebaseSnapshot {
    pub metadata: SnapshotMetadata,
    pub files: Vec<FileSummary>,
    pub modules: Vec<ModuleSummary>,
    pub dependency_graph: DependencyGraph,
    pub git_insights: GitInsights,
    pub services: Vec<ServiceInfo>,
    pub statistics: CodebaseStatistics,
}
```

### ModuleSummary

```rust
pub struct ModuleSummary {
    pub name: String,
    pub path: String,
    pub files: Vec<PathBuf>,
    pub public_functions: Vec<String>,
    pub public_types: Vec<String>,
    pub dependencies: Vec<ModuleDependency>,
    pub metrics: ModuleMetrics,
}

pub struct ModuleMetrics {
    pub file_count: usize,
    pub lines_of_code: usize,
    pub function_count: usize,
    pub type_count: usize,
    pub coupling_score: f32,    // 0.0 - 1.0 (lower is better)
    pub cohesion_score: f32,    // 0.0 - 1.0 (higher is better)
}
```

## CLI Reference

```bash
# Analyze a codebase
riwaq analyze --path /path/to/project [--verbose]

# Generate documentation
riwaq docgen --path /path/to/project --output ./docs [--skip-llm]

# Ask a question
riwaq ask --path /path/to/project --question "..." [--stream]

# Start HTTP server
riwaq serve [--host 127.0.0.1] [--port 9527]
```

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- analyze --path .

# Check for issues
cargo clippy
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime |
| `tree-sitter` | Code parsing |
| `git2` | Git operations |
| `reqwest` | HTTP client |
| `axum` | HTTP server |
| `serde` | Serialization |
| `tracing` | Logging |

## License

MIT License - see [LICENSE](../LICENSE) for details.
