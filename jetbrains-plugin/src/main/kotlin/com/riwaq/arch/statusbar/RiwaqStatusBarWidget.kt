package com.riwaq.arch.statusbar

import com.intellij.openapi.components.service
import com.intellij.openapi.project.Project
import com.intellij.openapi.wm.StatusBar
import com.intellij.openapi.wm.StatusBarWidget
import com.intellij.openapi.wm.StatusBarWidgetFactory
import com.riwaq.arch.server.ServerManager
import java.awt.Component

/**
 * Status bar widget showing Riwaq server status.
 */
class RiwaqStatusBarWidgetFactory : StatusBarWidgetFactory {

    override fun getId(): String = "RiwaqStatusBar"

    override fun getDisplayName(): String = "Riwaq Server Status"

    override fun isAvailable(project: Project): Boolean = true

    override fun createWidget(project: Project): StatusBarWidget {
        return RiwaqStatusBarWidget(project)
    }

    override fun disposeWidget(widget: StatusBarWidget) {}

    override fun canBeEnabledOn(statusBar: StatusBar): Boolean = true
}

class RiwaqStatusBarWidget(private val project: Project) : StatusBarWidget {

    private val serverManager = ServerManager.getInstance()

    override fun ID(): String = "RiwaqStatusBar"

    override fun install(statusBar: StatusBar) {
        statusBar.updateWidget(ID())
    }

    override fun dispose() {}

    fun update() {
        // Widget updates automatically through the platform
    }
}
