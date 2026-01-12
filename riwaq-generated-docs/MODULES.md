# Modules & Services

Detailed documentation for each module in the codebase.

## jetbrains-plugin/main/kotlin/com/riwaq/arch/server/ServerManager

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/server/ServerManager`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/server/ServerManager.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 292 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## visual-studio-extension/GuidList

**Path**: `visual-studio-extension/GuidList`

### Files

- visual-studio-extension/GuidList.cs

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 13 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## vscode-extension/panels/architecturePanel

**Path**: `vscode-extension/panels/architecturePanel`

### Files

- vscode-extension/src/panels/architecturePanel.ts

### Dependencies

- * as vscode from 'vscode
- { RiwaqClient, CodebaseSnapshot } from '..

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 440 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/client/models/ApiModels

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/client/models/ApiModels`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/client/models/ApiModels.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 115 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## vscode-extension/panels/askPanel

**Path**: `vscode-extension/panels/askPanel`

### Files

- vscode-extension/src/panels/askPanel.ts

### Dependencies

- * as vscode from 'vscode
- { RiwaqClient } from '..

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 343 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## context

**Path**: `core::llm::context`

### Files

- core/src/llm/context.rs

### Public API

#### Types

- EnhancedContext
- RelevantFile
- ContextBuilder
- QuestionIntent

#### Functions

- new
- with_max_chars
- build_for_question

### Dependencies

- crate::models
- crate::analysis
- std::collections

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 537 |
| Functions | 12 |
| Types | 4 |
| Coupling Score | 0.30 |
| Cohesion Score | 0.70 |

---

## renderer

**Path**: `core::diagrams::renderer`

### Files

- core/src/diagrams/renderer.rs

### Public API

#### Types

- DiagramFormat
- RenderedDiagram
- DiagramRendererConfig
- DiagramRenderer

#### Functions

- extension
- new
- render
- render_to_svg
- render_architecture_diagram
- render_er_diagram
- render_state_diagram

### Dependencies

- mermaid_rs::Mermaid
- std::path
- tokio::fs
- tracing::info

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 165 |
| Functions | 8 |
| Types | 4 |
| Coupling Score | 0.27 |
| Cohesion Score | 0.73 |

---

## vscode-extension/serverManager

**Path**: `vscode-extension/serverManager`

### Files

- vscode-extension/src/serverManager.ts

### Dependencies

- * as vscode from 'vscode
- * as cp from 'child_process
- * as path from 'path
- * as fs from 'fs

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 175 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## errors

**Path**: `core::errors`

### Files

- core/src/errors.rs

### Public API

#### Types

- RiwaqError
- ResultExt

#### Functions

- file_system
- parse_error
- llm_error
- config
- internal
- is_retryable
- retry_after

### Dependencies

- std::path
- thiserror::Error

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 204 |
| Functions | 8 |
| Types | 2 |
| Coupling Score | 0.18 |
| Cohesion Score | 0.82 |

---

## mod

**Path**: `core::models::mod`

### Files

- core/src/models/mod.rs

### Dependencies

