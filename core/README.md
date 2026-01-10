# Riwaq Core

> Rust-based codebase analysis engine for intelligent documentation generation.

## Features

- **Multi-language Parsing**: Supports Rust, Python, JavaScript, TypeScript, and Go via tree-sitter
- **Dependency Graph**: Builds module dependency graphs with circular dependency detection
- **Git Analysis**: Analyzes commit history, file churn, and co-change patterns
- **Service Detection**: Automatically detects HTTP, gRPC, GraphQL, and CLI entry points
- **LLM Integration**: Ready for GLM-4.7 (200K context) for documentation generation

## Installation

```bash
# Build from source
cargo build --release

# The binary will be at target/release/riwaq
```

## Usage

### Analyze a Codebase

```bash
# Analyze current directory
riwaq analyze --path .

# Output to JSON file
riwaq analyze --path ./my-project --output snapshot.json

# Limit git history
riwaq analyze --path . --max-commits 100
```

### Generate Documentation

```bash
# Generate docs with LLM
riwaq docgen --path . --output ./generated-docs

# Skip LLM calls (structure only)
riwaq docgen --path . --skip-llm
```

### Ask Questions

```bash
# Ask about the codebase
riwaq ask --path . --question "What does the UserService do?"

# With streaming response
riwaq ask --path . --question "Explain the authentication flow" --stream
```

### Start Server (for VS Code Extension)

```bash
# Start HTTP server
riwaq serve --port 9527
```

## Configuration

Create a `.riwaq.toml` file in your project root. See [.riwaq.example.toml](./.riwaq.example.toml) for all options.

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RIWAQ_LLM_ENDPOINT` | LLM API endpoint | - |
| `RIWAQ_LLM_API_KEY` | API key for LLM | - |
| `RIWAQ_LOG_LEVEL` | Log level (debug, info, warn, error) | info |

## Development

```bash
# Run tests
cargo test

# Run with verbose logging
RUST_LOG=debug cargo run -- analyze --path .

# Check formatting
cargo fmt --check

# Run lints
cargo clippy
```

## Architecture

```
core/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library exports
│   ├── errors.rs        # Error types
│   ├── logging.rs       # Logging utilities
│   ├── analysis/        # Analysis modules
│   │   ├── analyzer.rs  # Main orchestrator
│   │   ├── fs_scanner.rs
│   │   ├── parser.rs    # Tree-sitter parsing
│   │   └── git.rs       # Git history analysis
│   ├── models/          # Data models
│   │   ├── file.rs
│   │   ├── function.rs
│   │   ├── module.rs
│   │   ├── git.rs
│   │   └── snapshot.rs
│   ├── llm/             # LLM integration (Phase 2)
│   └── docs/            # Doc generation (Phase 3)
```

## License

MIT
