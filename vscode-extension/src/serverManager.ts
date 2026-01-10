import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';

export class ServerManager {
    private serverProcess: cp.ChildProcess | null = null;
    private outputChannel: vscode.OutputChannel;
    private isStarting = false;

    constructor() {
        this.outputChannel = vscode.window.createOutputChannel('Riwaq Server');
    }

    async start(): Promise<boolean> {
        if (this.serverProcess || this.isStarting) {
            return true;
        }

        this.isStarting = true;
        this.outputChannel.appendLine('Starting Riwaq server...');

        const config = vscode.workspace.getConfiguration('riwaq');
        const executable = config.get<string>('serverPath') || 'riwaq';
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

                // Prompt user to select binary if it's not found
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
            });

            this.serverProcess.on('exit', (code) => {
                if (code !== 0 && code !== null) {
                    this.outputChannel.appendLine(`Server exited with code ${code}`);
                }
                this.serverProcess = null;
                this.isStarting = false;
            });

            // Wait a bit to ensure it started
            await new Promise(resolve => setTimeout(resolve, 2000));

            if (this.serverProcess && !this.serverProcess.killed) {
                this.outputChannel.appendLine('Server started successfully.');
                vscode.window.setStatusBarMessage('Riwaq server running', 3000);
                this.isStarting = false;
                return true;
            } else {
                this.isStarting = false;
                return false;
            }

        } catch (error: any) {
            this.outputChannel.appendLine(`Error starting server: ${error.message}`);
            this.isStarting = false;
            return false;
        }
    }

    async stop() {
        if (this.serverProcess) {
            this.outputChannel.appendLine('Stopping server...');
            this.serverProcess.kill();
            this.serverProcess = null;
            this.outputChannel.appendLine('Server stopped.');
        }
    }

    isRunning(): boolean {
        return this.serverProcess !== null && !this.serverProcess.killed;
    }
}
