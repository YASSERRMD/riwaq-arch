// Ask Panel - Webview for asking questions about the codebase
import * as vscode from 'vscode';
import { RiwaqClient } from '../client';

export class AskPanel {
    public static currentPanel: AskPanel | undefined;
    private readonly _panel: vscode.WebviewPanel;
    private readonly _extensionUri: vscode.Uri;
    private _disposables: vscode.Disposable[] = [];
    private _client: RiwaqClient;

    public static createOrShow(extensionUri: vscode.Uri, client: RiwaqClient) {
        const column = vscode.window.activeTextEditor
            ? vscode.window.activeTextEditor.viewColumn
            : undefined;

        if (AskPanel.currentPanel) {
            AskPanel.currentPanel._panel.reveal(column);
            return;
        }

        const panel = vscode.window.createWebviewPanel(
            'riwaqAsk',
            'Ask About Code',
            column || vscode.ViewColumn.One,
            {
                enableScripts: true,
                retainContextWhenHidden: true,
            }
        );

        AskPanel.currentPanel = new AskPanel(panel, extensionUri, client);
    }

    private constructor(panel: vscode.WebviewPanel, extensionUri: vscode.Uri, client: RiwaqClient) {
        this._panel = panel;
        this._extensionUri = extensionUri;
        this._client = client;

        this._update();

        this._panel.onDidDispose(() => this.dispose(), null, this._disposables);

        this._panel.webview.onDidReceiveMessage(
            async message => {
                switch (message.command) {
                    case 'ask':
                        await this._handleQuestion(message.question);
                        return;
                }
            },
            null,
            this._disposables
        );
    }

    private async _handleQuestion(question: string) {
        this._panel.webview.postMessage({ command: 'loading', loading: true });

        try {
            const result = await this._client.askQuestion(question);
            this._panel.webview.postMessage({
                command: 'answer',
                answer: result.answer,
                fileRefs: result.fileRefs,
                confidence: result.confidence,
                diagrams: result.diagrams || []
            });
        } catch (error: any) {
            this._panel.webview.postMessage({
                command: 'error',
                message: error.message
            });
        }
    }

    private _update() {
        this._panel.webview.html = this._getHtmlForWebview();
    }

