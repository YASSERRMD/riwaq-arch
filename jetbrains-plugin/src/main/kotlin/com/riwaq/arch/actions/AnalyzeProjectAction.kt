package com.riwaq.arch.actions

import com.intellij.notification.Notification
import com.intellij.notification.NotificationType
import com.intellij.notification.Notifications
import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.components.service
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.riwaq.arch.client.RiwaqClient
import com.riwaq.arch.server.ServerManager
import kotlinx.coroutines.runBlocking

/**
 * Action to analyze the current project.
 */
class AnalyzeProjectAction : AnAction() {

    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        val serverManager = ServerManager.getInstance()
        val client = RiwaqClient.getInstance(project)

        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Analyzing Project", true) {
            override fun run(indicator: ProgressIndicator) {
                indicator.isIndeterminate = true
                indicator.text = "Ensuring server is running..."

                runBlocking {
                    // Start server if needed
                    if (!serverManager.isServerRunning()) {
                        indicator.text = "Starting server..."
                        serverManager.startServer()
                    }

                    // Analyze project
                    indicator.text = "Analyzing project..."
                    val result = client.analyzeProject()

                    result.onSuccess {
                        indicator.text = "Analysis complete"
                        Notification(
                            "Riwaq Arch",
                            "Analysis Complete",
                            "Your project has been analyzed successfully",
                            NotificationType.INFORMATION
                        ).notify(project)
                    }.onFailure { error ->
                        indicator.text = "Analysis failed"
                        Notification(
                            "Riwaq Arch",
                            "Analysis Failed",
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
