package com.riwaq.arch.toolwindow

import com.intellij.openapi.diagnostic.Logger
import com.intellij.openapi.project.Project
import com.intellij.ui.components.JBLabel
import com.intellij.ui.components.JBScrollPane
import com.intellij.ui.components.JBTextArea
import com.intellij.ui.components.JBTextField
import com.riwaq.arch.client.RiwaqClient
import com.riwaq.arch.client.models.StreamResponse
import kotlinx.coroutines.*
import kotlinx.coroutines.swing.Swing
import org.commonmark.parser.Parser
import org.commonmark.renderer.html.HtmlRenderer
import java.awt.BorderLayout
import java.awt.FlowLayout
import java.awt.event.ActionEvent
import java.awt.event.KeyEvent
import javax.swing.*

/**
 * Panel for AI Q&A about the codebase.
 */
class AskPanel(private val project: Project) : JPanel() {

    companion object {
        private val LOG = Logger.getInstance(AskPanel::class.java)
        private val EXAMPLE_QUESTIONS = listOf(
            "What is the overall architecture of this codebase?",
            "What are the main services and how do they interact?",
            "Show me the authentication flow",
            "What are the key utilities and helpers?",
            "How is data validation handled?"
        )
    }

    private val scope = CoroutineScope(Dispatchers.Default + SupervisorJob())
    private val client = RiwaqClient.getInstance(project)

    private val questionField = JBTextField()
    private val askButton = JButton("Ask")
    private val responseArea = JEditorPane()
    private val responseScrollPane = JBScrollPane(responseArea)
    private val examplesPanel = JPanel(FlowLayout(FlowLayout.LEFT))

    private var isProcessing = false

    private val markdownParser = Parser.builder().build()
    private val htmlRenderer = HtmlRenderer.builder().build()

    init {
        layout = BorderLayout(5, 5)

        // Question input panel
        val inputPanel = JPanel(BorderLayout(5, 0))
        inputPanel.add(JBLabel("Question:"), BorderLayout.WEST)
        inputPanel.add(questionField, BorderLayout.CENTER)
        inputPanel.add(askButton, BorderLayout.EAST)

        // Configure response area
        responseArea.contentType = "text/html"
        responseArea.isEditable = false
        responseArea.addHyperlinkListener { e ->
            if (e.eventType == HyperlinkEvent.EventType.ACTIVATED) {
                // Handle file path clicks
                val url = e.url?.toString()
                if (url?.startsWith("file://") == true) {
                    openFileInEditor(url.removePrefix("file://"))
                }
            }
        }

        // Example questions
        examplesPanel.border = BorderFactory.createTitledBorder("Example Questions")
        EXAMPLE_QUESTIONS.forEach { question ->
            val button = JButton(question)
            button.addActionListener {
                questionField.text = question
                askQuestion()
            }
            examplesPanel.add(button)
        }

        // Top panel with examples
        val topPanel = JPanel(BorderLayout())
        topPanel.add(inputPanel, BorderLayout.NORTH)
        topPanel.add(examplesPanel, BorderLayout.CENTER)

        add(topPanel, BorderLayout.NORTH)
        add(responseScrollPane, BorderLayout.CENTER)

        // Setup actions
        askButton.addActionListener { askQuestion() }
        questionField.addActionListener { askQuestion() }

        // Show welcome message
        showWelcomeMessage()
    }

    private fun showWelcomeMessage() {
        val html = """
            <html>
            <body style='font-family: sans-serif; padding: 10px;'>
                <h2>Welcome to Riwaq Arch</h2>
                <p>Ask questions about your codebase and get AI-powered answers with file references.</p>
                <p>Click an example question above or type your own question to get started.</p>
                <ul>
                    <li>💡 <strong>Architecture:</strong> Understand the overall structure</li>
                    <li>🔍 <strong>Modules:</strong> Learn about specific components</li>
                    <li>🔗 <strong>Dependencies:</strong> See how modules connect</li>
                    <li>📝 <strong>Patterns:</strong> Discover design patterns used</li>
                </ul>
            </body>
            </html>
        """.trimIndent()
        responseArea.text = html
    }

    private fun askQuestion() {
        if (isProcessing) return

        val question = questionField.text.trim()
        if (question.isEmpty()) return

        isProcessing = true
        askButton.isEnabled = false
        askButton.text = "Asking..."

        // Clear response area and show loading
        responseArea.text = "<html><body><i>Thinking...</i></body></html>"

        scope.launch {
            try {
                val responseBuilder = StringBuilder()
                val files = mutableListOf<String>()

                client.askQuestion(question) { streamResponse ->
                    when (streamResponse.type) {
                        "content" -> {
                            streamResponse.content?.let { responseBuilder.append(it) }
                            updateResponse(responseBuilder.toString(), files)
                        }
                        "file" -> {
                            streamResponse.file?.let { file ->
                                val fileRef = "[${file.path}${file.line?.let { ":$it" } ?: ""}]"
                                if (!files.contains(fileRef)) {
                                    files.add(fileRef)
                                    updateResponse(responseBuilder.toString(), files)
                                }
                            }
                        }
                        "error" -> {
                            streamResponse.error?.let { error ->
                                withContext(Dispatchers.Swing) {
                                    showError(error)
                                }
                            }
                        }
                        "done" -> {
                            withContext(Dispatchers.Swing) {
                                isProcessing = false
                                askButton.isEnabled = true
                                askButton.text = "Ask"
                            }
                        }
                    }
                }
            } catch (e: Exception) {
                LOG.error("Failed to ask question", e)
                withContext(Dispatchers.Swing) {
                    showError("Error: ${e.message}")
                    isProcessing = false
                    askButton.isEnabled = true
                    askButton.text = "Ask"
                }
            }
        }
    }

    private suspend fun updateResponse(content: String, files: List<String>) {
        withContext(Dispatchers.Swing) {
            val markdown = "# Answer\n\n$content\n\n"
            val htmlContent = markdownParser.parse(markdown).let { htmlRenderer.render(it) }

            val filesHtml = if (files.isNotEmpty()) {
                "<hr/><h3>Referenced Files</h3><ul>" +
                        files.joinToString("") { "<li><code>$it</code></li>" } +
                        "</ul>"
            } else {
                ""
            }

            val fullHtml = """
                <html>
                <head>
                    <style>
                        body { font-family: sans-serif; padding: 10px; }
                        code { background-color: #f4f4f4; padding: 2px 6px; border-radius: 3px; }
                        pre { background-color: #f4f4f4; padding: 10px; border-radius: 5px; overflow-x: auto; }
                        h1, h2, h3 { margin-top: 15px; }
                    </style>
                </head>
                <body>
                    $htmlContent
                    $filesHtml
                </body>
                </html>
            """.trimIndent()

            responseArea.text = fullHtml
            responseArea.caretPosition = 0
        }
    }

    private fun showError(error: String) {
        responseArea.text = """
            <html>
            <body style='font-family: sans-serif; padding: 10px; color: red;'>
                <h3>Error</h3>
                <p>$error</p>
            </body>
            </html>
        """.trimIndent()
    }

    private fun openFileInEditor(filePath: String) {
        // TODO: Implement opening file in editor
        LOG.info("Would open file: $filePath")
    }

    fun cleanup() {
        scope.cancel()
    }
}
