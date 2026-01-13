// HTTP client for communicating with the Riwaq backend
import axios, { AxiosInstance } from 'axios';

export interface AnalysisResult {
    success: boolean;
    snapshot: CodebaseSnapshot;
}

export interface CodebaseSnapshot {
    metadata: SnapshotMetadata;
    files: FileSummary[];
    modules: ModuleSummary[];
    services: ServiceInfo[];
    statistics: CodebaseStatistics;
}

export interface SnapshotMetadata {
    projectName: string;
    rootPath: string;
    analyzerVersion: string;
    createdAt: string;
    analysisDurationMs: number;
}

export interface FileSummary {
    path: string;
    language: string;
    lineCount: number;
    functions: FunctionSummary[];
    types: TypeSummary[];
    isTest: boolean;
}

export interface FunctionSummary {
    name: string;
    visibility: string;
    lineCount: number;
    parameters: string[];
    returnType?: string;
    docstring?: string;
}

export interface TypeSummary {
    name: string;
    kind: string;
    visibility: string;
    fields: string[];
    methods: string[];
}

export interface ModuleSummary {
    name: string;
    path: string;
    files: string[];
    publicFunctions: string[];
    publicTypes: string[];
    dependencies: ModuleDependency[];
    metrics: ModuleMetrics;
}

export interface ModuleDependency {
    target: string;
    kind: string;
    referenceCount: number;
}

export interface ModuleMetrics {
    fileCount: number;
    linesOfCode: number;
    functionCount: number;
    typeCount: number;
    couplingScore: number;
    cohesionScore: number;
}

export interface ServiceInfo {
    name: string;
    kind: string;
    entryFile: string;
    endpoints: string[];
}

export interface CodebaseStatistics {
    totalFiles: number;
    totalModules: number;
    totalFunctions: number;
    totalTypes: number;
    totalLines: number;
    languages: LanguageCount[];
}

export interface LanguageCount {
    language: string;
    fileCount: number;
    lineCount: number;
}

export interface DocGenerationResult {
    success: boolean;
    fileCount: number;
    outputDir: string;
    generationTimeMs: number;
}

export interface AnswerResult {
    answer: string;
    fileRefs: FileReference[];
    confidence: number;
    diagrams?: DiagramData[];
}

export interface DiagramData {
    name: string;
    svg: string;
}

export interface FileReference {
    path: string;
    line?: number;
    snippet?: string;
    relevance: string;
}

export interface DocTypeInfo {
    id: string;
    name: string;
    description: string;
}

export interface DocTypesResult {
    docTypes: DocTypeInfo[];
}

export interface SingleDocResult {
    success: boolean;
    docType: string;
    filePath?: string;
    generationTimeMs: number;
    error?: string;
}

export class RiwaqClient {
    private client: AxiosInstance;
    private snapshot: CodebaseSnapshot | null = null;

    constructor(serverUrl: string) {
        this.client = axios.create({
            baseURL: serverUrl,
            timeout: 120000, // 2 minute timeout for analysis
            headers: {
                'Content-Type': 'application/json'
            }
        });
    }

    setServerUrl(url: string) {
        this.client.defaults.baseURL = url;
    }

    async analyze(workspacePath: string): Promise<AnalysisResult> {
        const response = await this.client.post('/analyze', {
            path: workspacePath,
            maxCommits: 500,
            includeTests: false
        });

        this.snapshot = response.data.snapshot;
        return response.data;
    }

    async generateDocs(workspacePath: string, outputPath: string): Promise<DocGenerationResult> {
        const response = await this.client.post('/docgen', {
            path: workspacePath,
            output: outputPath,
            skipLlm: false
        });
        return response.data;
    }

    async askQuestion(question: string): Promise<AnswerResult> {
        if (!this.snapshot) {
            throw new Error('No analysis snapshot available. Please analyze the workspace first.');
        }

        const response = await this.client.post('/ask', {
            question,
            snapshotId: this.snapshot.metadata.projectName
        });
        return response.data;
    }

    async getArchitectureDiagram(): Promise<string> {
        if (!this.snapshot) {
            throw new Error('No analysis snapshot available.');
        }

        const response = await this.client.get('/architecture/diagram', {
            params: { format: 'mermaid' }
        });
        return response.data.diagram;
    }

    getSnapshot(): CodebaseSnapshot | null {
        return this.snapshot;
    }

    hasSnapshot(): boolean {
        return this.snapshot !== null;
    }

    // Helper methods for tree views
    getModules(): ModuleSummary[] {
        return this.snapshot?.modules || [];
    }

    getServices(): ServiceInfo[] {
        return this.snapshot?.services || [];
    }

    getStatistics(): CodebaseStatistics | null {
        return this.snapshot?.statistics || null;
    }

    async getDocTypes(): Promise<DocTypesResult> {
        const response = await this.client.get('/docgen/types');
        return response.data;
    }

    async generateSingleDoc(workspacePath: string, outputPath: string, docType: string): Promise<SingleDocResult> {
        const response = await this.client.post('/docgen/single', {
            path: workspacePath,
            output: outputPath,
            docType: docType
        }, {
            timeout: 300000 // 5 minute timeout for single doc generation
        });
        return response.data;
    }
}
