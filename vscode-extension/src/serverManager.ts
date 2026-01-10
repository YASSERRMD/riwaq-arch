import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

export class ServerManager {
    private serverProcess: cp.ChildProcess | null = null;
    private outputChannel: vscode.OutputChannel;
    private isStarting = false;

    constructor(
        private context: vscode.ExtensionContext,
        private statusBar: vscode.StatusBarItem
    ) {
        this.outputChannel = vscode.window.createOutputChannel('Riwaq Server');
        this.updateStatus(false);
    }

    async start(): Promise<boolean> {
        if (this.serverProcess || this.isStarting) {
            return true;
        }

        this.isStarting = true;
        this.updateStatus(false, true); // Starting...
        this.outputChannel.appendLine('Starting Riwaq server...');

        const config = vscode.workspace.getConfiguration('riwaq');
        let executable = config.get<string>('serverPath');

        // Auto-detect bundled binary if not configured
        if (!executable) {
            const bundledPath = path.join(this.context.extensionUri.fsPath, 'bin', 'riwaq');
            if (fs.existsSync(bundledPath)) {
                executable = bundledPath;
                this.outputChannel.appendLine(`Using bundled binary at: ${executable}`);
            } else {
                executable = 'riwaq'; // Fallback to PATH
            }
        }

        const serverUrl = config.get<string>('serverUrl') || 'http://127.0.0.1:9527';

        // Parse port from URL
        let port = 9527;
        try {
            const url = new URL(serverUrl);
            if (url.port) port = parseInt(url.port);
        } catch (e) {
            // Ignore parse error, use default
        }

        // Get LLM configuration
        const env = { ...process.env };
        const apiKey = config.get<string>('llmApiKey');
        const endpoint = config.get<string>('llmEndpoint');
        const model = config.get<string>('llmModel');

        if (apiKey) env['RIWAQ_LLM_API_KEY'] = apiKey;
        if (endpoint) env['RIWAQ_LLM_ENDPOINT'] = endpoint;
        if (model) env['RIWAQ_LLM_MODEL'] = model;

        try {
            this.serverProcess = cp.spawn(executable, ['serve', '--port', port.toString()], {
                env,
                stdio: 'pipe'
            });

            this.serverProcess.stdout?.on('data', (data) => {
                this.outputChannel.append(data.toString());
            });

            this.serverProcess.stderr?.on('data', (data) => {
                this.outputChannel.append(data.toString());
            });

            this.serverProcess.on('error', (error) => {
                this.outputChannel.appendLine(`Failed to start server: ${error.message}`);

                if ((error as any).code === 'ENOENT') {
                    vscode.window.showErrorMessage(
                        `Riwaq server binary not found. Please install it or configure the path.`,
                        'Select Binary', 'Download Instructions'
                    ).then(selection => {
                        if (selection === 'Select Binary') {
                            vscode.window.showOpenDialog({
                                canSelectFiles: true,
                                canSelectFolders: false,
                                canSelectMany: false,
                                openLabel: 'Select Riwaq Binary'
                            }).then(uris => {
                                if (uris && uris.length > 0) {
                                    config.update('serverPath', uris[0].fsPath, vscode.ConfigurationTarget.Global);
                                    vscode.window.showInformationMessage('Path updated. Please reload window or run "Start Server".');
                                }
                            });
                        } else if (selection === 'Download Instructions') {
                            vscode.env.openExternal(vscode.Uri.parse('https://github.com/YASSERRMD/riwaq-arch#installation'));
                        }
                    });
                } else {
                    vscode.window.showErrorMessage(`Riwaq: Failed to start server (${error.message})`);
                }

                this.serverProcess = null;
                this.isStarting = false;
                this.updateStatus(false);
            });

            this.serverProcess.on('exit', (code) => {
                if (code !== 0 && code !== null) {
                    this.outputChannel.appendLine(`Server exited with code ${code}`);
                }
                this.serverProcess = null;
                this.isStarting = false;
                this.updateStatus(false);
            });

            // Wait a bit to ensure it started
            await new Promise(resolve => setTimeout(resolve, 2000));

            if (this.serverProcess && !this.serverProcess.killed) {
                this.outputChannel.appendLine('Server started successfully.');
                this.updateStatus(true);
                this.isStarting = false;
                return true;
            } else {
                this.isStarting = false;
                this.updateStatus(false);
                return false;
            }

        } catch (error: any) {
            this.outputChannel.appendLine(`Error starting server: ${error.message}`);
            this.isStarting = false;
            this.updateStatus(false);
            return false;
        }
    }

    async stop() {
        if (this.serverProcess) {
            this.outputChannel.appendLine('Stopping server...');
            this.serverProcess.kill();
            this.serverProcess = null;
            this.outputChannel.appendLine('Server stopped.');
            this.updateStatus(false);
        }
    }

    isRunning(): boolean {
        return this.serverProcess !== null && !this.serverProcess.killed;
    }

    updateStatus(running: boolean, starting: boolean = false) {
        if (starting) {
            this.statusBar.text = '$(sync~spin) Riwaq: Starting...';
            this.statusBar.tooltip = 'Riwaq Server Is Starting';
            this.statusBar.backgroundColor = undefined;
        } else if (running) {
            this.statusBar.text = '$(zap) Riwaq: On';
            this.statusBar.tooltip = 'Riwaq Server Running (Click for menu)';
            this.statusBar.backgroundColor = undefined;
        } else {
            this.statusBar.text = '$(stop) Riwaq: Off';
            this.statusBar.tooltip = 'Riwaq Server Stopped (Click for menu)';
        }
        this.statusBar.show();
    }
}
