package com.riwaq.arch.server

import com.intellij.execution.ExecutionException
import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.execution.process.*
import com.intellij.execution.ui.ConsoleView
import com.intellij.execution.ui.ConsoleViewContentType
import com.intellij.openapi.Disposable
import com.intellij.openapi.components.Service
import com.intellij.openapi.components.service
import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.project.Project
import com.intellij.openapi.util.Key
import com.riwaq.arch.settings.RiwaqSettings
import kotlinx.coroutines.*
import java.io.File
import java.util.concurrent.atomic.AtomicBoolean

/**
 * Manages the Riwaq server process lifecycle.
 */
@Service(Service.Level.APP)
class ServerManager : Disposable {

    companion object {
        private val LOG = Logger.getInstance(ServerManager::class.java)

        fun getInstance(): ServerManager = service()

        private val SERVER_STARTUP_TIMEOUT = 30000L
        private val SERVER_SHUTDOWN_TIMEOUT = 5000L
    }

    private val scope = CoroutineScope(Dispatchers.IO + SupervisorJob())
    private var processHandler: OSProcessHandler? = null
    private var serverProcess: Process? = null
    private val isRunning = AtomicBoolean(false)
    private val startupJobHolder = mutableListOf<Job>()

    /**
     * Check if the server is currently running.
     */
    fun isServerRunning(): Boolean = isRunning.get()

    /**
     * Start the Riwaq server.
     * @return true if the server was started successfully or is already running
     */
    suspend fun startServer(): Boolean = withContext(Dispatchers.IO) {
        if (isRunning.get()) {
            LOG.info("Server is already running")
            return@withContext true
        }

        val settings = RiwaqSettings.getInstance()
        val serverPath = findServerPath(settings) ?: run {
            LOG.error("Could not find Riwaq server binary")
            return@withContext false
        }

        LOG.info("Starting Riwaq server from: $serverPath")

        try {
            val command = GeneralCommandLine(serverPath)
            command.withWorkDirectory(null as String?)

            // Pass environment variables for LLM configuration
            if (settings.llmApiKey.isNotEmpty()) {
                command.environment["LLM_API_KEY"] = settings.llmApiKey
            }
            if (settings.llmEndpoint.isNotEmpty()) {
                command.environment["LLM_ENDPOINT"] = settings.llmEndpoint
            }
            if (settings.llmModel.isNotEmpty()) {
                command.environment["LLM_MODEL"] = settings.llmModel
            }

            // Start the process
            val process = command.createProcess()
            serverProcess = process

            // Create process handler
            processHandler = object : OSProcessHandler(command) {
                override fun notifyTextAvailable(text: String, key: Key<*>) {
                    super.notifyTextAvailable(text, key)
                    LOG.info("Server: $text")
                }
            }

            processHandler?.addProcessListener(object : ProcessAdapter() {
                override fun processTerminated(event: ProcessEvent) {
                    super.processTerminated(event)
                    LOG.info("Server process terminated with exit code: ${event.exitCode}")
                    isRunning.set(false)
                }

                override fun onTextAvailable(event: ProcessEvent, outputType: Key<*>) {
                    super.onTextAvailable(event, outputType)
                    val text = event.text
                    if (text.contains("Server started") || text.contains("listening")) {
                        LOG.info("Server appears to be ready")
                    }
                }
            })

            processHandler?.startNotify()
            isRunning.set(true)

            // Wait for server to be ready
            waitForServerReady(settings.serverUrl)

            LOG.info("Riwaq server started successfully")
            return@withContext true
        } catch (e: Exception) {
            LOG.error("Failed to start Riwaq server", e)
            isRunning.set(false)
            return@withContext false
        }
    }

    /**
     * Stop the Riwaq server.
     */
    suspend fun stopServer(): Boolean = withContext(Dispatchers.IO) {
        if (!isRunning.get()) {
            LOG.info("Server is not running")
            return@withContext true
        }

        LOG.info("Stopping Riwaq server")

        // Cancel any pending startup jobs
        startupJobHolder.forEach { it.cancel() }
        startupJobHolder.clear()

        try {
            // Try graceful shutdown first
            processHandler?.destroyProcess()
            serverProcess?.destroy()

            withTimeoutOrNull(SERVER_SHUTDOWN_TIMEOUT) {
                while (isRunning.get()) {
                    delay(100)
                }
            } ?: run {
                LOG.warn("Server did not shut down gracefully, forcing...")
                serverProcess?.destroyForcibly()
            }

            processHandler = null
            serverProcess = null
            isRunning.set(false)

            LOG.info("Riwaq server stopped")
            return@withContext true
        } catch (e: Exception) {
            LOG.error("Error stopping server", e)
            return@withContext false
        }
    }

