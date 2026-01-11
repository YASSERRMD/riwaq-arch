# Riwaq Arch - JetBrains Plugin

AI-powered codebase documentation and architecture analysis tool for JetBrains IDEs.

## Features

- **Automatic Architecture Diagrams**: Visualize your codebase structure with interactive diagrams
- **Interactive Documentation**: Get comprehensive documentation generated automatically
- **AI-Powered Q&A**: Ask questions about your codebase and get intelligent answers with file references
- **Code Health Insights**: Monitor metrics like coupling, cohesion, and module quality
- **Module Dependency Visualization**: See how modules connect and depend on each other
- **Real-time Analysis**: Get updates as your codebase changes

## Installation

### From Source

1. Clone this repository
2. Build the Riwaq core binary:
   ```bash
   cd core
   cargo build --release
   # The binary will be at ../vscode-extension/bin/riwaq (or riwaq.exe on Windows)
   ```
3. Build the plugin:
   ```bash
   cd jetbrains-plugin
   ./gradlew buildPlugin
   ```
4. The plugin will be built in `build/distributions/` with the bundled Riwaq binary
5. Install from: **IntelliJ IDEA → Settings → Plugins → Gear Icon → Install Plugin from Disk**

**Note**: The build process automatically bundles the Riwaq server binary from `../vscode-extension/bin/riwaq` into the plugin. Make sure to build the core first (step 2) before building the plugin.

### From Marketplace (Coming Soon)

Search for "Riwaq Arch" in the JetBrains Marketplace.

## Configuration

### Server Setup

The plugin includes a **bundled Riwaq server binary** and will automatically start it when needed. The server management options are available in:

**Settings → Tools → Riwaq Arch**

Options:
- **Server URL**: The URL where the Riwaq server is running (default: `http://127.0.0.1:9527`)
- **Server Binary Path**: Optional path to a custom `riwaq` binary. Leave empty to use the bundled binary.
- **Auto-start Server**: Automatically start the bundled server when the IDE opens
- **LLM Configuration**: Configure your LLM API key, endpoint, and model

The plugin searches for the server binary in this order:
1. Custom path (if specified in settings)
2. Bundled binary in plugin's `lib/bin/` directory
3. System PATH (as `riwaq` command)

### Project Settings

**Settings → Project → Riwaq Arch**

Options:
- **Auto-analyze**: Automatically analyze project on changes
- **Excluded Files**: Comma-separated list of file patterns to exclude
- **Custom Prompt**: Custom instructions for AI analysis

## Usage

### Quick Start

1. Open a project
2. The tool window will appear on the right side: "Riwaq Architecture"
3. Click "Analyze Project" from the Tools menu (Tools → Riwaq Actions → Analyze Project)
4. Explore your codebase in the Modules, Ask AI, Diagram, and Insights tabs

### Tabs

#### Modules
- Browse your codebase modules
- See health indicators (🟢🟡🔴) based on coupling and cohesion
- View metrics like lines of code, function count, and dependencies

#### Ask AI
- Ask questions about your codebase
- Get answers with file references
- Click example questions or type your own

#### Diagram
- View architecture diagram (Mermaid format)
- Zoom in/out controls
- Refresh to regenerate

#### Insights
- See overall project statistics
- Language distribution
- Average coupling/cohesion metrics

### Actions

Available from **Tools → Riwaq Actions**:

- **Start Riwaq Server**: Manually start the server
- **Stop Riwaq Server**: Manually stop the server
- **Restart Riwaq Server**: Restart the server
- **Analyze Project**: Analyze the current project
- **Ask About Codebase**: Open the AI Q&A panel
- **View Architecture Diagram**: Open the diagram panel

### Context Menu

Right-click on selected code in the editor to use "Ask Riwaq About Selection".

## Development

### Building

```bash
./gradlew buildPlugin
```

### Running in Development

```bash
./gradlew runIde
```

### Testing

```bash
./gradlew test
```

## Requirements

- IntelliJ IDEA 2023.2 or later (or compatible JetBrains IDE)
- Java 17 or later
- Riwaq server (can be bundled or user-provided)

## License

MIT License - see LICENSE file for details

## Support

For issues and feature requests, please visit the GitHub repository.
