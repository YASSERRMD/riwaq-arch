// Modules tree view provider
import * as vscode from 'vscode';
import { RiwaqClient, ModuleSummary } from '../client';

export class ModulesTreeProvider implements vscode.TreeDataProvider<ModuleItem> {
    private _onDidChangeTreeData = new vscode.EventEmitter<ModuleItem | undefined | null | void>();
    readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

    constructor(private client: RiwaqClient) { }

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: ModuleItem): vscode.TreeItem {
        return element;
    }

    getChildren(element?: ModuleItem): Thenable<ModuleItem[]> {
        if (!this.client.hasSnapshot()) {
            return Promise.resolve([
                new ModuleItem(
                    'No modules available',
                    '',
                    vscode.TreeItemCollapsibleState.None,
                    'info'
                )
            ]);
        }

        if (!element) {
            // Root level - list all modules
            return Promise.resolve(this.getModuleItems());
        }

        // Children of a module
        if (element.module) {
            return Promise.resolve(this.getModuleDetails(element.module));
        }

        return Promise.resolve([]);
    }

    private getModuleItems(): ModuleItem[] {
        const modules = this.client.getModules();
        return modules.map(module => {
            const healthIcon = this.getHealthIcon(module.metrics);
            return new ModuleItem(
                `${healthIcon} ${module.name}`,
                `${module.files.length} files`,
                vscode.TreeItemCollapsibleState.Collapsed,
                'module',
                module
            );
        });
    }

    private getModuleDetails(module: ModuleSummary): ModuleItem[] {
        const items: ModuleItem[] = [];

        // Metrics section
        items.push(new ModuleItem(
            '📊 Metrics',
            '',
            vscode.TreeItemCollapsibleState.None,
            'header'
        ));
        items.push(new ModuleItem(
            `   Lines: ${module.metrics.linesOfCode.toLocaleString()}`,
            '',
            vscode.TreeItemCollapsibleState.None,
            'metric'
        ));
        items.push(new ModuleItem(
            `   Functions: ${module.metrics.functionCount}`,
            '',
            vscode.TreeItemCollapsibleState.None,
            'metric'
        ));
        items.push(new ModuleItem(
            `   Types: ${module.metrics.typeCount}`,
            '',
            vscode.TreeItemCollapsibleState.None,
            'metric'
        ));
        items.push(new ModuleItem(
            `   Coupling: ${(module.metrics.couplingScore * 100).toFixed(0)}%`,
            this.getCouplingDescription(module.metrics.couplingScore),
            vscode.TreeItemCollapsibleState.None,
            'metric'
        ));
        items.push(new ModuleItem(
            `   Cohesion: ${(module.metrics.cohesionScore * 100).toFixed(0)}%`,
            this.getCohesionDescription(module.metrics.cohesionScore),
            vscode.TreeItemCollapsibleState.None,
            'metric'
        ));

        // Public API section
        if (module.publicFunctions.length > 0 || module.publicTypes.length > 0) {
            items.push(new ModuleItem(
                '📚 Public API',
                '',
                vscode.TreeItemCollapsibleState.None,
                'header'
            ));

            for (const func of module.publicFunctions.slice(0, 10)) {
                items.push(new ModuleItem(
                    `   ƒ ${func}`,
                    'function',
                    vscode.TreeItemCollapsibleState.None,
                    'function'
                ));
            }

            for (const type of module.publicTypes.slice(0, 10)) {
                items.push(new ModuleItem(
                    `   ◆ ${type}`,
                    'type',
                    vscode.TreeItemCollapsibleState.None,
                    'type'
                ));
            }
        }

        // Dependencies section
        if (module.dependencies.length > 0) {
            items.push(new ModuleItem(
                '🔗 Dependencies',
                `${module.dependencies.length} deps`,
                vscode.TreeItemCollapsibleState.None,
                'header'
            ));

            for (const dep of module.dependencies.slice(0, 5)) {
                items.push(new ModuleItem(
                    `   → ${dep.target}`,
                    `${dep.referenceCount} refs`,
                    vscode.TreeItemCollapsibleState.None,
                    'dependency'
                ));
            }
        }

        return items;
    }

    private getHealthIcon(metrics: ModuleSummary['metrics']): string {
        // Health based on coupling (lower is better)
        if (metrics.couplingScore < 0.3) return '🟢';
        if (metrics.couplingScore < 0.6) return '🟡';
        return '🔴';
    }

    private getCouplingDescription(score: number): string {
        if (score < 0.3) return 'Low coupling - well isolated';
        if (score < 0.6) return 'Moderate coupling';
        return 'High coupling - consider refactoring';
    }

    private getCohesionDescription(score: number): string {
        if (score > 0.7) return 'High cohesion - focused module';
        if (score > 0.4) return 'Moderate cohesion';
        return 'Low cohesion - may need splitting';
    }
}

class ModuleItem extends vscode.TreeItem {
    constructor(
        public readonly label: string,
        public readonly description: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly contextValue: string,
        public readonly module?: ModuleSummary
    ) {
        super(label, collapsibleState);
        this.tooltip = module ? `Path: ${module.path}` : description;
    }
}
