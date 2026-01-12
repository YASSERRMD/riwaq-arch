package com.riwaq.arch.client.models

/**
 * Request models for API calls.
 */
data class AnalyzeRequest(
    val projectPath: String,
    val excludedDirs: List<String> = emptyList()
)

data class AskRequest(
    val question: String
)

data class DocGenerationRequest(
    val path: String,
    val output: String,
    val skipLlm: Boolean = false
)

data class DocGenerationResponse(
    val success: Boolean,
    val fileCount: Int,
    val outputDir: String,
    val generationTimeMs: Long
)

/**
 * Response models from API calls.
 */
data class AnalyzeResponse(
    val success: Boolean,
    val message: String,
    val analysisId: String? = null
)

data class AnalysisData(
    val modules: List<ModuleInfo>,
    val services: List<ServiceInfo>,
    val stats: AnalysisStats
)

data class ModuleInfo(
    val name: String,
    val path: String,
    val type: String,
    val health: String, // "good", "warning", "poor"
    val metrics: ModuleMetrics,
    val dependencies: List<String>,
    val dependents: List<String>,
    val publicApi: List<String>,
    val description: String? = null
)

data class ServiceInfo(
    val name: String,
    val path: String,
    val entryPoints: List<String>,
    val description: String? = null
)

data class ModuleMetrics(
    val linesOfCode: Int,
    val functionCount: Int,
    val typeCount: Int,
    val coupling: Int,
    val cohesion: Double
)

data class AnalysisStats(
    val totalModules: Int,
    val totalServices: Int,
    val totalLinesOfCode: Int,
    val averageCoupling: Double,
    val averageCohesion: Double,
    val languages: Map<String, Int>
)

data class DiagramData(
    val mermaid: String,
    val stats: DiagramStats,
    val modules: List<DiagramModule>
)

data class DiagramStats(
    val totalModules: Int,
    val totalConnections: Int,
    val maxDepth: Int
)

data class DiagramModule(
    val id: String,
    val name: String,
    val type: String,
    val level: Int,
    val dependencies: List<String>,
    val dependents: List<String>
)

data class DocumentationData(
    val title: String,
    val content: String,
    val lastUpdated: String,
    val sections: List<DocSection>
)

data class DocSection(
    val title: String,
    val content: String,
    val module: String? = null
)

/**
 * Streaming response for Ask API.
 */
data class StreamResponse(
    val type: String, // "content", "file", "error", "done"
    val content: String? = null,
    val file: FileReference? = null,
    val confidence: Double? = null,
    val error: String? = null
)

data class FileReference(
    val path: String,
    val line: Int? = null,
    val description: String? = null
)
