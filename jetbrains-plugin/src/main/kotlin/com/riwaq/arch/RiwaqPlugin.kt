package com.riwaq.arch

import com.intellij.openapi.components.Service
import com.intellij.openapi.components.service
import com.intellij.openapi.diagnostic.Logger
import com.riwaq.arch.settings.RiwaqSettings
import com.riwaq.arch.server.ServerManager

/**
 * Main plugin class that manages the plugin lifecycle.
 */
class RiwaqPlugin {
    companion object {
        private val LOG = Logger.getInstance(RiwaqPlugin::class.java)

        fun getInstance(): RiwaqPlugin = service()
    }

    private var initialized = false

    /**
     * Initialize the plugin.
     * This should be called when the plugin is activated.
     */
    fun initialize() {
        if (initialized) return
        initialized = true

        LOG.info("Initializing Riwaq Arch plugin")

        val settings = service<RiwaqSettings>()
        val serverManager = service<ServerManager>()

        // Auto-start server if configured
        if (settings.autoStartServer) {
            LOG.info("Auto-starting Riwaq server")
            serverManager.startServer()
        }
    }

    /**
     * Cleanup when the plugin is deactivated.
     */
    fun dispose() {
        LOG.info("Disposing Riwaq Arch plugin")
        initialized = false
    }
}
