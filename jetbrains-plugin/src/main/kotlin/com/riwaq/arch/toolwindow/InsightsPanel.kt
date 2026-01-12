package com.riwaq.arch.toolwindow

import com.intellij.ui.JBColor
import com.intellij.ui.components.JBLabel
import com.riwaq.arch.client.models.AnalysisStats
import java.awt.BorderLayout
import java.awt.Font
import java.awt.GridLayout
import javax.swing.*

/**
 * Panel displaying codebase insights and statistics.
 */
class InsightsPanel : JPanel() {

    private val statusLabel = JBLabel()
    private val statsPanel = JPanel()

    init {
        layout = BorderLayout(10, 10)

        // Add status label at top
        val topPanel = JPanel(BorderLayout())
        topPanel.add(statusLabel, BorderLayout.NORTH)
        add(topPanel, BorderLayout.NORTH)

        // Add stats panel in center
        statsPanel.layout = GridLayout(0, 2, 10, 10)
        add(JScrollPane(statsPanel), BorderLayout.CENTER)

        statusLabel.text = "No data available"
    }

    fun setStats(stats: AnalysisStats) {
        statsPanel.removeAll()

        // Create stat cards
        addStatCard("Total Modules", stats.totalModules.toString(), "📦")
        addStatCard("Total Services", stats.totalServices.toString(), "⚙️")
        addStatCard("Total Lines of Code", formatNumber(stats.totalLinesOfCode), "📝")
        addStatCard("Average Coupling", String.format("%.2f", stats.averageCoupling), "🔗")
        addStatCard("Average Cohesion", String.format("%.2f", stats.averageCohesion), "💎")

        // Languages
        if (stats.languages.isNotEmpty()) {
            val topLanguage = stats.languages.maxByOrNull { it.value }
            addStatCard("Primary Language", topLanguage?.key ?: "N/A", "🌐")
        }

        statsPanel.revalidate()
        statsPanel.repaint()
    }

    private fun addStatCard(title: String, value: String, icon: String) {
        val card = JPanel()
        card.layout = BorderLayout(5, 5)
        card.border = BorderFactory.createCompoundBorder(
            BorderFactory.createLineBorder(JBColor.GRAY),
            BorderFactory.createEmptyBorder(10, 10, 10, 10)
        )

        val titleLabel = JLabel("$icon $title")
        titleLabel.font = titleLabel.font.deriveFont(Font.PLAIN, 11f)

        val valueLabel = JLabel(value)
        valueLabel.font = valueLabel.font.deriveFont(Font.BOLD, 18f)

        card.add(titleLabel, BorderLayout.NORTH)
        card.add(valueLabel, BorderLayout.CENTER)

        statsPanel.add(card)
    }

    fun setStatus(status: String) {
        statusLabel.text = status
    }

    private fun formatNumber(num: Int): String {
        return when {
            num >= 1_000_000 -> String.format("%.1fM", num / 1_000_000.0)
            num >= 1_000 -> String.format("%.1fK", num / 1_000.0)
            else -> num.toString()
        }
    }
}