- file::FileSummary
- function::FunctionSummary
- git::GitInsights
- module::ModuleSummary
- snapshot::CodebaseSnapshot

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 17 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqConfigurable

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqConfigurable`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/settings/RiwaqConfigurable.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 121 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## fs_scanner

**Path**: `core::analysis::fs_scanner`

### Files

- core/src/analysis/fs_scanner.rs

### Public API

#### Types

- ScanConfig
- ScannedFile
- FsScanner
- ScanStats

#### Functions

- new
- exclude_dirs
- include_extensions
- max_file_size
- include_tests
- max_depth
- read_contents
- new
- scan
- get_stats

### Dependencies

- ignore::WalkBuilder
- std::collections
- std::path
- tracing::{debug, info, warn}
- crate::errors
- crate::models

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 397 |
| Functions | 12 |
| Types | 4 |
| Coupling Score | 0.30 |
| Cohesion Score | 0.70 |

---

## visual-studio-extension/RiwaqArchPackage

**Path**: `visual-studio-extension/RiwaqArchPackage`

### Files

- visual-studio-extension/RiwaqArchPackage.cs

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 50 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## entities

**Path**: `core::analysis::business::entities`

### Files

- core/src/analysis/business/entities.rs

### Public API

#### Types

- EntityAnalyzer

#### Functions

- new
- analyze

### Dependencies

- super::*
- regex::Regex
- std::collections
- std::path
- tokio::fs
- walkdir::WalkDir

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 537 |
| Functions | 18 |
| Types | 1 |
| Coupling Score | 0.67 |
| Cohesion Score | 0.33 |

---

## graphql

**Path**: `core::analysis::api::graphql`

### Files

- core/src/analysis/api/graphql.rs

### Public API

#### Types

- GraphQLAnalyzer

#### Functions

- new
- analyze

### Dependencies

- super::*
- regex::Regex
- std::path
- tokio::fs
- walkdir::WalkDir

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 292 |
| Functions | 10 |
| Types | 1 |
| Coupling Score | 0.62 |
| Cohesion Score | 0.38 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/OpenAskPanelAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/OpenAskPanelAction`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/actions/OpenAskPanelAction.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 24 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## vscode-extension/views/insightsTree

**Path**: `vscode-extension/views/insightsTree`

### Files

- vscode-extension/src/views/insightsTree.ts

### Public API

#### Types

- InsightItem

### Dependencies

- * as vscode from 'vscode
- { RiwaqClient } from '..

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 174 |
| Functions | 0 |
| Types | 1 |
| Coupling Score | 0.67 |
| Cohesion Score | 0.33 |

---

## handlers

**Path**: `core::server::handlers`

### Files

- core/src/server/handlers.rs

### Public API

#### Types

- AnalyzeRequest
- AnalyzeResponse
- DocgenRequest
- DocgenResponse
- AskRequest
- AskResponse
- FileRef
- HealthResponse
- DiagramResponse

#### Functions

- health
- list_projects
- analyze
- generate_docs
- ask_question
- get_architecture_diagram
- get_dependency_diagram

### Dependencies