    private _getHtmlForWebview(): string {
        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ask About Code</title>
    <script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.min.js"></script>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github-dark.min.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
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
            height: 100vh;
            display: flex;
            flex-direction: column;
        }
        h1 {
            font-size: 1.5rem;
            margin-bottom: 20px;
            color: var(--vscode-titleBar-activeForeground);
        }
        .input-container {
            display: flex;
            gap: 10px;
            margin-bottom: 20px;
        }
        input[type="text"] {
            flex: 1;
            padding: 12px 16px;
            border: 1px solid var(--vscode-input-border);
            background: var(--vscode-input-background);
            color: var(--vscode-input-foreground);
            border-radius: 6px;
            font-size: 14px;
        }
        input[type="text"]:focus {
            outline: none;
            border-color: var(--vscode-focusBorder);
        }
        button {
            padding: 12px 24px;
            background: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
            border: none;
            border-radius: 6px;
            cursor: pointer;
            font-weight: 600;
        }
        button:hover {
            background: var(--vscode-button-hoverBackground);
        }
        button:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }
        .answer-container {
            flex: 1;
            overflow-y: auto;
            padding: 15px;
            background: var(--vscode-editor-background);
            border: 1px solid var(--vscode-panel-border);
            border-radius: 8px;
        }
        .answer-content {
            line-height: 1.6;
        }
        .answer-content h1, .answer-content h2, .answer-content h3 {
            margin-top: 20px;
            margin-bottom: 10px;
        }
        .answer-content p {
            margin-bottom: 10px;
        }
        .answer-content pre {
            background: var(--vscode-textCodeBlock-background);
            padding: 15px;
            border-radius: 6px;
            overflow-x: auto;
            margin: 10px 0;
        }
        .answer-content code {
            font-family: var(--vscode-editor-font-family);
            font-size: 13px;
        }
        .answer-content ul, .answer-content ol {
            margin-left: 20px;
            margin-bottom: 10px;
        }
        
        /* Mermaid diagram styling - rendered as visual images */
        .mermaid {
            background: linear-gradient(135deg, #1e1e2e 0%, #2d2d44 100%);
            padding: 25px;
            border-radius: 12px;
            margin: 20px 0;
            text-align: center;
            border: 1px solid #3d3d5c;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
        }
        .mermaid svg {
            max-width: 100%;
            height: auto;
        }
        .diagram-container {
            background: linear-gradient(135deg, #1e1e2e 0%, #2d2d44 100%);
            padding: 25px;
            border-radius: 12px;
            margin: 20px 0;
            text-align: center;
            border: 1px solid #3d3d5c;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
        }
        .diagram-container svg {
            max-width: 100%;
            height: auto;
        }
        
        .loading {
            display: flex;
            align-items: center;
            gap: 10px;
            color: var(--vscode-descriptionForeground);
        }
        .spinner {
            width: 20px;
            height: 20px;
            border: 2px solid var(--vscode-progressBar-background);
            border-top-color: transparent;
            border-radius: 50%;
            animation: spin 1s linear infinite;
        }
        @keyframes spin {
            to { transform: rotate(360deg); }
        }
        .error {
            color: var(--vscode-errorForeground);
            padding: 15px;
            background: var(--vscode-inputValidation-errorBackground);
            border-radius: 6px;
            border-left: 3px solid var(--vscode-errorForeground);
        }
        .file-refs {
            margin-top: 20px;
            padding-top: 15px;
            border-top: 1px solid var(--vscode-panel-border);
        }
        .file-refs h3 {
            font-size: 0.9rem;
            margin-bottom: 10px;
            color: var(--vscode-descriptionForeground);
        }
        .file-ref {
            padding: 8px 12px;
            background: var(--vscode-editor-background);
            border-radius: 4px;
            margin-bottom: 5px;
            cursor: pointer;
            font-size: 13px;
            display: flex;
            justify-content: space-between;
        }
        .file-ref:hover {
            background: var(--vscode-list-hoverBackground);
        }
        .confidence {
            font-size: 0.8rem;
            color: var(--vscode-descriptionForeground);
            margin-top: 15px;
            text-align: right;
        }
    </style>
</head>
<body>
    <h1>🤔 Ask About Your Codebase</h1>
    
    <div class="input-container">
        <input type="text" id="question" placeholder="e.g., How does the authentication flow work?" />
        <button id="askBtn" onclick="askQuestion()">Ask</button>
    </div>

    <div class="answer-container" id="answerContainer">
        <p style="color: var(--vscode-descriptionForeground);">
            Ask any question about your codebase. Examples:
        </p>
        <ul style="margin-top: 10px; margin-left: 20px; color: var(--vscode-descriptionForeground);">
            <li>What is the main architecture pattern used?</li>
            <li>How does the authentication flow work?</li>
            <li>What are the key dependencies?</li>
            <li>Show me the data flow diagram</li>
        </ul>
    </div>

    <script>
        // Initialize Mermaid with dark theme for beautiful diagrams
        mermaid.initialize({
            startOnLoad: false,
            theme: 'dark',
            themeVariables: {
                primaryColor: '#7c3aed',
                primaryTextColor: '#fff',
                primaryBorderColor: '#5b21b6',
                lineColor: '#6366f1',
                secondaryColor: '#4f46e5',
                tertiaryColor: '#1e1e2e',
                background: '#1e1e2e',
                mainBkg: '#2d2d44',
                nodeBorder: '#5b21b6',
                clusterBkg: '#2d2d44',
                titleColor: '#fff',
                edgeLabelBackground: '#2d2d44'
            },
            flowchart: {
                htmlLabels: true,
                curve: 'basis',
                padding: 20
            },
            sequence: {
                actorMargin: 50,
                width: 150

            }
        });

        const vscode = acquireVsCodeApi();
        const questionInput = document.getElementById('question');
        const askBtn = document.getElementById('askBtn');
        const answerContainer = document.getElementById('answerContainer');

        questionInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                askQuestion();
            }
        });

        function askQuestion() {
            const question = questionInput.value.trim();
            if (!question) return;

            vscode.postMessage({
                command: 'ask',
                question: question
            });
        }

        // Custom marked renderer to handle mermaid code blocks
        const renderer = new marked.Renderer();
        const originalCodeRenderer = renderer.code.bind(renderer);
        
        renderer.code = function(code, language) {
            if (language === 'mermaid') {
                // Return a div that will be rendered by Mermaid as a visual diagram
                const id = 'mermaid-' + Math.random().toString(36).substr(2, 9);
                return '<div class="mermaid" id="' + id + '">' + code + '</div>';
            }
            return originalCodeRenderer(code, language);
        };

        marked.setOptions({ renderer: renderer });

        // Render all Mermaid diagrams as visual images
        async function renderMermaidDiagrams() {
            const mermaidDivs = document.querySelectorAll('.mermaid');
            for (const div of mermaidDivs) {
                try {
                    const id = div.id || 'mermaid-' + Math.random().toString(36).substr(2, 9);
                    const code = div.textContent.trim();
                    if (code) {
                        const { svg } = await mermaid.render(id + '-svg', code);
                        div.innerHTML = svg;
                    }
                } catch (error) {
                    console.error('Mermaid render error:', error);
                    div.innerHTML = '<p style="color: #f87171; padding: 10px;">⚠️ Failed to render diagram</p>';
                }
            }
        }

        window.addEventListener('message', async event => {
            const message = event.data;

            switch (message.command) {
                case 'loading':
                    askBtn.disabled = message.loading;
                    if (message.loading) {
                        answerContainer.innerHTML = '<div class="loading"><div class="spinner"></div>Thinking...</div>';
                    }
                    break;

                case 'answer':
                    // Convert Markdown to HTML - mermaid blocks become <div class="mermaid">
                    const rawHtml = marked.parse(message.answer);
                    
                    let html = '<div class="answer-content">' + rawHtml + '</div>';
                    
                    // Add pre-rendered SVG diagrams from backend
                    if (message.diagrams && message.diagrams.length > 0) {
                        for (const diagram of message.diagrams) {
                            html += '<div class="diagram-container">' + diagram.svg + '</div>';
                        }
                    }
                    
                    if (message.fileRefs && message.fileRefs.length > 0) {
                        html += '<div class="file-refs"><h3>📁 Referenced Files</h3>';
                        for (const ref of message.fileRefs) {
                            html += '<div class="file-ref"><span>' + escapeHtml(ref.path) + '</span><span style="opacity:0.7">' + escapeHtml(ref.relevance) + '</span></div>';
                        }
                        html += '</div>';
                    }

                    html += '<div class="confidence">Confidence: ' + Math.round(message.confidence * 100) + '%</div>';
                    
                    answerContainer.innerHTML = html;
                    
                    // Render Mermaid code blocks as visual diagram images
                    await renderMermaidDiagrams();
                    
                    // Apply syntax highlighting
                    document.querySelectorAll('pre code').forEach((block) => {
                        hljs.highlightElement(block);
                    });
                    
                    askBtn.disabled = false;
                    break;

                case 'error':
                    answerContainer.innerHTML = '<div class="error">❌ ' + escapeHtml(message.message) + '</div>';
                    askBtn.disabled = false;
                    break;
            }
        });

        function escapeHtml(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }
    </script>
</body>
</html>`;
    }

    public dispose() {
        AskPanel.currentPanel = undefined;
        this._panel.dispose();
        while (this._disposables.length) {
            const x = this._disposables.pop();
            if (x) {
                x.dispose();
            }
        }
    }
}
