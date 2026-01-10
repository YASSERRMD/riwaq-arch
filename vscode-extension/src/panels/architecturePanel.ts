// Architecture Panel - Webview for displaying architecture diagram
import * as vscode from 'vscode';
import { RiwaqClient, CodebaseSnapshot } from '../client';

export class ArchitecturePanel {
    public static currentPanel: ArchitecturePanel | undefined;
    private readonly _panel: vscode.WebviewPanel;
    private readonly _extensionUri: vscode.Uri;
    private _disposables: vscode.Disposable[] = [];
    private _client: RiwaqClient;

    public static createOrShow(extensionUri: vscode.Uri, client: RiwaqClient) {
        const column = vscode.ViewColumn.Beside;

        if (ArchitecturePanel.currentPanel) {
            ArchitecturePanel.currentPanel._panel.reveal(column);
            return;
        }

        const panel = vscode.window.createWebviewPanel(
            'riwaqArchitecture',
            'Riwaq: Architecture',
            column,
            {
                enableScripts: true,
                retainContextWhenHidden: true,
                localResourceRoots: [extensionUri]
            }
        );

        ArchitecturePanel.currentPanel = new ArchitecturePanel(panel, extensionUri, client);
    }

    private constructor(panel: vscode.WebviewPanel, extensionUri: vscode.Uri, client: RiwaqClient) {
        this._panel = panel;
        this._extensionUri = extensionUri;
        this._client = client;

        this._update();

        this._panel.onDidDispose(() => this.dispose(), null, this._disposables);
    }

    private _update() {
        const snapshot = this._client.getSnapshot();
        this._panel.webview.html = this._getHtmlForWebview(snapshot);
    }

