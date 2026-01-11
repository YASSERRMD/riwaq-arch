package com.riwaq.arch.actions

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.components.service
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.riwaq.arch.server.ServerManager
import kotlinx.coroutines.runBlocking

/**
 * Action to start the Riwaq server.
 */
class StartServerAction : AnAction() {

    override fun actionPerformed(e: AnActionEvent) {
        val serverManager = ServerManager.getInstance()

        ProgressManager.getInstance().run(object : Task.Backgroundable(e.project, "Starting Riwaq Server", true) {
            override fun run(indicator: ProgressIndicator) {
                indicator.text = "Starting Riwaq server..."
                runBlocking {
                    val success = serverManager.startServer()
                    if (!success) {
                        indicator.text = "Failed to start server"
                    }
                }
            }
        })
    }

    override fun update(e: AnActionEvent) {
        val serverManager = ServerManager.getInstance()
        e.presentation.isEnabled = !serverManager.isServerRunning()
    }
}
