package com.riwaq.arch.settings

import com.intellij.openapi.components.PersistentStateComponent
import com.intellij.openapi.components.Service
import com.intellij.openapi.components.State
import com.intellij.openapi.components.Storage
import com.intellij.openapi.project.Project
import com.intellij.util.xmlb.XmlSerializerUtil

/**
 * Project-level settings for Riwaq plugin.
 */
@Service(Service.Level.PROJECT)
@State(name = "RiwaqProjectSettings", storages = [Storage("RiwaqProjectSettings.xml")])
class RiwaqProjectSettings(val project: Project) : PersistentStateComponent<RiwaqProjectSettings.State> {

    data class State(
        var autoAnalyze: Boolean = false,
        var lastAnalyzedHash: String = "",
        var excludedFiles: String = "",
        var customPrompt: String = ""
    )

    private var state = State()

    override fun getState(): State = state

    override fun loadState(state: State) {
        this.state = state
    }

    var autoAnalyze: Boolean
        get() = state.autoAnalyze
        set(value) { state.autoAnalyze = value }

    var lastAnalyzedHash: String
        get() = state.lastAnalyzedHash
        set(value) { state.lastAnalyzedHash = value }

    var excludedFiles: String
        get() = state.excludedFiles
        set(value) { state.excludedFiles = value }

    var customPrompt: String
        get() = state.customPrompt
        set(value) { state.customPrompt = value }

    companion object {
        fun getInstance(project: Project): RiwaqProjectSettings = project.service()
    }
}
