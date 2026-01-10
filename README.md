# Riwaq Arch

<div align="center">

<img src="images/icon.png" alt="Riwaq Arch" width="128" height="128">

**AI-Powered Codebase Documentation & Architecture Analysis**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![VS Code](https://img.shields.io/badge/VS%20Code-Extension-007ACC.svg)](vscode-extension/)

*Transform your codebase into living documentation*

</div>

---

## Overview

**Riwaq Arch** is an intelligent codebase analysis and documentation platform that automatically generates comprehensive architectural documentation for your projects. By combining static code analysis with Large Language Models (LLMs), Riwaq creates living documentation that stays in sync with your codebase.

### Key Capabilities

- **Multi-Language Support**: Parse and analyze Rust, Python, JavaScript, TypeScript, and Go codebases
- **Architecture Discovery**: Automatically detect modules, services, and dependency relationships
- **LLM-Enhanced Documentation**: Generate human-readable documentation powered by AI
- **Visual Diagrams**: Create Mermaid diagrams for architecture and dependencies
- **Git Intelligence**: Analyze commit history to identify code hotspots and change patterns
- **Interactive Q&A**: Ask natural language questions about your codebase

---

## Installation

### Prerequisites

- **Rust** 1.75 or later ([Install Rust](https://rustup.rs/))
- **Git** for repository analysis
- **API Key** for LLM features (OpenRouter, Z.ai, or OpenAI-compatible)

### From Source

```bash
# Clone the repository
git clone https://github.com/YASSERRMD/riwaq-arch.git
cd riwaq-arch

# Build the CLI
cd core
cargo build --release

# Install to your PATH (optional)
cargo install --path .
```

### Binary Installation

```bash
# macOS/Linux
curl -sSL https://raw.githubusercontent.com/YASSERRMD/riwaq-arch/main/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/YASSERRMD/riwaq-arch/main/install.ps1 | iex
```

---

## Quick Start

### 1. Analyze Your Codebase

```bash
riwaq analyze --path /path/to/your/project
```

This scans your project and produces a comprehensive analysis including:
- File and module structure
- Function and type inventories
- Dependency graph
- Git history insights

### 2. Generate Documentation

```bash
riwaq docgen --path /path/to/your/project --output ./docs
```

Generates a complete documentation suite:
- `README.md` - Project overview with statistics
- `ARCHITECTURE_OVERVIEW.md` - System architecture with diagrams
- `MODULES.md` - Detailed module documentation
- `DEPENDENCIES.md` - Dependency analysis and risk assessment
- `API_REFERENCE.md` - Service endpoint documentation
- `adrs/` - Architecture Decision Records

### 3. Ask Questions

```bash
riwaq ask --path /path/to/your/project --question "How does authentication work?"
```

Get AI-powered answers to questions about your codebase.

### 4. Start the Server (for VS Code)

```bash
riwaq serve --host 127.0.0.1 --port 9527
```

> [!WARNING]
> **Docker vs. VS Code Extension**
> 
> While you can run the server via Docker, we **recommend running it natively** (`riwaq serve`) when using the VS Code extension.
>
> **Why?** Docker containers have different file paths than your host machine (e.g., `/data/project` vs `/Users/you/project`). The VS Code extension sends your local host paths to the server, which the Docker container won't recognize unless custom volume mapping mirrors your exact host structure.

---

## Configuration

### 🤖 LLM Setup

To use AI features, you **must** configure an LLM provider. Riwaq supports OpenRouter, OpenAI, and compatible services.

👉 **[Read the Detailed LLM Setup Guide](docs/LLM_SETUP.md)**

#### Quick Setup (Environment Variables)

```bash
# Option 1: OpenRouter (Recommended)
export RIWAQ_LLM_API_KEY="sk-or-..."

# Option 2: OpenAI
export RIWAQ_LLM_API_KEY="sk-..."
export RIWAQ_LLM_ENDPOINT="https://api.openai.com/v1"
export RIWAQ_LLM_MODEL="gpt-4-turbo"
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RIWAQ_LLM_ENDPOINT` | LLM API endpoint | `https://openrouter.ai/api/v1` |
| `RIWAQ_LLM_MODEL` | LLM model to use | `glm-4` |
| `RIWAQ_LLM_API_KEY` | API key for LLM provider | - |

### Configuration File

Create `.riwaq.toml` in your project root:

```toml
# Project configuration
[project]
name = "my-project"

# Analysis settings
[analysis]
max_file_size = 10485760  # 10MB
max_commits = 500
include_tests = false

# Excluded directories
excluded_dirs = [
    "node_modules",
    "target",
    ".git",
    "dist",
    "build",
    "vendor"
]

# LLM settings
[llm]
endpoint = "https://open.routers.ai/api/v1"
model = "glm-4"
max_tokens = 4096
temperature = 0.3
timeout_secs = 120
```

---

## VS Code Extension

The Riwaq VS Code extension provides an integrated experience directly in your editor.

### Features

- **Architecture View**: Visual tree view of your project structure
- **Module Explorer**: Browse modules with health metrics
- **Quick Insights**: Code health, language breakdown, and statistics
- **Ask Panel**: Interactive Q&A about your codebase
- **Architecture Diagrams**: Mermaid diagram visualization

### Installation

1. Open VS Code
2. Go to Extensions (`Ctrl+Shift+X`)
3. Search for "Riwaq Arch"
4. Click Install

Or install from source:
```bash
cd vscode-extension
npm install
npm run compile
# Press F5 to launch Extension Development Host
```

---

## API Reference

When running the server (`riwaq serve`), the following REST API endpoints are available:

### Health Check
```http
GET /health
```

### Analyze Codebase
```http
POST /analyze
Content-Type: application/json

{
  "path": "/path/to/project",
  "max_commits": 500,
  "include_tests": false
}
```

### Generate Documentation
```http
POST /docgen
Content-Type: application/json

{
  "path": "/path/to/project",
  "output": "./generated-docs",
  "skip_llm": false
}
```

### Ask Question
```http
POST /ask
Content-Type: application/json

{
  "question": "How does the authentication module work?",
  "project_path": "/path/to/project"
}
```

### Get Architecture Diagram
```http
GET /architecture/diagram
```

---

## Supported Languages

| Language | File Extensions | Parser |
|----------|-----------------|--------|
| Rust | `.rs` | tree-sitter-rust |
| Python | `.py` | tree-sitter-python |
| JavaScript | `.js`, `.jsx` | tree-sitter-javascript |
| TypeScript | `.ts`, `.tsx` | tree-sitter-typescript |
| Go | `.go` | tree-sitter-go |

---

## Architecture

```
riwaq-arch/
├── core/                    # Rust core library and CLI
│   ├── src/
│   │   ├── analysis/        # Code parsing and analysis
│   │   ├── docs/            # Documentation generation
│   │   ├── llm/             # LLM client and prompts
│   │   ├── models/          # Data structures
│   │   ├── server/          # HTTP API server
│   │   └── main.rs          # CLI entry point
│   └── Cargo.toml
│
├── vscode-extension/        # VS Code extension
│   ├── src/
│   │   ├── extension.ts     # Extension entry point
│   │   ├── client.ts        # HTTP client
│   │   ├── views/           # Tree view providers
│   │   └── panels/          # Webview panels
│   └── package.json
│
└── README.md
```

---

## Performance

Riwaq is designed for efficiency:

- **Incremental Analysis**: Only re-analyzes changed files
- **Parallel Processing**: Multi-threaded file parsing
- **Lazy LLM Calls**: LLM operations are optional and cached
- **Memory Efficient**: Streams large files instead of loading entirely

### Benchmarks

| Project Size | Files | Analysis Time | Memory |
|--------------|-------|---------------|--------|
| Small (<1K files) | 500 | ~2s | ~50MB |
| Medium (1K-10K) | 5,000 | ~15s | ~200MB |
| Large (>10K) | 20,000 | ~45s | ~500MB |

---

## Troubleshooting

### Common Issues

**LLM API Key Not Working**
```bash
# Verify your API key is set
echo $RIWAQ_LLM_API_KEY

# Test with a simple request
riwaq ask --path . --question "What is this project?"
```

**Analysis Takes Too Long**
- Reduce `max_commits` in configuration
- Add large directories to `excluded_dirs`
- Enable `--skip-llm` for faster analysis

**VS Code Extension Not Connecting**
1. Ensure server is running: `riwaq serve`
2. Check server URL in VS Code settings
3. Verify firewall allows localhost connections

---

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone and build
git clone https://github.com/YASSERRMD/riwaq-arch.git
cd riwaq-arch/core
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- analyze --path .
```

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- [tree-sitter](https://tree-sitter.github.io/tree-sitter/) for robust code parsing
- [OpenRouter](https://openrouter.ai/) for LLM API access
- The Rust community for excellent tooling

---

<div align="center">

**Built by [YASSERRMD](https://github.com/YASSERRMD)**

[Documentation](https://github.com/YASSERRMD/riwaq-arch/wiki) | [Issues](https://github.com/YASSERRMD/riwaq-arch/issues) | [Discussions](https://github.com/YASSERRMD/riwaq-arch/discussions)

</div>
