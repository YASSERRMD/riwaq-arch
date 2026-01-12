package com.riwaq.arch

import com.intellij.openapi.components.Service
import com.intellij.openapi.components.service
import com.intellij.openapi.diagnostic.Logger
import com.riwaq.arch.settings.RiwaqSettings
import com.riwaq.arch.server.ServerManager
import kotlinx.coroutines.*

/**
 * Main plugin class that manages the plugin lifecycle.
 */
@Service(Service.Level.APP)
class RiwaqPlugin {
    companion object {
        private val LOG = Logger.getInstance(RiwaqPlugin::class.java)

        fun getInstance(): RiwaqPlugin = service()
    }

    private val scope = CoroutineScope(Dispatchers.IO + SupervisorJob())
    private var initialized = false

    /**
     * Initialize the plugin.
     * This should be called when the plugin is activated.
     */
    fun initialize() {
        if (initialized) return
        initialized = true

        LOG.info("Initializing Riwaq Arch plugin")

        try {
            val settings = RiwaqSettings.getInstance()
            val serverManager = ServerManager.getInstance()

            // Auto-start server if configured
            if (settings.autoStartServer) {
                LOG.info("Auto-starting Riwaq server")
                scope.launch {
                    try {
                        serverManager.startServer()
                    } catch (e: Exception) {
                        LOG.error("Failed to auto-start server", e)
                    }
                }
            }
        } catch (e: Exception) {
            LOG.error("Failed to initialize plugin", e)
        }
    }

    /**
     * Cleanup when the plugin is deactivated.
     */
    fun dispose() {
        LOG.info("Disposing Riwaq Arch plugin")
        initialized = false
        scope.cancel()
    }
}
