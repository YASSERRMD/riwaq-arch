# Riwaq Arch - VS Code Extension

<div align="center">

<img src="images/icon.png" alt="Riwaq Arch" width="128" height="128">

**AI-powered codebase documentation and architecture analysis** for Visual Studio Code.

</div>

## Features

### Codebase Analysis
- Multi-language code parsing (Rust, Python, JavaScript, TypeScript, Go)
- Module detection and dependency graph construction
- Service/entry point identification
- Git history insights

### Documentation Generation
- Automatic architecture overview generation
- Per-module documentation
- Dependency analysis reports
- Architecture Decision Records (ADRs)

### AI-Powered Q&A
- Ask natural language questions about your codebase
- Get answers with file references
- Powered by GLM-4 or any OpenAI-compatible LLM

### Visual Architecture
- Mermaid diagram generation
- Interactive architecture exploration
- Module health metrics

## Installation

### From VS Code Marketplace
1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "Riwaq Arch"
4. Click Install

### From Source
```bash
cd vscode-extension
npm install
npm run compile
```

Then press F5 to launch the Extension Development Host.

## Usage

### Analyze Your Workspace
1. Open a project in VS Code
2. Open the Command Palette (Ctrl+Shift+P)
3. Run "Riwaq: Analyze Workspace"

### Generate Documentation
1. Run "Riwaq: Generate Documentation"
2. Choose output directory
3. Documentation files are generated as Markdown

### Ask Questions
1. Run "Riwaq: Ask About Codebase"
2. Type your question in natural language
3. View the AI-generated answer

### View Architecture
1. Run "Riwaq: Show Architecture Diagram"
2. Interactive diagram opens in a panel

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `riwaq.serverUrl` | `http://127.0.0.1:9527` | Backend server URL |
| `riwaq.autoAnalyze` | `true` | Auto-analyze on workspace open |
| `riwaq.llmApiKey` | - | API key for LLM provider |
| `riwaq.llmModel` | `glm-4` | LLM model to use |
| `riwaq.excludedDirs` | `[node_modules, target, ...]` | Directories to exclude |
| `riwaq.maxCommits` | `500` | Max git commits to analyze |

## Requirements

- VS Code 1.85.0 or higher
- Riwaq CLI installed (`cargo install --path ./core`)
- (Optional) LLM API key for AI features

## Backend Server

The extension communicates with the Riwaq backend. Start the server with:

```bash
riwaq serve --port 9527
```

Or run analysis directly from CLI:

```bash
riwaq analyze --path ./my-project
riwaq docgen --path ./my-project --output ./docs
```

## Development

```bash
# Install dependencies
npm install

# Compile TypeScript
npm run compile

# Watch mode
npm run watch

# Lint
npm run lint

# Package extension
npm run package
```

## License

MIT License - see [LICENSE](../LICENSE) for details.

## Contributing

Contributions are welcome! Please read the [contributing guidelines](../CONTRIBUTING.md) first.
