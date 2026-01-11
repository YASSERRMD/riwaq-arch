package com.riwaq.arch.toolwindow

import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.project.Project
import com.intellij.ui.JBColor
import com.intellij.ui.components.JBLabel
import com.intellij.ui.components.JBScrollPane
import com.riwaq.arch.client.RiwaqClient
import kotlinx.coroutines.*
import kotlinx.coroutines.swing.Swing
import java.awt.BorderLayout
import java.awt.FlowLayout
import javax.swing.*

/**
 * Panel displaying the architecture diagram.
 */
class DiagramPanel(private val project: Project) : JPanel() {

    companion object {
        private val LOG = Logger.getInstance(DiagramPanel::class.java)
    }

    private val scope = CoroutineScope(Dispatchers.Default + SupervisorJob())

    private val diagramLabel = JBLabel()
    private val scrollPane = JBScrollPane(diagramLabel)
    private val toolbar = JPanel(FlowLayout(FlowLayout.LEFT))

    private val zoomInButton = JButton("+")
    private val zoomOutButton = JButton("-")
    private val resetButton = JButton("Reset")
    private val refreshButton = JButton("Refresh")

    private var currentZoom = 1.0f
    private var currentDiagram: String? = null

    init {
        layout = BorderLayout(5, 5)

        // Setup toolbar
        toolbar.add(zoomInButton)
        toolbar.add(zoomOutButton)
        toolbar.add(resetButton)
        toolbar.add(refreshButton)

        add(toolbar, BorderLayout.NORTH)
        add(scrollPane, BorderLayout.CENTER)

        // Setup button actions
        zoomInButton.addActionListener { zoomIn() }
        zoomOutButton.addActionListener { zoomOut() }
        resetButton.addActionListener { resetZoom() }
        refreshButton.addActionListener { loadDiagram() }

        diagramLabel.horizontalAlignment = SwingConstants.CENTER
        diagramLabel.verticalAlignment = SwingConstants.CENTER
    }

    private fun loadDiagram() {
        scope.launch {
            try {
                val client = com.riwaq.arch.client.RiwaqClient.getInstance(project)
                val result = client.getArchitectureDiagram()

                result.onSuccess { diagramData ->
                    currentDiagram = diagramData.mermaid
                    withContext(Dispatchers.Swing) {
                        displayDiagram(diagramData.mermaid)
                    }
                }.onFailure { error ->
                    withContext(Dispatchers.Swing) {
                        showError("Failed to load diagram: ${error.message}")
                    }
                }
            } catch (e: Exception) {
                LOG.error("Failed to load diagram", e)
                withContext(Dispatchers.Swing) {
                    showError("Error: ${e.message}")
                }
            }
        }
    }

    private fun displayDiagram(mermaid: String) {
        // For now, display as text. In a full implementation, you would:
        // 1. Use a Mermaid library to render the diagram
        // 2. Or use a webview to render it with mermaid.js
        val html = """
            <html>
            <body style='font-family: monospace; padding: 10px; background-color: #f5f5f5;'>
                <h3>Architecture Diagram (Mermaid)</h3>
                <pre style='white-space: pre-wrap; word-wrap: break-word;'>$mermaid</pre>
                <p><i>Copy this code to a Mermaid editor to view the diagram</i></p>
            </body>
            </html>
        """.trimIndent()

        diagramLabel.text = html
    }

    private fun showError(message: String) {
        diagramLabel.text = """
            <html>
            <body style='font-family: sans-serif; padding: 10px; color: red;'>
                <h3>Error</h3>
                <p>$message</p>
            </body>
            </html>
        """.trimIndent()
    }

    private fun zoomIn() {
        currentZoom = (currentZoom * 1.2f).coerceAtMost(3.0f)
        applyZoom()
    }

    private fun zoomOut() {
        currentZoom = (currentZoom / 1.2f).coerceAtLeast(0.3f)
        applyZoom()
    }

    private fun resetZoom() {
        currentZoom = 1.0f
        applyZoom()
    }

    private fun applyZoom() {
        // Apply zoom to the diagram label
        // This is a simplified version - a full implementation would properly scale the diagram
        diagramLabel.text = diagramLabel.text
    }

    fun cleanup() {
        scope.cancel()
    }
}
