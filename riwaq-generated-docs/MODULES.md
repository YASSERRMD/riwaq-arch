# Modules & Services

Detailed documentation for each module in the codebase.

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

## lib

**Path**: `core::lib`

### Files

- core/src/lib.rs

### Dependencies

- analysis::analyzer
- docs::{DocGenerator, DocGeneratorConfig, GeneratedDocs}
- errors::{RiwaqError, Result}
- llm::{HttpLlmClient, LLMClient, LLMConfig, LLMResponse}
- models::snapshot
- server::{create_router, AppState}

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 73 |
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

## mod

**Path**: `core::llm::mod`

### Files

- core/src/llm/mod.rs

### Dependencies

- client::{HttpLlmClient, LLMClient, LLMResponse}
- config::LLMConfig

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

### Dependencies

- crate::models

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 189 |
| Functions | 5 |
| Types | 0 |
| Coupling Score | 0.20 |
| Cohesion Score | 0.80 |

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

**Path**: `core::analysis::mod`

### Files

- core/src/analysis/mod.rs

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 6 |
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

## mod

**Path**: `core::docs::mod`

### Files

- core/src/docs/mod.rs

### Dependencies

- generator::{DocGenerator, DocGeneratorConfig, GeneratedDocs}

### Metrics

| Metric | Value |
| --- | --- |
| Files | 1 |
| Lines of Code | 16 |
| Functions | 0 |
| Types | 0 |
| Coupling Score | 0.50 |
| Cohesion Score | 0.50 |

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

