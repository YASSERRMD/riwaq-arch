// Architecture tree view provider
import * as vscode from 'vscode';
import { RiwaqClient } from '../client';

export class ArchitectureTreeProvider implements vscode.TreeDataProvider<ArchitectureItem> {
    private _onDidChangeTreeData = new vscode.EventEmitter<ArchitectureItem | undefined | null | void>();
    readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

    constructor(private client: RiwaqClient) { }

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: ArchitectureItem): vscode.TreeItem {
        return element;
    }

    getChildren(element?: ArchitectureItem): Thenable<ArchitectureItem[]> {
        if (!this.client.hasSnapshot()) {
            return Promise.resolve([
                new ArchitectureItem(
                    'No analysis available',
                    'Click "Analyze Workspace" to start',
                    vscode.TreeItemCollapsibleState.None,
                    'info'
                )
            ]);
        }

        if (!element) {
            // Root level
            return Promise.resolve(this.getRootItems());
        }

        // Children based on type
        switch (element.contextValue) {
            case 'services':
                return Promise.resolve(this.getServiceItems());
            case 'statistics':
                return Promise.resolve(this.getStatisticsItems());
            default:
                return Promise.resolve([]);
        }
    }

    private getRootItems(): ArchitectureItem[] {
        const snapshot = this.client.getSnapshot();
        if (!snapshot) return [];

        return [
            new ArchitectureItem(
                `📊 ${snapshot.metadata.projectName}`,
                `Analyzed ${snapshot.statistics.totalFiles} files`,
                vscode.TreeItemCollapsibleState.None,
                'project'
            ),
            new ArchitectureItem(
                '🔧 Services',
                `${snapshot.services.length} entry points`,
                vscode.TreeItemCollapsibleState.Collapsed,
                'services'
            ),
            new ArchitectureItem(
                '📈 Statistics',
                'View metrics',
                vscode.TreeItemCollapsibleState.Collapsed,
                'statistics'
            )
        ];
    }

    private getServiceItems(): ArchitectureItem[] {
        const services = this.client.getServices();
        return services.map(service => {
            const icon = this.getServiceIcon(service.kind);
            return new ArchitectureItem(
                `${icon} ${service.name}`,
                `${service.kind} - ${service.entryFile}`,
                vscode.TreeItemCollapsibleState.None,
                'service',
                {
                    command: 'vscode.open',
                    title: 'Open Entry File',
                    arguments: [vscode.Uri.file(service.entryFile)]
                }
            );
        });
    }

    private getStatisticsItems(): ArchitectureItem[] {
        const stats = this.client.getStatistics();
        if (!stats) return [];

        return [
            new ArchitectureItem(`Files: ${stats.totalFiles}`, '', vscode.TreeItemCollapsibleState.None, 'stat'),
            new ArchitectureItem(`Modules: ${stats.totalModules}`, '', vscode.TreeItemCollapsibleState.None, 'stat'),
            new ArchitectureItem(`Functions: ${stats.totalFunctions}`, '', vscode.TreeItemCollapsibleState.None, 'stat'),
            new ArchitectureItem(`Types: ${stats.totalTypes}`, '', vscode.TreeItemCollapsibleState.None, 'stat'),
            new ArchitectureItem(`Lines: ${stats.totalLines.toLocaleString()}`, '', vscode.TreeItemCollapsibleState.None, 'stat'),
            ...stats.languages.map(lang =>
                new ArchitectureItem(
                    `${lang.language}: ${lang.fileCount} files`,
                    `${lang.lineCount.toLocaleString()} lines`,
                    vscode.TreeItemCollapsibleState.None,
                    'language'
                )
            )
        ];
    }

    private getServiceIcon(kind: string): string {
        switch (kind.toLowerCase()) {
            case 'httpserver': return '🌐';
            case 'grpcserver': return '⚡';
            case 'graphqlserver': return '◈';
            case 'websocket': return '🔌';
            case 'cli': return '💻';
            case 'worker': return '⚙️';
            default: return '📦';
        }
    }
}

class ArchitectureItem extends vscode.TreeItem {
    constructor(
        public readonly label: string,
        public readonly description: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly contextValue: string,
        public readonly command?: vscode.Command
    ) {
        super(label, collapsibleState);
        this.tooltip = description;
    }
}
