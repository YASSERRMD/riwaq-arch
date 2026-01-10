// Main extension entry point
import * as vscode from 'vscode';
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
        vscode.commands.registerCommand('riwaq.refreshSnapshot', refreshSnapshot)
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

    const outputPath = await vscode.window.showInputBox({
        prompt: 'Output directory for documentation',
        value: './generated-docs',
        placeHolder: 'Path relative to workspace root'
    });

    if (!outputPath) return;

    await vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: 'Riwaq: Generating documentation...',
        cancellable: false
    }, async (progress) => {
        try {
            const workspacePath = workspaceFolders[0].uri.fsPath;
            const result = await client.generateDocs(workspacePath, outputPath);

            vscode.window.showInformationMessage(
                `Documentation generated: ${result.fileCount} files`,
                'Open Folder'
            ).then(selection => {
                if (selection === 'Open Folder') {
                    vscode.commands.executeCommand('vscode.openFolder',
                        vscode.Uri.file(`${workspacePath}/${outputPath}`),
                        { forceNewWindow: false }
                    );
                }
            });
        } catch (error: any) {
            vscode.window.showErrorMessage(`Riwaq: Doc generation failed - ${error.message}`);
        }
    });
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
