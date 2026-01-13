// Main extension entry point
import * as vscode from 'vscode';
import * as path from 'path';
import { RiwaqClient } from './client';
import { ServerManager } from './serverManager';
import { ArchitectureTreeProvider } from './views/architectureTree';
import { ModulesTreeProvider } from './views/modulesTree';
import { InsightsTreeProvider } from './views/insightsTree';
import { AskPanel } from './panels/askPanel';
import { ArchitecturePanel } from './panels/architecturePanel';

let client: RiwaqClient;
let serverManager: ServerManager;
let architectureProvider: ArchitectureTreeProvider;
let modulesProvider: ModulesTreeProvider;
let insightsProvider: InsightsTreeProvider;
let serverStatusBar: vscode.StatusBarItem;

export async function activate(context: vscode.ExtensionContext) {
    console.log('Riwaq Arch extension is now active');

    // Initialize Status Bar Item
    serverStatusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    serverStatusBar.command = 'riwaq.showServerMenu';
    context.subscriptions.push(serverStatusBar);

    // Initialize the server manager
    serverManager = new ServerManager(context, serverStatusBar);

    // Initialize the client
    const config = vscode.workspace.getConfiguration('riwaq');
    client = new RiwaqClient(config.get('serverUrl') || 'http://127.0.0.1:9527');

    // Initialize tree providers
    architectureProvider = new ArchitectureTreeProvider(client);
    modulesProvider = new ModulesTreeProvider(client);
    insightsProvider = new InsightsTreeProvider(client);

    // Register tree views
    context.subscriptions.push(
        vscode.window.registerTreeDataProvider('riwaq.architecture', architectureProvider),
        vscode.window.registerTreeDataProvider('riwaq.modules', modulesProvider),
        vscode.window.registerTreeDataProvider('riwaq.insights', insightsProvider)
    );

    // Register commands
    context.subscriptions.push(
        vscode.commands.registerCommand('riwaq.startServer', () => serverManager.start()),
        vscode.commands.registerCommand('riwaq.stopServer', () => serverManager.stop()),
        vscode.commands.registerCommand('riwaq.showServerMenu', () => showServerMenu()),
        vscode.commands.registerCommand('riwaq.analyzeWorkspace', analyzeWorkspace),
        vscode.commands.registerCommand('riwaq.generateDocs', generateDocs),
        vscode.commands.registerCommand('riwaq.askQuestion', () => askQuestion(context)),
        vscode.commands.registerCommand('riwaq.showArchitecture', () => showArchitecture(context)),
        vscode.commands.registerCommand('riwaq.refreshSnapshot', refreshSnapshot),
        vscode.commands.registerCommand('riwaq.setApiKey', async () => {
            const secret = await vscode.window.showInputBox({
                prompt: 'Enter your LLM API Key (e.g. from OpenRouter, OpenAI)',
                password: true, // This masks the input with dots
                placeHolder: 'sk-...'
            });

            if (secret) {
                await context.secrets.store('riwaq.llmApiKey', secret);
                const selection = await vscode.window.showInformationMessage(
                    'API Key saved securely. You should restart the server to apply changes.',
                    'Restart Server'
                );

                if (selection === 'Restart Server') {
                    await serverManager.stop();
                    await serverManager.start();
                }
            }
        })
    );

    // Auto-start server if configured
    if (config.get('autoStartServer')) {
        const success = await serverManager.start();
        if (success && config.get('autoAnalyze')) {
            vscode.commands.executeCommand('riwaq.analyzeWorkspace');
        }
    } else {
        serverManager.updateStatus(false); // Show "Off" status
    }

    // Watch for configuration changes
    context.subscriptions.push(
        vscode.workspace.onDidChangeConfiguration(e => {
            if (e.affectsConfiguration('riwaq.serverUrl')) {
                const newUrl = vscode.workspace.getConfiguration('riwaq').get('serverUrl') || '';
                client.setServerUrl(newUrl as string);
            }
        })
    );
}

async function showServerMenu() {
    const isRunning = await serverManager.isRunning();
    const items = [];

    if (isRunning) {
        items.push({ label: '$(stop) Stop Server', description: 'Stop the Riwaq background process', command: 'riwaq.stopServer' });
        items.push({ label: '$(refresh) Restart Server', description: 'Reload with current config', command: 'riwaq.startServer' });
    } else {
        items.push({ label: '$(play) Start Server', description: 'Start the Riwaq background process', command: 'riwaq.startServer' });
    }

    items.push({ label: '$(settings) Configure Server Path', description: 'Set path to custom binary', command: 'workbench.action.openSettings', args: ['riwaq.serverPath'] });

    const selection = await vscode.window.showQuickPick(items, { placeHolder: 'Riwaq Server Control' });
    if (selection) {
        if (selection.command === 'workbench.action.openSettings' && selection.args) {
            vscode.commands.executeCommand(selection.command, selection.args[0]);
        } else {
            vscode.commands.executeCommand(selection.command);
        }
    }
}