- axum::{
    extract
- serde::{Deserialize, Serialize}
- std::path
- tracing::{error, info}
- super::state
- crate::analysis
- crate::docs
- crate::llm

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 399 |
| Functions | 8 |
| Types | 9 |
| Coupling Score | 0.33 |
| Cohesion Score | 0.67 |

---

## mod

**Path**: `core::analysis::api::mod`

### Files

- core/src/analysis/api/mod.rs

### Public API

#### Types

- ApiInventory
- RestApiSummary
- RestEndpoint
- ApiParameter
- ParameterLocation
- ApiResponse
- SchemaRef
- SchemaProperty
- AuthMethod
- RateLimitInfo
- GrpcSummary
- GrpcService
- GrpcMethod
- GrpcMessage
- GrpcField
- GraphQLSummary
- GraphQLType
- GraphQLTypeKind
- GraphQLField
- GraphQLArg
- GraphQLOperation
- WebhookDefinition
- RetryPolicy
- ApiStatistics
- ApiAnalyzer

#### Functions

- new
- analyze

### Dependencies

- serde::{Deserialize, Serialize}
- std::path
- rest::RestAnalyzer
- grpc::GrpcAnalyzer
- graphql::GraphQLAnalyzer
- webhooks::WebhookAnalyzer

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 370 |
| Functions | 3 |
| Types | 25 |
| Coupling Score | 0.18 |
| Cohesion Score | 0.82 |

---

## lib

**Path**: `core::lib`

### Files

- core/src/lib.rs

### Dependencies

- analysis::analyzer
- analysis::{ApiAnalyzer, BusinessLogicAnalyzer}
- docs::{DocGenerator, DocGeneratorConfig, GeneratedDocs}
- errors::{RiwaqError, Result}
- llm::{HttpLlmClient, LLMClient, LLMConfig, LLMResponse, ContextBuilder}
- models::snapshot
- server::{create_router, AppState}
- cache::CacheManager
- security::{PathValidator, SecretDetector, RateLimiter}
- metrics::{MetricsCollector, HealthChecker, HealthStatus}
- diagrams::{DiagramRenderer, DiagramFormat, RenderedDiagram}

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 86 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## api_reference

**Path**: `core::docs::api_reference`

### Files

- core/src/docs/api_reference.rs

### Public API

#### Types

- ApiReferenceGenerator

#### Functions

- generate

### Dependencies

- crate::analysis
- std::io

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 396 |
| Functions | 6 |
| Types | 1 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqProjectSettings

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqProjectSettings`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/settings/RiwaqProjectSettings.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 52 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/AskPanel

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/AskPanel`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/toolwindow/AskPanel.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 239 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## visual-studio-extension/ToolWindows/ArchitectureToolWindow

**Path**: `visual-studio-extension/ToolWindows/ArchitectureToolWindow`

### Files

- visual-studio-extension/ToolWindows/ArchitectureToolWindow.cs

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 273 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## prompts

**Path**: `core::llm::prompts`

### Files

- core/src/llm/prompts.rs

### Public API

#### Functions

- architecture_overview_prompt
- component_doc_prompt
- adr_generation_prompt
- question_answer_prompt
- api_documentation_prompt
- dependency_analysis_prompt
- truncate_context

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 333 |
| Functions | 7 |
| Types | 0 |
| Coupling Score | 0.00 |
| Cohesion Score | 1.00 |

---

## cache

**Path**: `core::cache`

### Files

- core/src/cache.rs

### Public API

#### Types

- CacheEntry
- MemoryCache
- CacheStats
- AnalysisCacheKey
- LLMCacheKey
- CacheManager
- AllCacheStats

#### Functions

- new
- is_expired
- new
- get
- insert
- insert_with_ttl
- remove
- clear
- stats
- cleanup
- new
- new
- new
- clear_all
- cleanup
- all_stats

### Dependencies

- std::collections
- std::hash
- std::path
- std::sync
- std::time

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 296 |
| Functions | 19 |
| Types | 7 |
| Coupling Score | 0.18 |
| Cohesion Score | 0.82 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/InsightsPanel

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/InsightsPanel`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/toolwindow/InsightsPanel.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 85 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/OpenArchitecturePanelAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/OpenArchitecturePanelAction`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/actions/OpenArchitecturePanelAction.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 24 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## markdown

**Path**: `core::docs::markdown`

### Files

- core/src/docs/markdown.rs

### Public API

#### Types

- MarkdownBuilder
- AdmonitionKind

#### Functions

- new
- h1
- h2
- h3
- h4
- paragraph
- bold
- italic
- code_inline
- code_block
- blockquote
- bullet_list
- numbered_list
- table
- hr
- link
- image
- raw
- newline
- toc
- collapsible
- admonition
- build
- as_str
- file_link
- comma_list
- escape

### Dependencies

- std::fmt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 277 |
| Functions | 27 |
| Types | 2 |
| Coupling Score | 0.03 |
| Cohesion Score | 0.97 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/RiwaqPlugin

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/RiwaqPlugin`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/RiwaqPlugin.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 62 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## git

**Path**: `core::analysis::git`

### Files

- core/src/analysis/git.rs

### Public API

#### Types

- GitAnalysisConfig
- GitAnalyzer

#### Functions

- new
- with_config
- max_commits
- analyze
- identify_hotspots

### Dependencies

- chrono::{TimeZone, Utc}
- git2::{Commit, DiffOptions, Repository, Sort}
- std::collections
- std::path
- tracing::info
- crate::errors
- crate::models

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 623 |
| Functions | 16 |
| Types | 2 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## visual-studio-extension/Commands/StartServerCommand

**Path**: `visual-studio-extension/Commands/StartServerCommand`

### Files

- visual-studio-extension/Commands/StartServerCommand.cs

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 45 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## integrations

**Path**: `core::analysis::business::integrations`

### Files

- core/src/analysis/business/integrations.rs

### Public API

#### Types

- IntegrationAnalyzer

#### Functions

- new
- analyze

### Dependencies

- super::*
- regex::Regex
- std::collections
- std::path
- tokio::fs
- walkdir::WalkDir

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 257 |
| Functions | 7 |
| Types | 1 |
| Coupling Score | 0.67 |
| Cohesion Score | 0.33 |

---

## mod

**Path**: `core::diagrams::mod`

### Files

- core/src/diagrams/mod.rs

### Dependencies

- renderer::{
    DiagramRenderer,
    DiagramRendererConfig,
    DiagramFormat,
    RenderedDiagram,
    render_architecture_diagram,
    render_er_diagram,
    render_state_diagram,
}

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 19 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## visual-studio-extension/PkgCmdIDList

**Path**: `visual-studio-extension/PkgCmdIDList`

### Files

- visual-studio-extension/PkgCmdIDList.cs

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 14 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/AskAboutSelectionAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/AskAboutSelectionAction`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/actions/AskAboutSelectionAction.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 33 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## business_model

**Path**: `core::docs::business_model`

### Files

- core/src/docs/business_model.rs

### Public API

#### Types

- BusinessModelGenerator

#### Functions

- generate

### Dependencies

- crate::analysis

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 377 |
| Functions | 7 |
| Types | 1 |
| Coupling Score | 0.33 |
| Cohesion Score | 0.67 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/DiagramPanel

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/DiagramPanel`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/toolwindow/DiagramPanel.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 137 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## vscode-extension/extension

**Path**: `vscode-extension/extension`

### Files

- vscode-extension/src/extension.ts

### Public API

#### Functions

- showServerMenu
- analyzeWorkspace
- generateDocs
- askQuestion
- showArchitecture
- refreshSnapshot

### Dependencies

- * as vscode from 'vscode
- * as path from 'path
- { RiwaqClient } from '.
- { ServerManager } from '.
- { ArchitectureTreeProvider } from '.
- { ModulesTreeProvider } from '.
- { InsightsTreeProvider } from '.
- { AskPanel } from '.
- { ArchitecturePanel } from '.

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 203 |
| Functions | 6 |
| Types | 0 |
| Coupling Score | 0.60 |
| Cohesion Score | 0.40 |

---

## rules

**Path**: `core::analysis::business::rules`

### Files

- core/src/analysis/business/rules.rs

### Public API

#### Types

- RuleAnalyzer

#### Functions

- new
- analyze

### Dependencies

- super::*
- regex::Regex
- std::path
- tokio::fs
- walkdir::WalkDir

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 275 |
| Functions | 11 |
| Types | 1 |
| Coupling Score | 0.62 |
| Cohesion Score | 0.38 |

---

## grpc

**Path**: `core::analysis::api::grpc`

### Files

- core/src/analysis/api/grpc.rs

### Public API

#### Types

- GrpcAnalyzer

#### Functions

- new
- analyze

### Dependencies

- super::*
- regex::Regex
- std::path
- tokio::fs
- walkdir::WalkDir

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 265 |
| Functions | 9 |
| Types | 1 |
| Coupling Score | 0.62 |
| Cohesion Score | 0.38 |

---

## client

**Path**: `core::llm::client`

### Files

- core/src/llm/client.rs

### Public API

#### Types

- LLMResponse
- ArchitectureOverview
- ComponentSummary
- Adr
- AnswerWithRefs
- FileReference
- LLMClient
- HttpLlmClient
- MockLlmClient

#### Functions

- new
- with_defaults
- new
- add_response

### Dependencies

- async_trait::async_trait
- reqwest::Client
- serde::{Deserialize, Serialize}
- std::time
- tracing::{debug, info, warn}
- super::config
- super::prompts
- crate::errors
- crate::models

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 644 |
| Functions | 19 |
| Types | 14 |
| Coupling Score | 0.41 |
| Cohesion Score | 0.59 |

---

## main

**Path**: `core::main`

### Files

- core/src/main.rs

### Dependencies

- clap::{Parser, Subcommand}
- riwaq_core::logging
- std::path
- tracing::info

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 273 |
| Functions | 5 |
| Types | 2 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/client/RiwaqClient

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/client/RiwaqClient`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/client/RiwaqClient.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 244 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## vscode-extension/views/architectureTree

**Path**: `vscode-extension/views/architectureTree`

### Files

- vscode-extension/src/views/architectureTree.ts

### Public API

#### Types

- ArchitectureItem

### Dependencies

- * as vscode from 'vscode
- { RiwaqClient } from '..

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 136 |
| Functions | 0 |
| Types | 1 |
| Coupling Score | 0.67 |
| Cohesion Score | 0.33 |

---

## workflows

**Path**: `core::analysis::business::workflows`

### Files

- core/src/analysis/business/workflows.rs

### Public API

#### Types

- WorkflowAnalyzer

#### Functions

- new
- analyze

### Dependencies

- super::*
- regex::Regex
- std::collections
- std::path
- tokio::fs
- walkdir::WalkDir

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 395 |
| Functions | 15 |
| Types | 1 |
| Coupling Score | 0.67 |
| Cohesion Score | 0.33 |

---

## vscode-extension/views/modulesTree

**Path**: `vscode-extension/views/modulesTree`

### Files

- vscode-extension/src/views/modulesTree.ts

### Public API

#### Types

- ModuleItem

### Dependencies

- * as vscode from 'vscode
- { RiwaqClient, ModuleSummary } from '..

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 180 |
| Functions | 0 |
| Types | 1 |
| Coupling Score | 0.67 |
| Cohesion Score | 0.33 |

---

## function

**Path**: `core::models::function`

### Files

- core/src/models/function.rs

### Public API

#### Types

- FunctionSummary
- ParameterInfo
- ComplexityMetrics

#### Functions

- new
- line_count
- is_test
- new
- with_type
- new
- is_high_complexity

### Dependencies

- serde::{Deserialize, Serialize}
- super::file

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 197 |
| Functions | 8 |
| Types | 3 |
| Coupling Score | 0.17 |
| Cohesion Score | 0.83 |

---

## metrics

**Path**: `core::metrics`

### Files

- core/src/metrics.rs

### Public API

#### Types

- HealthStatus
- HealthState
- HealthCheck
- MetricsCollector
- OperationTracker
- MetricsSnapshot
- TimingStats
- HealthChecker
- Timer

#### Functions

- new
- increment
- increment_error
- record_timing
- start_operation
- snapshot
- uptime
- reset
- with_name
- complete
- from_durations
- new
- add_check
- check
- is_alive
- is_ready
- memory_check
- disk_check
- new
- elapsed
- elapsed_ms

### Dependencies

- std::collections
- std::sync
- std::time
- serde::{Serialize, Deserialize}

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 445 |
| Functions | 26 |
| Types | 9 |
| Coupling Score | 0.12 |
| Cohesion Score | 0.88 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/ArchitectureToolWindowFactory

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/ArchitectureToolWindowFactory`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/toolwindow/ArchitectureToolWindowFactory.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 19 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## mod

**Path**: `core::docs::mod`

### Files

- core/src/docs/mod.rs

### Dependencies

- generator::{DocGenerator, DocGeneratorConfig, GeneratedDocs}
- api_reference::ApiReferenceGenerator
- business_model::BusinessModelGenerator

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 22 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## logging

**Path**: `core::logging`

### Files

- core/src/logging.rs

### Public API

#### Types

- TimingGuard
- ProgressReporter

#### Functions

- init_logging
- init_json_logging
- new
- new
- tick
- finish

### Dependencies

- tracing::Level
- tracing_subscriber::{
    fmt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 187 |
| Functions | 7 |
| Types | 2 |
| Coupling Score | 0.20 |
| Cohesion Score | 0.80 |

---

## security

**Path**: `core::security`

### Files

- core/src/security.rs

### Public API

#### Types

- ValidationResult
- SecurityIssue
- IssueSeverity
- IssueCategory
- PathValidator
- SecretDetector
- InputSanitizer
- RateLimiter

#### Functions

- new
- validate
- sanitize
- new
- scan
- has_secrets
- sanitize_html
- sanitize_markdown
- sanitize_project_name
- is_valid_url
- new
- check
- remaining

### Dependencies

- std::path
- std::collections
- regex::Regex

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 410 |
| Functions | 16 |
| Types | 9 |
| Coupling Score | 0.12 |
| Cohesion Score | 0.88 |

---

## analyzer

**Path**: `core::analysis::analyzer`

### Files

- core/src/analysis/analyzer.rs

### Public API

#### Types

- AnalyzerConfig
- CodebaseAnalyzer

#### Functions

- new
- with_config
- with_max_commits
- with_include_tests
- without_git
- without_parsing
- analyze

### Dependencies

- std::collections
- std::path
- std::time
- tracing::{info, warn}
- crate::analysis
- crate::errors
- crate::logging
- crate::models

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 583 |
| Functions | 17 |
| Types | 2 |
| Coupling Score | 0.47 |
| Cohesion Score | 0.53 |

---

## visual-studio-extension/RiwaqClient

**Path**: `visual-studio-extension/RiwaqClient`

### Files

- visual-studio-extension/RiwaqClient.cs

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 152 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## parser

**Path**: `core::analysis::parser`

### Files

- core/src/analysis/parser.rs

### Public API

#### Types

- CodeParser

#### Functions

- new
- parse_file

### Dependencies

- std::path
- tracing::debug
- crate::errors
- crate::models

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 952 |
| Functions | 34 |
| Types | 1 |
| Coupling Score | 0.57 |
| Cohesion Score | 0.43 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/AnalyzeProjectAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/AnalyzeProjectAction`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/actions/AnalyzeProjectAction.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 67 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/RestartServerAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/RestartServerAction`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/actions/RestartServerAction.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 37 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## module

**Path**: `core::models::module`

### Files

- core/src/models/module.rs

### Public API

#### Types

- ModuleSummary
- ModuleDependency
- DependencyKind
- ModuleMetrics
- DependencyGraph
- DependencyEdge

#### Functions

- new
- is_leaf
- public_item_count
- has_high_coupling
- has_low_cohesion
- new
- add_node
- add_edge
- get_dependencies
- get_dependents
- find_circular_dependencies
- to_mermaid

### Dependencies

- serde::{Deserialize, Serialize}
- std::collections
- std::path

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 323 |
| Functions | 14 |
| Types | 6 |
| Coupling Score | 0.14 |
| Cohesion Score | 0.86 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqSettings

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqSettings`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/settings/RiwaqSettings.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 70 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## visual-studio-extension/Commands/StopServerCommand

**Path**: `visual-studio-extension/Commands/StopServerCommand`

### Files

- visual-studio-extension/Commands/StopServerCommand.cs

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 34 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## mermaid

**Path**: `core::docs::mermaid`

### Files

- core/src/docs/mermaid.rs

### Public API

#### Functions

- generate_dependency_diagram
- generate_architecture_diagram
- generate_dataflow_diagram
- generate_type_diagram
- generate_state_diagram
- generate_er_diagram
- generate_sequence_diagram
- generate_component_diagram
- sanitize_id

### Dependencies

- crate::models
- crate::analysis

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 368 |
| Functions | 9 |
| Types | 0 |
| Coupling Score | 0.18 |
| Cohesion Score | 0.82 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/StopServerAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/StopServerAction`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/actions/StopServerAction.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 37 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/statusbar/RiwaqStatusBarWidget

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/statusbar/RiwaqStatusBarWidget`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/statusbar/RiwaqStatusBarWidget.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 46 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/listeners/ProjectManagerListener

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/listeners/ProjectManagerListener`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/listeners/ProjectManagerListener.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 22 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/ArchitecturePanel

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/ArchitecturePanel`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/toolwindow/ArchitecturePanel.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 116 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## mod

**Path**: `core::analysis::business::mod`

### Files

- core/src/analysis/business/mod.rs

### Public API

#### Types

- BusinessLogicInventory
- Entity
- EntityType
- EntityField
- Validation
- EntityRelationship
- RelationshipType
- Workflow
- WorkflowState
- StateTransition
- BusinessRule
- BusinessRuleType
- RuleSeverity
- Integration
- IntegrationType
- BusinessStatistics
- BusinessLogicAnalyzer

#### Functions

- new
- analyze

### Dependencies

- serde::{Deserialize, Serialize}
- std::path
- entities::EntityAnalyzer
- workflows::WorkflowAnalyzer
- rules::RuleAnalyzer
- integrations::IntegrationAnalyzer

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 285 |
| Functions | 3 |
| Types | 17 |
| Coupling Score | 0.24 |
| Cohesion Score | 0.76 |

---

## webhooks

**Path**: `core::analysis::api::webhooks`

### Files

- core/src/analysis/api/webhooks.rs

### Public API

#### Types

- WebhookAnalyzer

#### Functions

- new
- analyze

### Dependencies

- super::*
- regex::Regex
- std::path
- tokio::fs
- walkdir::WalkDir

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 224 |
| Functions | 9 |
| Types | 1 |
| Coupling Score | 0.62 |
| Cohesion Score | 0.38 |

---

## mod

**Path**: `core::server::mod`

### Files

- core/src/server/mod.rs

### Dependencies

- routes::create_router
- state::AppState

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 14 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## routes

**Path**: `core::server::routes`

### Files

- core/src/server/routes.rs

### Public API

#### Functions

- create_router
- run_server

### Dependencies

- axum::{
    routing
- tower_http::cors
- super::handlers
- super::state

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 55 |
| Functions | 2 |
| Types | 0 |
| Coupling Score | 0.67 |
| Cohesion Score | 0.33 |

---

## git

**Path**: `core::models::git`

### Files

- core/src/models/git.rs

### Public API

#### Types

- GitInsights
- RepositoryInfo
- CommitInfo
- FileChange
- FileChangeKind
- FileChurn
- AuthorStats
- CoChangePattern
- Hotspot
- Domain

#### Functions

- new
- top_churning_files
- top_authors
- parse_conventional_tags
- is_hotspot
- calculate_risk

### Dependencies

- chrono::{DateTime, Utc}
- serde::{Deserialize, Serialize}
- std::collections
- std::path

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 313 |
| Functions | 6 |
| Types | 10 |
| Coupling Score | 0.20 |
| Cohesion Score | 0.80 |

---

## snapshot

**Path**: `core::models::snapshot`

### Files

- core/src/models/snapshot.rs

### Public API

#### Types

- CodebaseSnapshot
- SnapshotMetadata
- AnalysisConfig
- CodebaseStatistics
- LanguageCount
- ServiceInfo
- ServiceKind
- ApiEndpoint
- ApiParameter
- ParameterLocation

#### Functions

- new
- get_file
- get_module
- compute_statistics
- new

### Dependencies

- serde::{Deserialize, Serialize}
- std::path
- super::file
- super::git
- super::module

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 332 |
| Functions | 7 |
| Types | 10 |
| Coupling Score | 0.25 |
| Cohesion Score | 0.75 |

---

## mod

**Path**: `core::analysis::mod`

### Files

- core/src/analysis/mod.rs

### Dependencies

- api::ApiAnalyzer
- business::BusinessLogicAnalyzer

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 15 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## rest

**Path**: `core::analysis::api::rest`

### Files

- core/src/analysis/api/rest.rs

### Public API

#### Types

- RestAnalyzer

#### Functions

- new
- analyze

### Dependencies

- super::*
- regex::Regex
- std::collections
- std::path
- tokio::fs
- walkdir::WalkDir

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 415 |
| Functions | 17 |
| Types | 3 |
| Coupling Score | 0.67 |
| Cohesion Score | 0.33 |

---

## mod

**Path**: `core::llm::mod`

### Files

- core/src/llm/mod.rs

### Dependencies

- client::{HttpLlmClient, LLMClient, LLMResponse}
- config::LLMConfig
- context::{ContextBuilder, EnhancedContext, QuestionIntent}

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 19 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## state

**Path**: `core::server::state`

### Files

- core/src/server/state.rs

### Public API

#### Types

- AppState
- ProjectInfo

#### Functions

- new
- with_defaults
- get_snapshot
- set_snapshot
- get_llm_client
- update_llm_config
- list_projects

### Dependencies

- std::collections
- std::sync
- tokio::sync
- crate::llm
- crate::models

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 108 |
| Functions | 9 |
| Types | 2 |
| Coupling Score | 0.36 |
| Cohesion Score | 0.64 |

---

## file

**Path**: `core::models::file`

### Files

- core/src/models/file.rs

### Public API

#### Types

- Language
- FileSummary
- TypeSummary
- TypeKind
- Visibility
- FieldInfo
- ImportInfo
- ExportInfo

#### Functions

- from_extension
- is_supported
- extension
- new
- file_name

### Dependencies

- serde::{Deserialize, Serialize}
- std::path
- super::function

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 308 |
| Functions | 6 |
| Types | 8 |
| Coupling Score | 0.19 |
| Cohesion Score | 0.81 |

---

## visual-studio-extension/Commands/AnalyzeProjectCommand

**Path**: `visual-studio-extension/Commands/AnalyzeProjectCommand`

### Files

- visual-studio-extension/Commands/AnalyzeProjectCommand.cs

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 78 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## generator

**Path**: `core::docs::generator`

### Files

- core/src/docs/generator.rs

### Public API

#### Types

- DocGeneratorConfig
- GeneratedDocs
- GeneratedFile
- DocType
- DocGenerator

#### Functions

- new
- with_llm
- with_llm_config
- with_project_name
- new
- generate

### Dependencies

- std::fs
- std::path
- tracing::info
- super::markdown
- super::mermaid
- crate::errors
- crate::llm
- crate::models

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 647 |
| Functions | 15 |
| Types | 5 |
| Coupling Score | 0.42 |
| Cohesion Score | 0.58 |

---

## visual-studio-extension/ServerManager

**Path**: `visual-studio-extension/ServerManager`

### Files

- visual-studio-extension/ServerManager.cs

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 168 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## visual-studio-extension/SettingsOptions

**Path**: `visual-studio-extension/SettingsOptions`

### Files

- visual-studio-extension/SettingsOptions.cs

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 45 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## config

**Path**: `core::llm::config`

### Files

- core/src/llm/config.rs

### Public API

#### Types

- LLMConfig

#### Functions

- new
- with_api_key
- with_model
- with_max_tokens
- with_temperature
- with_thinking_mode
- validate
- estimate_tokens
- fits_in_context

### Dependencies

- serde::{Deserialize, Serialize}

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 191 |
| Functions | 17 |
| Types | 1 |
| Coupling Score | 0.09 |
| Cohesion Score | 0.91 |

---

## vscode-extension/client

**Path**: `vscode-extension/client`

### Files

- vscode-extension/src/client.ts

### Dependencies

- axios, { AxiosInstance } from 'axios

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 199 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/ModulesPanel

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/ModulesPanel`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/toolwindow/ModulesPanel.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 105 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/StartServerAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/StartServerAction`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/actions/StartServerAction.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 37 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

## jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqProjectConfigurable

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqProjectConfigurable`

### Files

- jetbrains-plugin/src/main/kotlin/com/riwaq/arch/settings/RiwaqProjectConfigurable.kt

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 75 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

---

