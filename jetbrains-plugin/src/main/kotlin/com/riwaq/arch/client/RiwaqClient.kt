package com.riwaq.arch.client

import com.google.gson.Gson
import com.google.gson.JsonSyntaxException
import com.intellij.openapi.components.Service
import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.project.Project
import com.riwaq.arch.client.models.*
import com.riwaq.arch.settings.RiwaqSettings
import okhttp3.*
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.RequestBody.Companion.toRequestBody
import okhttp3.sse.EventSource
import okhttp3.sse.EventSourceListener
import okhttp3.sse.EventSources
import org.jetbrains.annotations.Nullable
import java.io.IOException
import java.util.concurrent.TimeUnit

/**
 * Client for communicating with the Riwaq server.
 */
@Service(Service.Level.PROJECT)
class RiwaqClient(val project: Project) {

    companion object {
        private val LOG = Logger.getInstance(RiwaqClient::class.java)
        private val JSON = "application/json; charset=utf-8".toMediaType()

        fun getInstance(project: Project): RiwaqClient = project.service()
    }

    private val settings = RiwaqSettings.getInstance()
    private val client = OkHttpClient.Builder()
        .connectTimeout(30, TimeUnit.SECONDS)
        .readTimeout(60, TimeUnit.SECONDS)
        .writeTimeout(60, TimeUnit.SECONDS)
        .build()

    private val gson = Gson()

    /**
     * Get the base URL for the server.
     */
    private fun getBaseUrl(): String {
        return settings.serverUrl
    }

    /**
     * Check if the server is available.
     */
    suspend fun isServerAvailable(): Boolean {
        return try {
            val request = Request.Builder()
                .url("${getBaseUrl()}/health")
                .get()
                .build()

            val response = client.newCall(request).execute()
            response.isSuccessful
        } catch (e: Exception) {
            LOG.warn("Server health check failed: ${e.message}")
            false
        }
    }

    /**
     * Analyze the current project.
     */
    suspend fun analyzeProject(): Result<AnalyzeResponse> {
        return try {
            val projectPath = project.basePath ?: run {
                return Result.failure(IOException("Project path is null"))
            }

            val request = AnalyzeRequest(
                projectPath = projectPath,
                excludedDirs = settings.getExcludedDirsList()
            )

            val body = gson.toJson(request).toRequestBody(JSON)

            val httpRequest = Request.Builder()
                .url("${getBaseUrl()}/api/analyze")
                .post(body)
                .build()

            val response = client.newCall(httpRequest).execute()
            val responseBody = response.body?.string() ?: run {
                return Result.failure(IOException("Empty response body"))
            }

            if (response.isSuccessful) {
                val result = gson.fromJson(responseBody, AnalyzeResponse::class.java)
                Result.success(result)
            } else {
                Result.failure(IOException("Server error: ${response.code} - $responseBody"))
            }
        } catch (e: Exception) {
            LOG.error("Failed to analyze project", e)
            Result.failure(e)
        }
    }

    /**
     * Get the analysis results.
     */
    suspend fun getAnalysis(): Result<AnalysisData> {
        return try {
            val request = Request.Builder()
                .url("${getBaseUrl()}/api/analysis")
                .get()
                .build()

            val response = client.newCall(request).execute()
            val responseBody = response.body?.string() ?: run {
                return Result.failure(IOException("Empty response body"))
            }

            if (response.isSuccessful) {
                val result = gson.fromJson(responseBody, AnalysisData::class.java)
                Result.success(result)
            } else {
                Result.failure(IOException("Server error: ${response.code} - $responseBody"))
            }
        } catch (e: Exception) {
            LOG.error("Failed to get analysis", e)
            Result.failure(e)
        }
    }

    /**
     * Ask a question about the codebase.
     */
    suspend fun askQuestion(question: String, callback: (streamResponse: StreamResponse) -> Unit): Result<Unit> {
        return try {
            val request = AskRequest(question = question)

            val body = gson.toJson(request).toRequestBody(JSON)

            val httpRequest = Request.Builder()
                .url("${getBaseUrl()}/api/ask")
                .post(body)
                .build()

            val eventSourceListener = object : EventSourceListener() {
                override fun onOpen(eventSource: EventSource, response: Response) {
                    super.onOpen(eventSource, response)
                }

                override fun onEvent(
                    eventSource: EventSource,
                    id: String?,
                    type: String?,
                    data: String
                ) {
                    super.onEvent(eventSource, id, type, data)
                    try {
                        val streamResponse = gson.fromJson(data, StreamResponse::class.java)
                        callback(streamResponse)
                    } catch (e: JsonSyntaxException) {
                        LOG.warn("Failed to parse SSE data: $data", e)
                    }
                }

                override fun onClosed(eventSource: EventSource) {
                    super.onClosed(eventSource)
                }

                override fun onFailure(
                    eventSource: EventSource,
                    t: Throwable?,
                    response: Response?
                ) {
                    super.onFailure(eventSource, t, response)
                    LOG.error("SSE connection failed", t)
                }
            }

            val factory = EventSources.createFactory(client)
            factory.newEventSource(httpRequest, eventSourceListener)

            Result.success(Unit)
        } catch (e: Exception) {
            LOG.error("Failed to ask question", e)
            Result.failure(e)
        }
    }

    /**
     * Get architecture diagram data.
     */
    suspend fun getArchitectureDiagram(): Result<DiagramData> {
        return try {
            val request = Request.Builder()
                .url("${getBaseUrl()}/api/diagram")
                .get()
                .build()

            val response = client.newCall(request).execute()
            val responseBody = response.body?.string() ?: run {
                return Result.failure(IOException("Empty response body"))
            }

            if (response.isSuccessful) {
                val result = gson.fromJson(responseBody, DiagramData::class.java)
                Result.success(result)
            } else {
                Result.failure(IOException("Server error: ${response.code} - $responseBody"))
            }
        } catch (e: Exception) {
            LOG.error("Failed to get architecture diagram", e)
            Result.failure(e)
        }
    }

    /**
     * Get documentation for the codebase.
     */
    suspend fun getDocumentation(): Result<DocumentationData> {
        return try {
            val request = Request.Builder()
                .url("${getBaseUrl()}/api/docs")
                .get()
                .build()

            val response = client.newCall(request).execute()
            val responseBody = response.body?.string() ?: run {
                return Result.failure(IOException("Empty response body"))
            }

            if (response.isSuccessful) {
                val result = gson.fromJson(responseBody, DocumentationData::class.java)
                Result.success(result)
            } else {
                Result.failure(IOException("Server error: ${response.code} - $responseBody"))
            }
        } catch (e: Exception) {
            LOG.error("Failed to get documentation", e)
            Result.failure(e)
        }
    }
}
