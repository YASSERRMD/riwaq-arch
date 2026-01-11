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
 * Action to restart the Riwaq server.
 */
class RestartServerAction : AnAction() {

    override fun actionPerformed(e: AnActionEvent) {
        val serverManager = ServerManager.getInstance()

        ProgressManager.getInstance().run(object : Task.Backgroundable(e.project, "Restarting Riwaq Server", true) {
            override fun run(indicator: ProgressIndicator) {
                indicator.text = "Restarting Riwaq server..."
                runBlocking {
                    val success = serverManager.restartServer()
                    if (!success) {
                        indicator.text = "Failed to restart server"
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
