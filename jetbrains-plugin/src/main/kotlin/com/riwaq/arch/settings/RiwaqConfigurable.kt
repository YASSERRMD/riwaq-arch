package com.riwaq.arch.settings

import com.intellij.openapi.components.service
import com.intellij.openapi.options.Configurable
import com.intellij.openapi.ui.TextFieldWithBrowseButton
import com.intellij.openapi.ui.VerticalFlowLayout
import com.intellij.ui.components.JBCheckBox
import com.intellij.ui.components.JBLabel
import com.intellij.ui.components.JBPasswordField
import com.intellij.ui.components.JBTextArea
import com.intellij.ui.components.JBTextField
import com.intellij.util.ui.FormBuilder
import com.intellij.util.ui.UIUtil
import java.awt.BorderLayout
import javax.swing.JComponent
import javax.swing.JPanel

/**
 * Application-level settings UI.
 */
class RiwaqConfigurable : Configurable {

    private val settings = service<RiwaqSettings>()

    private lateinit var serverUrlField: JBTextField
    private lateinit var serverPathField: TextFieldWithBrowseButton
    private lateinit var autoStartServerCheckBox: JBCheckBox
    private lateinit var llmApiKeyField: JBPasswordField
    private lateinit var llmEndpointField: JBTextField
    private lateinit var llmModelField: JBTextField
    private lateinit var excludedDirsField: JBTextArea

    private val mainPanel: JPanel by lazy {
        createPanel()
    }

    private fun createPanel(): JPanel {
        serverUrlField = JBTextField(settings.serverUrl)
        serverPathField = TextFieldWithBrowseButton().apply {
            text = settings.serverPath
            addBrowseFolderListener(
                "Select Riwaq Server Binary",
                "Choose the riwaq executable",
                null,
                com.intellij.openapi.fileChooser.FileChooserDescriptorFactory.createSingleFileDescriptor()
            )
        }
        autoStartServerCheckBox = JBCheckBox("Automatically start server when IDE opens", settings.autoStartServer)
        llmApiKeyField = JBPasswordField()
        llmApiKeyField.text = settings.llmApiKey
        llmEndpointField = JBTextField(settings.llmEndpoint)
        llmModelField = JBTextField(settings.llmModel)
        excludedDirsField = JBTextArea(settings.excludedDirs)
        excludedDirsField.rows = 3

        return FormBuilder.createFormBuilder()
            .addLabeledComponent("Server URL:", serverUrlField)
            .addTooltip("The URL where the Riwaq server is running")
            .addComponent(serverUrlField, 1)
            .addVerticalGap(10)
            .addLabeledComponent("Server Binary Path:", serverPathField)
            .addTooltip("Optional: Path to the riwaq server binary. Leave empty to use bundled or system PATH.")
            .addComponent(serverPathField, 1)
            .addVerticalGap(10)
            .addComponent(autoStartServerCheckBox)
            .addVerticalGap(10)
            .addSeparator(10)
            .addComponent(JBLabel("LLM Configuration"))
            .addVerticalGap(5)
            .addLabeledComponent("API Key:", llmApiKeyField)
            .addComponent(llmApiKeyField, 1)
            .addVerticalGap(10)
            .addLabeledComponent("API Endpoint:", llmEndpointField)
            .addComponent(llmEndpointField, 1)
            .addVerticalGap(10)
            .addLabeledComponent("Model:", llmModelField)
            .addComponent(llmModelField, 1)
            .addVerticalGap(10)
            .addSeparator(10)
            .addComponent(JBLabel("Excluded Directories"))
            .addTooltip("Comma-separated list of directories to exclude from analysis")
            .addVerticalGap(5)
            .addComponent(excludedDirsField, 1)
            .addComponentFillVertically(JPanel(), 0)
            .panel
    }

    override fun getDisplayName(): String = "Riwaq Arch"

    override fun createComponent(): JComponent = mainPanel

    override fun isModified(): Boolean {
        return serverUrlField.text != settings.serverUrl ||
                serverPathField.text != settings.serverPath ||
                autoStartServerCheckBox.isSelected != settings.autoStartServer ||
                llmApiKeyField.text != settings.llmApiKey ||
                llmEndpointField.text != settings.llmEndpoint ||
                llmModelField.text != settings.llmModel ||
                excludedDirsField.text != settings.excludedDirs
    }

    override fun apply() {
        settings.serverUrl = serverUrlField.text
        settings.serverPath = serverPathField.text
        settings.autoStartServer = autoStartServerCheckBox.isSelected
        settings.llmApiKey = llmApiKeyField.text
        settings.llmEndpoint = llmEndpointField.text
        settings.llmModel = llmModelField.text
        settings.excludedDirs = excludedDirsField.text
    }

    override fun reset() {
        serverUrlField.text = settings.serverUrl
        serverPathField.text = settings.serverPath
        autoStartServerCheckBox.isSelected = settings.autoStartServer
        llmApiKeyField.text = settings.llmApiKey
        llmEndpointField.text = settings.llmEndpoint
        llmModelField.text = settings.llmModel
        excludedDirsField.text = settings.excludedDirs
    }
}