    /**
     * Restart the server.
     */
    suspend fun restartServer(): Boolean {
        stopServer()
        delay(1000) // Give it a moment to fully stop
        return startServer()
    }

    /**
     * Wait for the server to be ready by checking the health endpoint.
     */
    private suspend fun waitForServerReady(serverUrl: String): Boolean {
        val client = okhttp3.OkHttpClient.Builder().build()
        val request = okhttp3.Request.Builder()
            .url("$serverUrl/health")
            .build()

        return withTimeoutOrNull(SERVER_STARTUP_TIMEOUT) {
            var attempts = 0
            while (true) {
                try {
                    val response = client.newCall(request).execute()
                    if (response.isSuccessful) {
                        LOG.info("Server health check passed")
                        return@withTimeoutOrNull true
                    }
                } catch (e: Exception) {
                    // Server not ready yet, try again
                    attempts++
                    if (attempts % 10 == 0) {
                        LOG.debug("Waiting for server to be ready... (attempt $attempts)")
                    }
                }
                delay(500)
            }
            false
        } ?: run {
            LOG.warn("Server did not become ready within timeout")
            false
        }
    }

    /**
     * Find the server binary path based on settings and defaults.
     */
    private fun findServerPath(settings: RiwaqSettings): String? {
        // User-configured path takes priority
        if (settings.serverPath.isNotEmpty()) {
            val file = File(settings.serverPath)
            if (file.exists() && file.canExecute()) {
                return settings.serverPath
            }
            LOG.warn("Configured server path does not exist or is not executable: ${settings.serverPath}")
        }

        // Check bundled binary
        val bundledPath = getBundledServerPath()
        if (bundledPath != null) {
            return bundledPath
        }

        // Check system PATH
        return try {
            val whichCommand = if (System.getProperty("os.name").contains("Windows")) {
                "where riwaq"
            } else {
                "which riwaq"
            }
            val process = Runtime.getRuntime().exec(whichCommand)
            val exitCode = process.waitFor()
            if (exitCode == 0) {
                process.inputStream.bufferedReader().readText().trim().takeIf { it.isNotEmpty() }
            } else {
                null
            }
        } catch (e: Exception) {
            LOG.warn("Could not check system PATH for riwaq binary", e)
            null
        }
    }

    /**
     * Get the path to the bundled server binary.
     */
    private fun getBundledServerPath(): String? {
        val os = System.getProperty("os.name").lowercase()
        val arch = System.getProperty("os.arch").lowercase()

        val binaryName = when {
            os.contains("win") -> "riwaq.exe"
            else -> "riwaq"
        }

        // Try multiple possible locations for the bundled binary
        val possiblePaths = listOf(
            // Plugin lib/bin directory (where Gradle copies it)
            File(System.getProperty("user.dir"), "lib/bin/$binaryName"),
            // Alternative plugin installation locations
            File(System.getProperty("user.dir"), "lib/$binaryName"),
            File(System.getProperty("java.io.tmpdir"), "Riwaq Arch/lib/bin/$binaryName"),
            // Development environment
            File("${System.getProperty("user.dir")}/../../vscode-extension/bin", binaryName)
        )

        for (path in possiblePaths) {
            if (path.exists() && path.canExecute()) {
                LOG.info("Found bundled server at: ${path.absolutePath}")
                return path.absolutePath
            } else if (path.exists() && !path.canExecute()) {
                // Try to make it executable
                path.setExecutable(true, false)
                if (path.canExecute()) {
                    LOG.info("Found and made executable bundled server at: ${path.absolutePath}")
                    return path.absolutePath
                }
            }
            LOG.debug("Bundled server not found at: ${path.absolutePath}")
        }

        LOG.warn("Bundled Riwaq binary not found. Tried: ${possiblePaths.map { it.absolutePath }}")
        return null
    }

    override fun dispose() {
        scope.cancel()
        runBlocking {
            stopServer()
        }
    }
}
