# Riwaq Arch - Visual Studio Extension

AI-powered codebase documentation and architecture analysis tool for Visual Studio 2022.

## Features

- **Tool Window**: Comprehensive 4-tab interface (Modules, Ask AI, Diagram, Insights)
- **Server Management**: Built-in server process control with auto-start capability
- **Interactive Chat UI**: AI-powered Q&A about your codebase
- **Architecture Diagrams**: Mermaid-based visualization with zoom controls
- **Code Health Metrics**: Real-time statistics on coupling, cohesion, and module quality
- **Integrated Commands**: Menu commands for server control and project analysis

## Installation

### From Source

**Prerequisites:**
- Visual Studio 2022 (17.0 or later)
- .NET Framework 4.7.2 or later
- Visual Studio SDK (included with VS 2022)

**Build Steps:**

1. Clone the repository
2. Build the Riwaq core binary:
   ```bash
   cd core
   cargo build --release
   # The binary will be placed at ../vscode-extension/bin/riwaq.exe
   ```
3. Open `visual-studio-extension\RiwaqArch.sln` in Visual Studio 2022
4. Build the solution (Ctrl+Shift+B)
5. The VSIX file will be in `bin\Debug\` or `bin\Release\`
6. Double-click the VSIX file to install

### Development

```bash
# Open in Visual Studio 2022
visual-studio-extension\RiwaqArch.sln

# Build (Ctrl+Shift+B)

# Run with debugging (F5)
# This will launch an Experimental Instance of Visual Studio
```

## Configuration

Configure the extension via **Tools → Options → Riwaq Arch**:

- **Server URL**: Where the Riwaq server is running (default: `http://127.0.0.1:9527`)
- **Auto-start Server**: Automatically start the bundled server when Visual Studio opens
- **LLM Configuration**: API key, endpoint, and model selection
- **Excluded Directories**: Directories to skip during analysis (e.g., `node_modules,dist,build,target`)

The extension includes a **bundled Riwaq server binary** that is automatically copied during the build process.

## Usage

### Quick Start

1. Open a solution in Visual Studio
2. Go to **Tools → Riwaq Arch → Start Server**
3. Go to **Tools → Riwaq Arch → Analyze Project**
4. Open the **Riwaq Architecture** tool window (View → Other Windows)
5. Explore your codebase in the Modules, Ask AI, Diagram, and Insights tabs

### Tool Window Tabs

- **Modules**: Browse your codebase modules with health indicators (🟢🟡🔴)
- **Ask AI**: Ask questions about your codebase and get AI-powered answers
- **Diagram**: View architecture diagrams (Mermaid format)
- **Insights**: See overall project statistics and metrics

### Menu Commands

Available from **Tools → Riwaq Arch**:

- **Analyze Project**: Analyze the current solution
- **Start Server**: Start the Riwaq analysis server
- **Stop Server**: Stop the running server

## Requirements

- Visual Studio 2022 (Community, Professional, or Enterprise)
- Windows 10 or later
- Riwaq server (bundled or user-provided)

## Architecture

```
visual-studio-extension/
├── Commands/                # Menu command handlers
│   ├── AnalyzeProjectCommand.cs
│   ├── StartServerCommand.cs
│   └── StopServerCommand.cs
├── ToolWindows/             # Tool window implementation
│   └── ArchitectureToolWindow.cs
├── Resources/               # Icons and images
│   ├── icon.png
│   └── preview.png
├── bin/                     # Bundled server binary (copied during build)
│   └── riwaq.exe
├── RiwaqArch.csproj         # Project file
├── RiwaqArchPackage.cs      # Package entry point
├── ServerManager.cs         # Server process management
├── SettingsOptions.cs       # Settings page
└── RiwaqClient.cs           # API client for server
```

## Troubleshooting

### Server Not Starting

1. Check if the bundled binary exists in the extension directory
2. Verify the server URL in Tools → Options → Riwaq Arch
3. Check Windows Firewall settings
4. Look for error messages in the Visual Studio output window

### Binary Not Found

If you see "Riwaq binary not found":
1. Build the core first: `cd core && cargo build --release`
2. Rebuild the extension
3. The binary should be automatically copied to the output directory

## Building for Release

1. Set the configuration to Release
2. Build the solution
3. Find the VSIX file in `bin\Release\`
4. The VSIX can be distributed to users for installation

## License

MIT License - see the [LICENSE](../../LICENSE) file for details.
