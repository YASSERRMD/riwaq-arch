package com.riwaq.arch.listeners

import com.intellij.openapi.components.service
import com.intellij.openapi.project.Project
import com.intellij.openapi.project.ProjectManagerListener
import com.riwaq.arch.RiwaqPlugin

/**
 * Listener for project lifecycle events.
 */
class ProjectManagerListener : ProjectManagerListener {

    override fun projectOpened(project: Project) {
        // Initialize plugin when project is opened
        val plugin = com.riwaq.arch.RiwaqPlugin.getInstance()
        plugin.initialize()
    }

    override fun projectClosing(project: Project) {
        // Cleanup when project is closing
    }
}
