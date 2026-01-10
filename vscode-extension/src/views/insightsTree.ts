// Insights tree view provider
import * as vscode from 'vscode';
import { RiwaqClient } from '../client';

export class InsightsTreeProvider implements vscode.TreeDataProvider<InsightItem> {
    private _onDidChangeTreeData = new vscode.EventEmitter<InsightItem | undefined | null | void>();
    readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

    constructor(private client: RiwaqClient) { }

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: InsightItem): vscode.TreeItem {
        return element;
    }

    getChildren(element?: InsightItem): Thenable<InsightItem[]> {
        if (!this.client.hasSnapshot()) {
            return Promise.resolve([
                new InsightItem(
                    'No insights available',
                    'Analyze workspace to see insights',
                    vscode.TreeItemCollapsibleState.None,
                    'info',
                    'info'
                )
            ]);
        }

        if (!element) {
            return Promise.resolve(this.getRootItems());
        }

        return Promise.resolve([]);
    }

    private getRootItems(): InsightItem[] {
        const snapshot = this.client.getSnapshot();
        if (!snapshot) return [];

        const items: InsightItem[] = [];

        // Language insights
        const primaryLang = snapshot.statistics.languages[0];
        if (primaryLang) {
            items.push(new InsightItem(
                '🔤 Primary Language',
                `${primaryLang.language} (${primaryLang.lineCount.toLocaleString()} lines)`,
                vscode.TreeItemCollapsibleState.None,
                'insight',
                'language'
            ));
        }

        // Code health
        const avgCoupling = this.calculateAverageCoupling();
        const healthStatus = avgCoupling < 0.3 ? '✅ Good' : avgCoupling < 0.6 ? '⚠️ Moderate' : '❌ Needs Work';
        items.push(new InsightItem(
            '💪 Code Health',
            healthStatus,
            vscode.TreeItemCollapsibleState.None,
            'insight',
            'health'
        ));

        // Module count
        items.push(new InsightItem(
            '📦 Modules',
            `${snapshot.modules.length} detected`,
            vscode.TreeItemCollapsibleState.None,
            'insight',
            'modules'
        ));

        // Services count
        if (snapshot.services.length > 0) {
            items.push(new InsightItem(
                '🚀 Services',
                `${snapshot.services.length} entry points`,
                vscode.TreeItemCollapsibleState.None,
                'insight',
                'services'
            ));
        }

        // Analysis time
        items.push(new InsightItem(
            '⏱️ Analysis Time',
            `${snapshot.metadata.analysisDurationMs}ms`,
            vscode.TreeItemCollapsibleState.None,
            'insight',
            'time'
        ));

        // Recommendations
        items.push(new InsightItem(
            '━━━━━━━━━━━━━━━━━━━',
            '',
            vscode.TreeItemCollapsibleState.None,
            'separator',
            'separator'
        ));

        items.push(new InsightItem(
            '💡 Quick Actions',
            '',
            vscode.TreeItemCollapsibleState.None,
            'header',
            'header'
        ));

        items.push(new InsightItem(
            '   📝 Generate Documentation',
            'Create markdown docs',
            vscode.TreeItemCollapsibleState.None,
            'action',
            'generateDocs',
            {
                command: 'riwaq.generateDocs',
                title: 'Generate Documentation'
            }
        ));

        items.push(new InsightItem(
            '   ❓ Ask About Code',
            'Get AI answers',
            vscode.TreeItemCollapsibleState.None,
            'action',
            'askQuestion',
            {
                command: 'riwaq.askQuestion',
                title: 'Ask Question'
            }
        ));

        items.push(new InsightItem(
            '   🏗️ View Architecture',
            'See diagram',
            vscode.TreeItemCollapsibleState.None,
            'action',
            'showArchitecture',
            {
                command: 'riwaq.showArchitecture',
                title: 'Show Architecture'
            }
        ));

        return items;
    }

    private calculateAverageCoupling(): number {
        const modules = this.client.getModules();
        if (modules.length === 0) return 0;

        const totalCoupling = modules.reduce((sum, m) => sum + m.metrics.couplingScore, 0);
        return totalCoupling / modules.length;
    }
}

class InsightItem extends vscode.TreeItem {
    constructor(
        public readonly label: string,
        public readonly description: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly contextValue: string,
        public readonly insightType: string,
        public readonly command?: vscode.Command
    ) {
        super(label, collapsibleState);
        this.tooltip = description;
    }
}
