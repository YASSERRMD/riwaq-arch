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
            'Riwaq: Ask About Code',
            column || vscode.ViewColumn.One,
            {
                enableScripts: true,
                retainContextWhenHidden: true,
                localResourceRoots: [extensionUri]
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
                confidence: result.confidence
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
            font-size: 14px;
            font-weight: 500;
        }
        button:hover {
            background: var(--vscode-button-hoverBackground);
        }
        button:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }
        .answer-container {
            flex: 1;
            overflow-y: auto;
            padding: 20px;
            background: var(--vscode-editor-inactiveSelectionBackground);
            border-radius: 8px;
            border: 1px solid var(--vscode-panel-border);
        }
        .answer-content {
            line-height: 1.6;
            white-space: pre-wrap;
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
        }
        .file-ref:hover {
            background: var(--vscode-list-hoverBackground);
        }
        .confidence {
            font-size: 0.8rem;
            color: var(--vscode-descriptionForeground);
            margin-top: 15px;
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
            <li>How do modules communicate with each other?</li>
            <li>Where is the database connection managed?</li>
            <li>What are the main entry points?</li>
        </ul>
    </div>

    <script>
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

        window.addEventListener('message', event => {
            const message = event.data;

            switch (message.command) {
                case 'loading':
                    askBtn.disabled = message.loading;
                    if (message.loading) {
                        answerContainer.innerHTML = '<div class="loading"><div class="spinner"></div>Thinking...</div>';
                    }
                    break;

                case 'answer':
                    let html = '<div class="answer-content">' + escapeHtml(message.answer) + '</div>';
                    
                    if (message.fileRefs && message.fileRefs.length > 0) {
                        html += '<div class="file-refs"><h3>📁 Referenced Files</h3>';
                        for (const ref of message.fileRefs) {
                            html += '<div class="file-ref">' + escapeHtml(ref.path) + ' - ' + escapeHtml(ref.relevance) + '</div>';
                        }
                        html += '</div>';
                    }

                    html += '<div class="confidence">Confidence: ' + Math.round(message.confidence * 100) + '%</div>';
                    
                    answerContainer.innerHTML = html;
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
