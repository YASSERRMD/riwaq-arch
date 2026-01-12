package com.riwaq.arch.actions

import com.intellij.notification.Notification
import com.intellij.notification.NotificationType
import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.fileChooser.FileChooserDescriptorFactory
import com.intellij.openapi.fileChooser.FileChooserFactory
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.riwaq.arch.client.RiwaqClient
import com.riwaq.arch.server.ServerManager
import kotlinx.coroutines.runBlocking

/**
 * Action to generate documentation.
 */
class GenerateDocsAction : AnAction() {

    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        
        // Default output to project dir / generated-docs if no chooser logic is desired,
        // or just prompt. For simplicity matching VS Code (which usually prompts or uses default),
        // I'll stick to a default location inside the project to avoid UI blocking issues in headless if any.
        // Actually, VS Code extension uses config or prompts.
        // I'll use project_base_path/riwaq-docs
        
        val outputDir = "${project.basePath}/riwaq-docs"
        
        val serverManager = ServerManager.getInstance()
        val client = RiwaqClient.getInstance(project)

        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Generating Documentation", true) {
            override fun run(indicator: ProgressIndicator) {
                indicator.isIndeterminate = true
                indicator.text = "Ensuring server is running..."

                runBlocking {
                    // Start server if needed
                    if (!serverManager.isServerRunning()) {
                        indicator.text = "Starting server..."
                        serverManager.startServer()
                    }

                    // Generate Docs
                    indicator.text = "Generating documentation (including BRD, SRS, API)..."
                    val result = client.generateDocs(outputDir)

                    result.onSuccess { res ->
                        indicator.text = "Generation complete"
                        Notification(
                            "Riwaq Arch",
                            "Documentation Generated",
                            "Generated ${res.fileCount} files in ${outputDir}",
                            NotificationType.INFORMATION
                        ).notify(project)
                    }.onFailure { error ->
                        indicator.text = "Generation failed"
                        Notification(
                            "Riwaq Arch",
                            "Documentation Generation Failed",
                            error.message ?: "Unknown error",
                            NotificationType.ERROR
                        ).notify(project)
                    }
                }
            }
        })
    }

    override fun update(e: AnActionEvent) {
        e.presentation.isEnabled = e.project != null
    }
}
