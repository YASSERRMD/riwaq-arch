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
 * Action to stop the Riwaq server.
 */
class StopServerAction : AnAction() {

    override fun actionPerformed(e: AnActionEvent) {
        val serverManager = ServerManager.getInstance()

        ProgressManager.getInstance().run(object : Task.Backgroundable(e.project, "Stopping Riwaq Server", true) {
            override fun run(indicator: ProgressIndicator) {
                indicator.text = "Stopping Riwaq server..."
                runBlocking {
                    val success = serverManager.stopServer()
                    if (!success) {
                        indicator.text = "Failed to stop server"
                    }
                }
            }
        })
    }

    override fun update(e: AnActionEvent) {
        val serverManager = ServerManager.getInstance()
        e.presentation.isEnabled = serverManager.isServerRunning()
    }
}
