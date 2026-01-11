package com.riwaq.arch.toolwindow

import com.intellij.openapi.project.Project
import com.intellij.openapi.wm.ToolWindow
import com.intellij.openapi.wm.ToolWindowFactory
import com.intellij.ui.content.ContentFactory

/**
 * Factory for creating the Architecture Tool Window.
 */
class ArchitectureToolWindowFactory : ToolWindowFactory {

    override fun createToolWindowContent(project: Project, toolWindow: ToolWindow) {
        val architecturePanel = ArchitecturePanel(project)
        val contentFactory = ContentFactory.getInstance()
        val content = contentFactory.createContent(architecturePanel, "", false)
        toolWindow.contentManager.addContent(content)
    }
}