async function analyzeWorkspace() {
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (!workspaceFolders || workspaceFolders.length === 0) {
        vscode.window.showErrorMessage('No workspace folder open');
        return;
    }

    const workspacePath = workspaceFolders[0].uri.fsPath;

    await vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: 'Riwaq: Analyzing workspace...',
        cancellable: false
    }, async (progress) => {
        try {
            progress.report({ message: 'Scanning files...' });
            await client.analyze(workspacePath);

            progress.report({ message: 'Refreshing views...' });
            architectureProvider.refresh();
            modulesProvider.refresh();
            insightsProvider.refresh();

            vscode.window.showInformationMessage('Riwaq: Workspace analysis complete!');
        } catch (error: any) {
            vscode.window.showErrorMessage(`Riwaq: Analysis failed - ${error.message} (Is the server running?)`);
        }
    });
}

// ... other functions (generateDocs, askQuestion, etc.) remain roughly the same, 
// just updating the error message in generateDocs slightly and copying them back

async function generateDocs() {
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (!workspaceFolders) {
        vscode.window.showErrorMessage('No workspace folder open');
        return;
    }

    const workspacePath = workspaceFolders[0].uri.fsPath;
    const outputPath = 'riwaq-generated-docs';
    const fullOutputPath = path.join(workspacePath, outputPath);

    try {
        // Get available document types from server
        const docTypesResult = await client.getDocTypes();

        // Show quick pick for document type selection
        const items = docTypesResult.docTypes.map(dt => ({
            label: dt.name,
            description: dt.id,
            detail: dt.description,
            id: dt.id
        }));

        // Add "All Documents" option at the top
        items.unshift({
            label: '📦 Generate All Documents',
            description: 'all',
            detail: 'Generate all 9 document types (may take several minutes)',
            id: 'all'
        });

        const selected = await vscode.window.showQuickPick(items, {
            placeHolder: 'Select a document type to generate',
            title: 'Riwaq: Generate Documentation'
        });

        if (!selected) {
            return; // User cancelled
        }

        if (selected.id === 'all') {
            // Generate all docs (old behavior)
            await generateAllDocs(workspacePath, fullOutputPath, outputPath);
        } else {
            // Generate single document
            await generateSingleDoc(workspacePath, fullOutputPath, selected.id, selected.label);
        }
    } catch (error: any) {
        vscode.window.showErrorMessage(`Riwaq: Failed to get document types - ${error.message}`);
    }
}

async function generateAllDocs(workspacePath: string, fullOutputPath: string, outputPath: string) {
    await vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: 'Riwaq: Generating all documentation...',
        cancellable: false
    }, async (progress) => {
        try {
            progress.report({ message: 'Analyzing codebase...' });

            if (!client.hasSnapshot()) {
                await client.analyze(workspacePath);
            }

            progress.report({ message: 'Generating all docs (this may take a while)...' });
            const result = await client.generateDocs(workspacePath, fullOutputPath);

            const selection = await vscode.window.showInformationMessage(
                `✅ Documentation generated: ${result.fileCount} files in '${outputPath}'`,
                'Open Folder',
                'Open in Explorer'
            );

            if (selection === 'Open Folder') {
                const folderUri = vscode.Uri.file(fullOutputPath);
                vscode.commands.executeCommand('vscode.openFolder', folderUri, { forceNewWindow: true });
            } else if (selection === 'Open in Explorer') {
                vscode.commands.executeCommand('revealFileInOS', vscode.Uri.file(fullOutputPath));
            }
        } catch (error: any) {
            vscode.window.showErrorMessage(`Riwaq: Doc generation failed - ${error.message}`);
        }
    });
}

async function generateSingleDoc(workspacePath: string, fullOutputPath: string, docType: string, docName: string) {
    const result = await vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: `Riwaq: Generating ${docName}...`,
        cancellable: false
    }, async (progress) => {
        try {
            progress.report({ message: 'Analyzing codebase...' });

            if (!client.hasSnapshot()) {
                await client.analyze(workspacePath);
            }

            progress.report({ message: `Generating ${docName}...` });
            return await client.generateSingleDoc(workspacePath, fullOutputPath, docType);
        } catch (error: any) {
            vscode.window.showErrorMessage(`Riwaq: Doc generation failed - ${error.message}`);
            return null;
        }
    });

    // Handle result AFTER progress completes
    if (result && result.success && result.filePath) {
        const selection = await vscode.window.showInformationMessage(
            `✅ ${docName} generated in ${Math.round(result.generationTimeMs / 1000)}s`,
            'Open Document',
            'Open Folder'
        );

        if (selection === 'Open Document') {
            const docUri = vscode.Uri.file(result.filePath);
            vscode.commands.executeCommand('markdown.showPreview', docUri);
        } else if (selection === 'Open Folder') {
            vscode.commands.executeCommand('revealFileInOS', vscode.Uri.file(result.filePath));
        }
    } else if (result && !result.success) {
        vscode.window.showErrorMessage(`Riwaq: Failed to generate ${docName} - ${result.error || 'Unknown error'}`);
    }
}

async function askQuestion(context: vscode.ExtensionContext) {
    AskPanel.createOrShow(context.extensionUri, client);
}

async function showArchitecture(context: vscode.ExtensionContext) {
    ArchitecturePanel.createOrShow(context.extensionUri, client);
}

async function refreshSnapshot() {
    architectureProvider.refresh();
    modulesProvider.refresh();
    insightsProvider.refresh();
    vscode.window.showInformationMessage('Riwaq: Views refreshed');
}

export function deactivate() {
    if (serverManager) {
        serverManager.stop();
    }
    console.log('Riwaq Arch extension is now deactivated');
}
