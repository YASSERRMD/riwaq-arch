package com.riwaq.arch.settings

import com.intellij.openapi.options.Configurable
import com.intellij.openapi.project.Project
import com.intellij.ui.components.JBCheckBox
import com.intellij.ui.components.JBLabel
import com.intellij.ui.components.JBTextArea
import com.intellij.ui.components.JBTextField
import com.intellij.util.ui.FormBuilder
import javax.swing.JComponent
import javax.swing.JPanel

/**
 * Project-level settings UI.
 */
class RiwaqProjectConfigurable(private val project: Project) : Configurable {

    private val settings = RiwaqProjectSettings.getInstance(project)

    private lateinit var autoAnalyzeCheckBox: JBCheckBox
    private lateinit var excludedFilesField: JBTextArea
    private lateinit var customPromptField: JBTextArea

    private val mainPanel: JPanel by lazy {
        createPanel()
    }

    private fun createPanel(): JPanel {
        autoAnalyzeCheckBox = JBCheckBox("Automatically analyze project on changes", settings.autoAnalyze)
        excludedFilesField = JBTextArea(settings.excludedFiles)
        excludedFilesField.rows = 3
        customPromptField = JBTextArea(settings.customPrompt)
        customPromptField.rows = 5

        return FormBuilder.createFormBuilder()
            .addComponent(autoAnalyzeCheckBox)
            .addTooltip("Automatically re-analyze the project when files change")
            .addVerticalGap(10)
            .addSeparator(10)
            .addComponent(JBLabel("Excluded Files"))
            .addTooltip("Comma-separated list of file patterns to exclude from analysis (e.g., *.test.ts,*.spec.js)")
            .addVerticalGap(5)
            .addComponent(excludedFilesField, 1)
            .addVerticalGap(10)
            .addSeparator(10)
            .addComponent(JBLabel("Custom Analysis Prompt"))
            .addTooltip("Custom instructions for the AI when analyzing your codebase")
            .addVerticalGap(5)
            .addComponent(customPromptField, 1)
            .addComponentFillVertically(JPanel(), 0)
            .panel
    }

    override fun getDisplayName(): String = "Riwaq Arch"

    override fun createComponent(): JComponent = mainPanel

    override fun isModified(): Boolean {
        return autoAnalyzeCheckBox.isSelected != settings.autoAnalyze ||
                excludedFilesField.text != settings.excludedFiles ||
                customPromptField.text != settings.customPrompt
    }

    override fun apply() {
        settings.autoAnalyze = autoAnalyzeCheckBox.isSelected
        settings.excludedFiles = excludedFilesField.text
        settings.customPrompt = customPromptField.text
    }

    override fun reset() {
        autoAnalyzeCheckBox.isSelected = settings.autoAnalyze
        excludedFilesField.text = settings.excludedFiles
        customPromptField.text = settings.customPrompt
    }
}
