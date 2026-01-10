# Riwaq Arch

> Intelligent codebase analysis and documentation generation powered by LLM.

**Riwaq Arch** is a VS Code extension with a Rust core that:
- 📊 Analyzes entire codebases (multi-language)
- 🧠 Uses GLM-4.7 (200K context) to understand structure & intent
- 📝 Generates complete, living documentation (architecture, APIs, ADRs, dependencies)
- 💬 Provides an interactive "Ask about this codebase" chat inside VS Code
- 🔄 Keeps docs in sync with code changes

## Project Structure

```
riwaq-arch/
├── core/           # Rust analysis engine & CLI
├── vscode/         # VS Code extension (TypeScript)
├── docs/           # Project documentation
├── scripts/        # Build and dev scripts
└── README.md       # This file
```

## Quick Start

### Prerequisites

- Rust 1.75+ (with cargo)
- Node.js 18+ (for VS Code extension)
- An LLM API key (OpenRouter, Z.ai, or compatible)

### Build the Core

```bash
cd core
cargo build --release

# Test the CLI
./target/release/riwaq analyze --path .
```

### Install the VS Code Extension

```bash
cd vscode
npm install
npm run compile

# Install in VS Code (development)
code --install-extension riwaq-arch-0.1.0.vsix
```

### Configure LLM

Set your LLM endpoint and API key:

```bash
export RIWAQ_LLM_ENDPOINT="https://api.openrouter.ai/api/v1"
export RIWAQ_LLM_API_KEY="your-api-key"
```

Or create a `.riwaq.toml` config file (see `core/.riwaq.example.toml`).

## Usage

### Generate Documentation

```bash
# From CLI
riwaq docgen --path ./my-project --output ./docs

# From VS Code
# Cmd/Ctrl + Shift + P → "Riwaq: Generate Documentation"
```

### Ask Questions

```bash
# From CLI
riwaq ask --path . --question "How does authentication work?"

# From VS Code
# Open the Riwaq sidebar → Q&A tab
```

## Generated Documentation

Riwaq generates the following in your `generated-docs/` folder:

| File | Description |
|------|-------------|
| `ARCHITECTURE_OVERVIEW.md` | High-level system architecture |
| `SERVICES_AND_MODULES.md` | Module descriptions and responsibilities |
| `API_REFERENCE.md` | HTTP/gRPC/GraphQL endpoints |
| `DEPENDENCIES_AND_RISKS.md` | Dependency analysis and risk assessment |
| `ADRS/ADR-*.md` | Architecture Decision Records |

## Development

### Phase 1: Core Analysis Engine (Rust) ✅
- [x] Multi-language parsing (tree-sitter)
- [x] Dependency graph construction
- [x] Git history analysis
- [x] Service detection

### Phase 2: LLM Integration 🚧
- [ ] LLM client trait
- [ ] GLM-4.7 HTTP client
- [ ] Prompt engineering
- [ ] Context management

### Phase 3: Documentation Generation 🔜
- [ ] Markdown generators
- [ ] Mermaid diagrams
- [ ] ADR generation

### Phase 4: VS Code Extension 🔜
- [ ] Extension scaffolding
- [ ] Webview UI
- [ ] Q&A chat

### Phase 5: Production Hardening 🔜
- [ ] Docker support
- [ ] Benchmarks
- [ ] Complete documentation

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see [LICENSE](./LICENSE) for details.

---

Built with ❤️ by [YASSERRMD](https://github.com/YASSERRMD)
