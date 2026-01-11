package com.riwaq.arch.toolwindow

import com.intellij.openapi.Disposable
import com.intellij.openapi.components.service
import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.project.Project
import com.intellij.ui.components.JBList
import com.intellij.ui.components.JBScrollPane
import com.intellij.ui.components.JBTabbedPane
import com.riwaq.arch.client.RiwaqClient
import com.riwaq.arch.client.models.ModuleInfo
import com.riwaq.arch.server.ServerManager
import kotlinx.coroutines.*
import kotlinx.coroutines.swing.Swing
import java.awt.BorderLayout
import javax.swing.*

/**
 * Main panel for the Architecture Tool Window.
 */
class ArchitecturePanel(private val project: Project) : JPanel(), Disposable {

    companion object {
        private val LOG = Logger.getInstance(ArchitecturePanel::class.java)
    }

    private val scope = CoroutineScope(Dispatchers.Default + SupervisorJob())
    private val client = RiwaqClient.getInstance(project)
    private val serverManager = ServerManager.getInstance()

    private val tabbedPane = JBTabbedPane()
    private val modulesPanel = ModulesPanel()
    private val askPanel = AskPanel(project)
    private val diagramPanel = DiagramPanel(project)
    private val insightsPanel = InsightsPanel()

    init {
        layout = BorderLayout()
        add(tabbedPane, BorderLayout.CENTER)

        tabbedPane.addTab("Modules", modulesPanel)
        tabbedPane.addTab("Ask AI", askPanel)
        tabbedPane.addTab("Diagram", diagramPanel)
        tabbedPane.addTab("Insights", insightsPanel)

        // Load data on initialization
        loadData()
    }

    private fun loadData() {
        scope.launch {
            try {
                // Ensure server is running
                if (!serverManager.isServerRunning()) {
                    withContext(Dispatchers.Swing) {
                        modulesPanel.setStatus("Starting server...")
                    }
                    serverManager.startServer()
                }

                // Load analysis data
                loadModules()
                loadInsights()
            } catch (e: Exception) {
                LOG.error("Failed to load data", e)
                withContext(Dispatchers.Swing) {
                    modulesPanel.setStatus("Error: ${e.message}")
                }
            }
        }
    }

    private suspend fun loadModules() {
        withContext(Dispatchers.Swing) {
            modulesPanel.setStatus("Loading modules...")
        }

        val result = client.getAnalysis()
        result.onSuccess { analysis ->
            withContext(Dispatchers.Swing) {
                modulesPanel.setModules(analysis.modules)
                modulesPanel.setStatus("")
            }
        }.onFailure { error ->
            withContext(Dispatchers.Swing) {
                modulesPanel.setStatus("Failed to load: ${error.message}")
            }
        }
    }

    private suspend fun loadInsights() {
        withContext(Dispatchers.Swing) {
            insightsPanel.setStatus("Loading insights...")
        }

        val result = client.getAnalysis()
        result.onSuccess { analysis ->
            withContext(Dispatchers.Swing) {
                insightsPanel.setStats(analysis.stats)
                insightsPanel.setStatus("")
            }
        }.onFailure { error ->
            withContext(Dispatchers.Swing) {
                insightsPanel.setStatus("Failed to load: ${error.message}")
            }
        }
    }

    fun refreshData() {
        loadData()
    }

    override fun dispose() {
        scope.cancel()
    }
}
