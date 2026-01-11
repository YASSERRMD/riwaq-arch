package com.riwaq.arch.toolwindow

import com.intellij.openapi.project.Project
import com.intellij.ui.JBColor
import com.intellij.ui.components.JBLabel
import com.intellij.ui.components.JBList
import com.intellij.ui.components.JBScrollPane
import com.riwaq.arch.client.models.ModuleInfo
import java.awt.BorderLayout
import java.awt.Component
import javax.swing.*

/**
 * Panel displaying the module hierarchy.
 */
class ModulesPanel : JPanel() {

    private val listModel = DefaultListModel<ModuleListItem>()
    private val list = JBList(listModel)
    private val scrollPane = JBScrollPane(list)
    private val statusLabel = JBLabel()

    init {
        layout = BorderLayout(0, 5)

        // Add status label at top
        val topPanel = JPanel(BorderLayout())
        topPanel.add(statusLabel, BorderLayout.NORTH)
        add(topPanel, BorderLayout.NORTH)

        // Add list in center
        add(scrollPane, BorderLayout.CENTER)

        // Set up list cell renderer
        list.cellRenderer = ModuleListCellRenderer()
        list.selectionMode = ListSelectionModel.SINGLE_SELECTION
    }

    fun setModules(modules: List<ModuleInfo>) {
        listModel.clear()
        modules.forEach { module ->
            listModel.addElement(ModuleListItem(module))
        }
    }

    fun setStatus(status: String) {
        statusLabel.text = status
    }

    private data class ModuleListItem(val module: ModuleInfo) {
        override fun toString(): String = module.name
    }

    private class ModuleListCellRenderer : ListCellRenderer<ModuleListItem> {
        override fun getListCellRendererComponent(
            list: JList<out ModuleListItem>,
            value: ModuleListItem,
            index: Int,
            isSelected: Boolean,
            cellHasFocus: Boolean
        ): Component {
            val module = value.module
            val panel = JPanel(BorderLayout(10, 2))
            panel.isOpaque = false

            if (isSelected) {
                panel.background = list.selectionBackground
                panel.foreground = list.selectionForeground
            } else {
                panel.background = list.background
                panel.foreground = list.foreground
            }

            // Health indicator
            val healthIcon = when (module.health) {
                "good" -> "🟢"
                "warning" -> "🟡"
                "poor" -> "🔴"
                else -> "⚪"
            }

            // Main label with module name and health
            val nameLabel = JLabel("$healthIcon ${module.name}")
            nameLabel.font = nameLabel.font.deriveFont(Font.BOLD, nameLabel.font.size + 1f)
            panel.add(nameLabel, BorderLayout.NORTH)

            // Details label with metrics
            val details = "${module.type} • LoC: ${module.metrics.linesOfCode} • Functions: ${module.metrics.functionCount} • Coupling: ${module.metrics.coupling}"
            val detailsLabel = JLabel(details)
            detailsLabel.font = detailsLabel.font.deriveFont(Font.PLAIN, detailsLabel.font.size - 1f)
            detailsLabel.foreground = JBColor.GRAY
            panel.add(detailsLabel, BorderLayout.CENTER)

            // Dependencies count
            if (module.dependencies.isNotEmpty()) {
                val depsLabel = JLabel("Deps: ${module.dependencies.size}")
                depsLabel.font = depsLabel.font.deriveFont(Font.PLAIN, depsLabel.font.size - 1f)
                panel.add(depsLabel, BorderLayout.SOUTH)
            }

            return panel
        }
    }
}
