// Architecture Panel - Webview for displaying architecture diagram
import * as vscode from 'vscode';
import { RiwaqClient } from '../client';

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

    private _getHtmlForWebview(snapshot: any): string {
        const mermaidDiagram = this._generateMermaidDiagram(snapshot);

        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Architecture Diagram</title>
    <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
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
        h1 {
            font-size: 1.5rem;
            margin-bottom: 10px;
        }
        .subtitle {
            color: var(--vscode-descriptionForeground);
            margin-bottom: 20px;
        }
        .diagram-container {
            background: white;
            border-radius: 8px;
            padding: 20px;
            overflow: auto;
            min-height: 400px;
        }
        .mermaid {
            text-align: center;
        }
        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            gap: 15px;
            margin-top: 20px;
        }
        .stat-card {
            background: var(--vscode-editor-inactiveSelectionBackground);
            padding: 15px;
            border-radius: 8px;
            text-align: center;
        }
        .stat-value {
            font-size: 1.5rem;
            font-weight: bold;
            color: var(--vscode-textLink-foreground);
        }
        .stat-label {
            font-size: 0.8rem;
            color: var(--vscode-descriptionForeground);
            margin-top: 5px;
        }
        .no-data {
            text-align: center;
            padding: 50px;
            color: var(--vscode-descriptionForeground);
        }
    </style>
</head>
<body>
    <h1>🏗️ Architecture Overview</h1>
    <p class="subtitle">Auto-generated from codebase analysis</p>

    ${snapshot ? this._getContentHtml(snapshot, mermaidDiagram) : this._getNoDataHtml()}

    <script>
        mermaid.initialize({ 
            startOnLoad: true,
            theme: 'neutral',
            securityLevel: 'loose',
            flowchart: {
                curve: 'basis',
                padding: 20
            }
        });
    </script>
</body>
</html>`;
    }

    private _getContentHtml(snapshot: any, mermaidDiagram: string): string {
        return `
            <div class="diagram-container">
                <div class="mermaid">
${mermaidDiagram}
                </div>
            </div>

            <div class="stats">
                <div class="stat-card">
                    <div class="stat-value">${snapshot.statistics.totalModules}</div>
                    <div class="stat-label">Modules</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${snapshot.statistics.totalFiles}</div>
                    <div class="stat-label">Files</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${snapshot.statistics.totalFunctions}</div>
                    <div class="stat-label">Functions</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${snapshot.services.length}</div>
                    <div class="stat-label">Services</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${(snapshot.statistics.totalLines || 0).toLocaleString()}</div>
                    <div class="stat-label">Lines of Code</div>
                </div>
            </div>
        `;
    }

    private _getNoDataHtml(): string {
        return `
            <div class="no-data">
                <h2>No Analysis Available</h2>
                <p>Run "Riwaq: Analyze Workspace" to generate the architecture diagram.</p>
            </div>
        `;
    }

    private _generateMermaidDiagram(snapshot: any): string {
        if (!snapshot || !snapshot.modules) {
            return 'graph TD\n    A[No data available]';
        }

        let diagram = 'graph TD\n';

        // Add services
        if (snapshot.services && snapshot.services.length > 0) {
            diagram += '    subgraph Services\n';
            for (const service of snapshot.services) {
                const icon = this._getServiceIcon(service.kind);
                const id = this._sanitizeId(service.name);
                diagram += `        ${id}[${icon} ${service.name}]\n`;
            }
            diagram += '    end\n';
        }

        // Add modules
        if (snapshot.modules.length > 0) {
            diagram += '    subgraph Modules\n';
            for (const module of snapshot.modules.slice(0, 15)) {
                const id = this._sanitizeId(module.name);
                diagram += `        ${id}[${module.name}]\n`;
            }
            diagram += '    end\n';
        }

        // Add some dependency arrows
        for (const module of snapshot.modules.slice(0, 10)) {
            const sourceId = this._sanitizeId(module.name);
            for (const dep of (module.dependencies || []).slice(0, 3)) {
                const targetId = this._sanitizeId(dep.target);
                diagram += `    ${sourceId} --> ${targetId}\n`;
            }
        }

        return diagram;
    }

    private _sanitizeId(name: string): string {
        return name.replace(/[^a-zA-Z0-9]/g, '_');
    }

    private _getServiceIcon(kind: string): string {
        switch (kind.toLowerCase()) {
            case 'httpserver': return '🌐';
            case 'grpcserver': return '⚡';
            case 'cli': return '💻';
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
