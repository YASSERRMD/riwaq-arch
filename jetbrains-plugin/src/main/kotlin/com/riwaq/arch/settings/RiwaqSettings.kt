package com.riwaq.arch.settings

import com.intellij.openapi.components.PersistentStateComponent
import com.intellij.openapi.components.Service
import com.intellij.openapi.components.State
import com.intellij.openapi.components.Storage
import com.intellij.util.xmlb.XmlSerializerUtil

/**
 * Application-level settings for Riwaq plugin.
 */
@Service(Service.Level.APP)
@State(name = "RiwaqSettings", storages = [Storage("RiwaqSettings.xml")])
class RiwaqSettings : PersistentStateComponent<RiwaqSettings.State> {

    data class State(
        var serverUrl: String = "http://127.0.0.1:9527",
        var serverPath: String = "",
        var autoStartServer: Boolean = true,
        var llmApiKey: String = "",
        var llmEndpoint: String = "https://openrouter.ai/api/v1/chat/completions",
        var llmModel: String = "glm-4",
        var excludedDirs: String = "node_modules,dist,build,target,.git"
    )

    private var state = State()

    override fun getState(): State = state

    override fun loadState(state: State) {
        this.state = state
    }

    var serverUrl: String
        get() = state.serverUrl
        set(value) { state.serverUrl = value }

    var serverPath: String
        get() = state.serverPath
        set(value) { state.serverPath = value }

    var autoStartServer: Boolean
        get() = state.autoStartServer
        set(value) { state.autoStartServer = value }

    var llmApiKey: String
        get() = state.llmApiKey
        set(value) { state.llmApiKey = value }

    var llmEndpoint: String
        get() = state.llmEndpoint
        set(value) { state.llmEndpoint = value }

    var llmModel: String
        get() = state.llmModel
        set(value) { state.llmModel = value }

    var excludedDirs: String
        get() = state.excludedDirs
        set(value) { state.excludedDirs = value }

    fun getExcludedDirsList(): List<String> {
        return excludedDirs.split(",").map { it.trim() }.filter { it.isNotEmpty() }
    }

    companion object {
        fun getInstance(): RiwaqSettings = service()
    }
}