    private _getHtmlForWebview(snapshot: CodebaseSnapshot | null): string {
        const mermaidDiagram = this._generateMermaidDiagram(snapshot);

        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Architecture Diagram</title>
    <script src="https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.min.js"></script>
    <style>
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }
        body {
            font-family: var(--vscode-font-family);
            background: var(--vscode-editor-background);
            color: var(--vscode-editor-foreground);
            padding: 20px;
            min-height: 100vh;
        }
        .header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 20px;
        }
        h1 {
            font-size: 1.5rem;
        }
        .controls {
            display: flex;
            gap: 10px;
        }
        .btn {
            padding: 8px 16px;
            background: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 13px;
        }
        .btn:hover {
            background: var(--vscode-button-hoverBackground);
        }
        .subtitle {
            color: var(--vscode-descriptionForeground);
            margin-bottom: 20px;
        }
        .diagram-container {
            background: #1e1e2e;
            border-radius: 12px;
            padding: 30px;
            overflow: auto;
            min-height: 500px;
            border: 1px solid var(--vscode-panel-border);
        }
        .mermaid {
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 400px;
        }
        .mermaid svg {
            max-width: 100%;
            height: auto;
        }
        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
            gap: 12px;
            margin-top: 20px;
        }
        .stat-card {
            background: var(--vscode-editor-inactiveSelectionBackground);
            padding: 16px;
            border-radius: 8px;
            text-align: center;
            border: 1px solid var(--vscode-panel-border);
        }
        .stat-value {
            font-size: 1.8rem;
            font-weight: bold;
            color: #7c3aed;
        }
        .stat-label {
            font-size: 0.75rem;
            color: var(--vscode-descriptionForeground);
            margin-top: 4px;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        .legend {
            margin-top: 20px;
            padding: 15px;
            background: var(--vscode-editor-inactiveSelectionBackground);
            border-radius: 8px;
            display: flex;
            flex-wrap: wrap;
            gap: 20px;
        }
        .legend-item {
            display: flex;
            align-items: center;
            gap: 8px;
            font-size: 12px;
        }
        .legend-color {
            width: 16px;
            height: 16px;
            border-radius: 4px;
        }
        .no-data {
            text-align: center;
            padding: 80px 20px;
            color: var(--vscode-descriptionForeground);
        }
        .no-data h2 {
            margin-bottom: 10px;
        }
    </style>
</head>
<body>
    <div class="header">
        <div>
            <h1>🏗️ Architecture Overview</h1>
            <p class="subtitle">Auto-generated from codebase analysis</p>
        </div>
        <div class="controls">
            <button class="btn" onclick="zoomIn()">🔍+</button>
            <button class="btn" onclick="zoomOut()">🔍-</button>
            <button class="btn" onclick="resetZoom()">Reset</button>
        </div>
    </div>

    ${snapshot ? this._getContentHtml(snapshot, mermaidDiagram) : this._getNoDataHtml()}

    <script>
        let currentZoom = 1;
        
        mermaid.initialize({ 
            startOnLoad: true,
            theme: 'dark',
            securityLevel: 'loose',
            flowchart: {
                curve: 'basis',
                padding: 30,
                nodeSpacing: 50,
                rankSpacing: 80,
                htmlLabels: true,
                useMaxWidth: false
            },
            themeVariables: {
                primaryColor: '#7c3aed',
                primaryTextColor: '#fff',
                primaryBorderColor: '#5b21b6',
                lineColor: '#6366f1',
                secondaryColor: '#4f46e5',
                tertiaryColor: '#1e1e2e',
                background: '#1e1e2e',
                mainBkg: '#2d2d3f',
                nodeBorder: '#5b21b6',
                clusterBkg: '#2d2d3f',
                clusterBorder: '#5b21b6',
                titleColor: '#e2e8f0',
                edgeLabelBackground: '#1e1e2e'
            }
        });

        function zoomIn() {
            currentZoom = Math.min(currentZoom + 0.2, 3);
            applyZoom();
        }
        
        function zoomOut() {
            currentZoom = Math.max(currentZoom - 0.2, 0.3);
            applyZoom();
        }
        
        function resetZoom() {
            currentZoom = 1;
            applyZoom();
        }
        
        function applyZoom() {
            const svg = document.querySelector('.mermaid svg');
            if (svg) {
                svg.style.transform = 'scale(' + currentZoom + ')';
                svg.style.transformOrigin = 'center top';
            }
        }
    </script>
</body>
</html>`;
    }

    private _getContentHtml(snapshot: CodebaseSnapshot, mermaidDiagram: string): string {
        const stats = snapshot.statistics || {};
        const modules = stats.totalModules || 0;
        const files = stats.totalFiles || 0;
        const functions = stats.totalFunctions || 0;
        const lines = stats.totalLines || 0;
        const services = snapshot.services?.length || 0;

        return `
            <div class="diagram-container">
                <div class="mermaid">
${mermaidDiagram}
                </div>
            </div>

            <div class="legend">
                <div class="legend-item">
                    <div class="legend-color" style="background: #7c3aed;"></div>
                    <span>Services / Entry Points</span>
                </div>
                <div class="legend-item">
                    <div class="legend-color" style="background: #4f46e5;"></div>
                    <span>Core Modules</span>
                </div>
                <div class="legend-item">
                    <div class="legend-color" style="background: #6366f1;"></div>
                    <span>Dependencies</span>
                </div>
            </div>

            <div class="stats">
                <div class="stat-card">
                    <div class="stat-value">${modules}</div>
                    <div class="stat-label">Modules</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${files}</div>
                    <div class="stat-label">Files</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${functions}</div>
                    <div class="stat-label">Functions</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${services}</div>
                    <div class="stat-label">Services</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${lines.toLocaleString()}</div>
                    <div class="stat-label">Lines</div>
                </div>
            </div>
        `;
    }

    private _getNoDataHtml(): string {
        return `
            <div class="no-data">
                <h2>📊 No Analysis Available</h2>
                <p>Run <strong>Riwaq: Analyze Workspace</strong> to generate the architecture diagram.</p>
                <p style="margin-top: 10px; font-size: 12px;">Tip: Click the refresh icon in the sidebar or use Command Palette</p>
            </div>
        `;
    }

    private _generateMermaidDiagram(snapshot: CodebaseSnapshot | null): string {
        if (!snapshot || !snapshot.modules || snapshot.modules.length === 0) {
            return `graph TD
    A[("🔍 Analyze your workspace first")]
    style A fill:#4f46e5,stroke:#7c3aed,color:#fff`;
        }

        let diagram = 'graph TB\n';

        // Style definitions
        diagram += '    classDef service fill:#7c3aed,stroke:#5b21b6,color:#fff,stroke-width:2px\n';
        diagram += '    classDef module fill:#4f46e5,stroke:#3730a3,color:#fff,stroke-width:1px\n';
        diagram += '    classDef util fill:#6366f1,stroke:#4f46e5,color:#fff,stroke-width:1px\n\n';

        // Add services subgraph
        if (snapshot.services && snapshot.services.length > 0) {
            diagram += '    subgraph SERVICES["🚀 Services & Entry Points"]\n';
            diagram += '        direction TB\n';
            for (const service of snapshot.services) {
                const icon = this._getServiceIcon(service.kind);
                const id = this._sanitizeId(service.name);
                const label = service.name.length > 20 ? service.name.substring(0, 18) + '...' : service.name;
                diagram += `        ${id}["${icon} ${label}"]\n`;
            }
            diagram += '    end\n\n';
        }

        // Group modules by type/prefix
        const coreModules = snapshot.modules.filter(m =>
            !m.name.includes('test') && !m.name.includes('util') && !m.name.includes('helper')
        ).slice(0, 12);

        const utilModules = snapshot.modules.filter(m =>
            m.name.includes('util') || m.name.includes('helper') || m.name.includes('common')
        ).slice(0, 6);

        // Core modules subgraph
        if (coreModules.length > 0) {
            diagram += '    subgraph CORE["📦 Core Modules"]\n';
            diagram += '        direction TB\n';
            for (const module of coreModules) {
                const id = this._sanitizeId(module.name);
                const label = module.name.length > 25 ? module.name.substring(0, 22) + '...' : module.name;
                const fileCount = module.files?.length || 0;
                diagram += `        ${id}["${label}<br/><small>${fileCount} files</small>"]\n`;
            }
            diagram += '    end\n\n';
        }

        // Utility modules subgraph
        if (utilModules.length > 0) {
            diagram += '    subgraph UTILS["🔧 Utilities"]\n';
            diagram += '        direction TB\n';
            for (const module of utilModules) {
                const id = this._sanitizeId(module.name);
                const label = module.name.length > 20 ? module.name.substring(0, 18) + '...' : module.name;
                diagram += `        ${id}["${label}"]\n`;
            }
            diagram += '    end\n\n';
        }

        // Add dependency links (limit to avoid clutter)
        let linkCount = 0;
        const maxLinks = 20;

        for (const module of coreModules) {
            if (linkCount >= maxLinks) break;
            const sourceId = this._sanitizeId(module.name);
            for (const dep of (module.dependencies || []).slice(0, 2)) {
                if (linkCount >= maxLinks) break;
                const targetModule = snapshot.modules.find(m =>
                    m.name === dep.target || m.path === dep.target
                );
                if (targetModule) {
                    const targetId = this._sanitizeId(targetModule.name);
                    if (sourceId !== targetId) {
                        diagram += `    ${sourceId} --> ${targetId}\n`;
                        linkCount++;
                    }
                }
            }
        }

        // Apply styles
        diagram += '\n';
        if (snapshot.services) {
            for (const service of snapshot.services) {
                diagram += `    class ${this._sanitizeId(service.name)} service\n`;
            }
        }
        for (const module of coreModules) {
            diagram += `    class ${this._sanitizeId(module.name)} module\n`;
        }
        for (const module of utilModules) {
            diagram += `    class ${this._sanitizeId(module.name)} util\n`;
        }

        return diagram;
    }

    private _sanitizeId(name: string): string {
        return name.replace(/[^a-zA-Z0-9]/g, '_').substring(0, 30);
    }

    private _getServiceIcon(kind: string): string {
        switch (kind?.toLowerCase()) {
            case 'httpserver': return '🌐';
            case 'grpcserver': return '⚡';
            case 'graphqlserver': return '📊';
            case 'websocket': return '🔌';
            case 'cli': return '💻';
            case 'worker': return '⚙️';
            case 'library': return '📚';
            default: return '📦';
        }
    }

    public dispose() {
        ArchitecturePanel.currentPanel = undefined;
        this._panel.dispose();
        while (this._disposables.length) {
            const x = this._disposables.pop();
            if (x) {
                x.dispose();
            }
        }
    }
}
