# riwaq-arch - Architecture Overview

## System Overview

This document provides an architectural overview of the riwaq-arch codebase, consisting of 86 modules across 86 files with 18628 total lines of code.

## Architecture Diagram

```mermaid
graph TB
    classDef service fill:#7c3aed,stroke:#5b21b6,color:#fff,stroke-width:2px
    classDef module fill:#4f46e5,stroke:#3730a3,color:#fff
    classDef external fill:#6366f1,stroke:#4f46e5,color:#fff,stroke-dasharray: 5 5

    subgraph SERVICES["🚀 Services"]
        direction TB
        src["🌐 src"]
    end

    subgraph visual_studio_extension["📦 visual-studio-extension"]
        direction TB
        visual_studio_extension_GuidList["visual-studio-extension/GuidList"]
        visual_studio_extension_RiwaqArchPackage["visual-studio-extension/RiwaqArchPackage"]
        visual_studio_extension_ToolWindows_ArchitectureToolWindow["visual-studio-extension/ToolWindows/ArchitectureToolWindow"]
        visual_studio_extension_Commands_StartServerCommand["visual-studio-extension/Commands/StartServerCommand"]
        visual_studio_extension_PkgCmdIDList["visual-studio-extension/PkgCmdIDList"]
        visual_studio_extension_RiwaqClient["visual-studio-extension/RiwaqClient"]
        visual_studio_extension_Commands_StopServerCommand["visual-studio-extension/Commands/StopServerCommand"]
        visual_studio_extension_Commands_AnalyzeProjectCommand["visual-studio-extension/Commands/AnalyzeProjectCommand"]
        visual_studio_extension_ServerManager["visual-studio-extension/ServerManager"]
        visual_studio_extension_SettingsOptions["visual-studio-extension/SettingsOptions"]
    end
    subgraph core["📦 core"]
        direction TB
        context["context"]
        renderer["renderer"]
        errors["errors"]
        mod["mod"]
        fs_scanner["fs_scanner"]
        entities["entities"]
        graphql["graphql"]
        handlers["handlers"]
        mod["mod"]
        lib["lib"]
        coremore["... +35 more"]
    end
    subgraph vscode_extension["📦 vscode-extension"]
        direction TB
        vscode_extension_panels_architecturePanel["vscode-extension/panels/architecturePanel"]
        vscode_extension_panels_askPanel["vscode-extension/panels/askPanel"]
        vscode_extension_serverManager["vscode-extension/serverManager"]
        vscode_extension_views_insightsTree["vscode-extension/views/insightsTree"]
        vscode_extension_extension["vscode-extension/extension"]
        vscode_extension_views_architectureTree["vscode-extension/views/architectureTree"]
        vscode_extension_views_modulesTree["vscode-extension/views/modulesTree"]
        vscode_extension_client["vscode-extension/client"]
    end
    subgraph jetbrains_plugin["📦 jetbrains-plugin"]
        direction TB
        jetbrains_plugin_main_kotlin_com_riwaq_arch_server_ServerManager["jetbrains-plugin/main/kotlin/com/riwaq/arch/server/ServerManager"]
        jetbrains_plugin_main_kotlin_com_riwaq_arch_client_models_ApiModels["jetbrains-plugin/main/kotlin/com/riwaq/arch/client/models/ApiModels"]
        jetbrains_plugin_main_kotlin_com_riwaq_arch_settings_RiwaqConfigurable["jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqConfigurable"]
        jetbrains_plugin_main_kotlin_com_riwaq_arch_actions_OpenAskPanelAction["jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/OpenAskPanelAction"]
        jetbrains_plugin_main_kotlin_com_riwaq_arch_settings_RiwaqProjectSettings["jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqProjectSettings"]
        jetbrains_plugin_main_kotlin_com_riwaq_arch_toolwindow_AskPanel["jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/AskPanel"]
        jetbrains_plugin_main_kotlin_com_riwaq_arch_toolwindow_InsightsPanel["jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/InsightsPanel"]
        jetbrains_plugin_main_kotlin_com_riwaq_arch_actions_OpenArchitecturePanelAction["jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/OpenArchitecturePanelAction"]
        jetbrains_plugin_main_kotlin_com_riwaq_arch_RiwaqPlugin["jetbrains-plugin/main/kotlin/com/riwaq/arch/RiwaqPlugin"]
        jetbrains_plugin_main_kotlin_com_riwaq_arch_actions_AskAboutSelectionAction["jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/AskAboutSelectionAction"]
        jetbrains_pluginmore["... +13 more"]
    end

    class src service
```

## Services & Entry Points

| Service | Type | Entry File |
| --- | --- | --- |
| src | HttpServer | core/src/main.rs |

## Modules

### jetbrains-plugin/main/kotlin/com/riwaq/arch/server/ServerManager

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/server/ServerManager`

**Files**: 1


### visual-studio-extension/GuidList

**Path**: `visual-studio-extension/GuidList`

**Files**: 1


### vscode-extension/panels/architecturePanel

**Path**: `vscode-extension/panels/architecturePanel`

**Files**: 1


### jetbrains-plugin/main/kotlin/com/riwaq/arch/client/models/ApiModels

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/client/models/ApiModels`

**Files**: 1


### vscode-extension/panels/askPanel

**Path**: `vscode-extension/panels/askPanel`

**Files**: 1


### context

**Path**: `core::llm::context`

**Files**: 1

**Public Functions**: `new`, `with_max_chars`, `build_for_question`


### renderer

**Path**: `core::diagrams::renderer`

**Files**: 1

**Public Functions**: `extension`, `new`, `render`, `render_to_svg`, `render_architecture_diagram`, `render_er_diagram`, `render_state_diagram`


### vscode-extension/serverManager

**Path**: `vscode-extension/serverManager`

**Files**: 1


### errors

**Path**: `core::errors`

**Files**: 1

**Public Functions**: `file_system`, `parse_error`, `llm_error`, `config`, `internal`, `is_retryable`, `retry_after`


### mod

**Path**: `core::models::mod`

**Files**: 1


### jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqConfigurable

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqConfigurable`

**Files**: 1


### fs_scanner

**Path**: `core::analysis::fs_scanner`

**Files**: 1

**Public Functions**: `new`, `exclude_dirs`, `include_extensions`, `max_file_size`, `include_tests`, `max_depth`, `read_contents`, `new`, `scan`, `get_stats`


### visual-studio-extension/RiwaqArchPackage

**Path**: `visual-studio-extension/RiwaqArchPackage`

**Files**: 1


### entities

**Path**: `core::analysis::business::entities`

**Files**: 1

**Public Functions**: `new`, `analyze`


### graphql

**Path**: `core::analysis::api::graphql`

**Files**: 1

**Public Functions**: `new`, `analyze`


### jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/OpenAskPanelAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/OpenAskPanelAction`

**Files**: 1


### vscode-extension/views/insightsTree

**Path**: `vscode-extension/views/insightsTree`

**Files**: 1


### handlers

**Path**: `core::server::handlers`

**Files**: 1

**Public Functions**: `health`, `list_projects`, `analyze`, `generate_docs`, `ask_question`, `get_architecture_diagram`, `get_dependency_diagram`


### mod

**Path**: `core::analysis::api::mod`

**Files**: 1

**Public Functions**: `new`, `analyze`


### lib

**Path**: `core::lib`

**Files**: 1


### api_reference

**Path**: `core::docs::api_reference`

**Files**: 1

**Public Functions**: `generate`


### jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqProjectSettings

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqProjectSettings`

**Files**: 1


### jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/AskPanel

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/AskPanel`

**Files**: 1


### visual-studio-extension/ToolWindows/ArchitectureToolWindow

**Path**: `visual-studio-extension/ToolWindows/ArchitectureToolWindow`

**Files**: 1


### prompts

**Path**: `core::llm::prompts`

**Files**: 1

**Public Functions**: `architecture_overview_prompt`, `component_doc_prompt`, `adr_generation_prompt`, `question_answer_prompt`, `api_documentation_prompt`, `dependency_analysis_prompt`, `truncate_context`


### cache

**Path**: `core::cache`

**Files**: 1

**Public Functions**: `new`, `is_expired`, `new`, `get`, `insert`, `insert_with_ttl`, `remove`, `clear`, `stats`, `cleanup`

... and 6 more


### jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/InsightsPanel

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/InsightsPanel`

**Files**: 1


### jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/OpenArchitecturePanelAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/OpenArchitecturePanelAction`

**Files**: 1


### markdown

**Path**: `core::docs::markdown`

**Files**: 1

**Public Functions**: `new`, `h1`, `h2`, `h3`, `h4`, `paragraph`, `bold`, `italic`, `code_inline`, `code_block`

... and 17 more


### jetbrains-plugin/main/kotlin/com/riwaq/arch/RiwaqPlugin

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/RiwaqPlugin`

**Files**: 1


### git

**Path**: `core::analysis::git`

**Files**: 1

**Public Functions**: `new`, `with_config`, `max_commits`, `analyze`, `identify_hotspots`


### visual-studio-extension/Commands/StartServerCommand

**Path**: `visual-studio-extension/Commands/StartServerCommand`

**Files**: 1


### integrations

**Path**: `core::analysis::business::integrations`

**Files**: 1

**Public Functions**: `new`, `analyze`


### mod

**Path**: `core::diagrams::mod`

**Files**: 1


### visual-studio-extension/PkgCmdIDList

**Path**: `visual-studio-extension/PkgCmdIDList`

**Files**: 1


### jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/AskAboutSelectionAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/AskAboutSelectionAction`

**Files**: 1


### business_model

**Path**: `core::docs::business_model`

**Files**: 1

**Public Functions**: `generate`


### jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/DiagramPanel

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/DiagramPanel`

**Files**: 1


### vscode-extension/extension

**Path**: `vscode-extension/extension`

**Files**: 1

**Public Functions**: `showServerMenu`, `analyzeWorkspace`, `generateDocs`, `askQuestion`, `showArchitecture`, `refreshSnapshot`


### rules

**Path**: `core::analysis::business::rules`

**Files**: 1

**Public Functions**: `new`, `analyze`


### grpc

**Path**: `core::analysis::api::grpc`

**Files**: 1

**Public Functions**: `new`, `analyze`


### client

**Path**: `core::llm::client`

**Files**: 1

**Public Functions**: `new`, `with_defaults`, `new`, `add_response`


### main

**Path**: `core::main`

**Files**: 1


### jetbrains-plugin/main/kotlin/com/riwaq/arch/client/RiwaqClient

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/client/RiwaqClient`

**Files**: 1


### vscode-extension/views/architectureTree

**Path**: `vscode-extension/views/architectureTree`

**Files**: 1


### workflows

**Path**: `core::analysis::business::workflows`

**Files**: 1

**Public Functions**: `new`, `analyze`


### vscode-extension/views/modulesTree

**Path**: `vscode-extension/views/modulesTree`

**Files**: 1


### function

**Path**: `core::models::function`

**Files**: 1

**Public Functions**: `new`, `line_count`, `is_test`, `new`, `with_type`, `new`, `is_high_complexity`


### metrics

**Path**: `core::metrics`

**Files**: 1

**Public Functions**: `new`, `increment`, `increment_error`, `record_timing`, `start_operation`, `snapshot`, `uptime`, `reset`, `with_name`, `complete`

... and 11 more


### jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/ArchitectureToolWindowFactory

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/ArchitectureToolWindowFactory`

**Files**: 1


### mod

**Path**: `core::docs::mod`

**Files**: 1


### logging

**Path**: `core::logging`

**Files**: 1

**Public Functions**: `init_logging`, `init_json_logging`, `new`, `new`, `tick`, `finish`


### security

**Path**: `core::security`

**Files**: 1

**Public Functions**: `new`, `validate`, `sanitize`, `new`, `scan`, `has_secrets`, `sanitize_html`, `sanitize_markdown`, `sanitize_project_name`, `is_valid_url`

... and 3 more


### analyzer

**Path**: `core::analysis::analyzer`

**Files**: 1

**Public Functions**: `new`, `with_config`, `with_max_commits`, `with_include_tests`, `without_git`, `without_parsing`, `analyze`


### visual-studio-extension/RiwaqClient

**Path**: `visual-studio-extension/RiwaqClient`

**Files**: 1


### parser

**Path**: `core::analysis::parser`

**Files**: 1

**Public Functions**: `new`, `parse_file`


### jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/AnalyzeProjectAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/AnalyzeProjectAction`

**Files**: 1


### jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/RestartServerAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/RestartServerAction`

**Files**: 1


### module

**Path**: `core::models::module`

**Files**: 1

**Public Functions**: `new`, `is_leaf`, `public_item_count`, `has_high_coupling`, `has_low_cohesion`, `new`, `add_node`, `add_edge`, `get_dependencies`, `get_dependents`

... and 2 more


### jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqSettings

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqSettings`

**Files**: 1


### visual-studio-extension/Commands/StopServerCommand

**Path**: `visual-studio-extension/Commands/StopServerCommand`

**Files**: 1


### mermaid

**Path**: `core::docs::mermaid`

**Files**: 1

**Public Functions**: `generate_dependency_diagram`, `generate_architecture_diagram`, `generate_dataflow_diagram`, `generate_type_diagram`, `generate_state_diagram`, `generate_er_diagram`, `generate_sequence_diagram`, `generate_component_diagram`, `sanitize_id`


### jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/StopServerAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/StopServerAction`

**Files**: 1


### jetbrains-plugin/main/kotlin/com/riwaq/arch/statusbar/RiwaqStatusBarWidget

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/statusbar/RiwaqStatusBarWidget`

**Files**: 1


### jetbrains-plugin/main/kotlin/com/riwaq/arch/listeners/ProjectManagerListener

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/listeners/ProjectManagerListener`

**Files**: 1


### jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/ArchitecturePanel

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/ArchitecturePanel`

**Files**: 1


### mod

**Path**: `core::analysis::business::mod`

**Files**: 1

**Public Functions**: `new`, `analyze`


### webhooks

**Path**: `core::analysis::api::webhooks`

**Files**: 1

**Public Functions**: `new`, `analyze`


### mod

**Path**: `core::server::mod`

**Files**: 1


### routes

**Path**: `core::server::routes`

**Files**: 1

**Public Functions**: `create_router`, `run_server`


### git

**Path**: `core::models::git`

**Files**: 1

**Public Functions**: `new`, `top_churning_files`, `top_authors`, `parse_conventional_tags`, `is_hotspot`, `calculate_risk`


### snapshot

**Path**: `core::models::snapshot`

**Files**: 1

**Public Functions**: `new`, `get_file`, `get_module`, `compute_statistics`, `new`


### mod

**Path**: `core::analysis::mod`

**Files**: 1


### rest

**Path**: `core::analysis::api::rest`

**Files**: 1

**Public Functions**: `new`, `analyze`


### mod

**Path**: `core::llm::mod`

**Files**: 1


### state

**Path**: `core::server::state`

**Files**: 1

**Public Functions**: `new`, `with_defaults`, `get_snapshot`, `set_snapshot`, `get_llm_client`, `update_llm_config`, `list_projects`


### file

**Path**: `core::models::file`

**Files**: 1

**Public Functions**: `from_extension`, `is_supported`, `extension`, `new`, `file_name`


### visual-studio-extension/Commands/AnalyzeProjectCommand

**Path**: `visual-studio-extension/Commands/AnalyzeProjectCommand`

**Files**: 1


### generator

**Path**: `core::docs::generator`

**Files**: 1

**Public Functions**: `new`, `with_llm`, `with_llm_config`, `with_project_name`, `new`, `generate`


### visual-studio-extension/ServerManager

**Path**: `visual-studio-extension/ServerManager`

**Files**: 1


### visual-studio-extension/SettingsOptions

**Path**: `visual-studio-extension/SettingsOptions`

**Files**: 1


### config

**Path**: `core::llm::config`

**Files**: 1

**Public Functions**: `new`, `with_api_key`, `with_model`, `with_max_tokens`, `with_temperature`, `with_thinking_mode`, `validate`, `estimate_tokens`, `fits_in_context`


### vscode-extension/client

**Path**: `vscode-extension/client`

**Files**: 1


### jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/ModulesPanel

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/toolwindow/ModulesPanel`

**Files**: 1


### jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/StartServerAction

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/actions/StartServerAction`

**Files**: 1


### jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqProjectConfigurable

**Path**: `jetbrains-plugin/main/kotlin/com/riwaq/arch/settings/RiwaqProjectConfigurable`

**Files**: 1


## Module Dependencies

```mermaid
graph TD
    classDef module fill:#4f46e5,stroke:#3730a3,color:#fff
    classDef circular fill:#ef4444,stroke:#dc2626,color:#fff

    core__analysis__api__grpc[core::analysis::api::grpc] --> super___[super::*]
    core__analysis__api__grpc[core::analysis::api::grpc] --> regex__Regex[regex::Regex]
    core__analysis__api__grpc[core::analysis::api::grpc] --> std__path[std::path]
    core__analysis__api__grpc[core::analysis::api::grpc] --> tokio__fs[tokio::fs]
    core__analysis__api__grpc[core::analysis::api::grpc] --> walkdir__WalkDir[walkdir::WalkDir]
    core__analysis__business__mod[core::analysis::business::mod] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__analysis__business__mod[core::analysis::business::mod] --> std__path[std::path]
    core__analysis__business__mod[core::analysis::business::mod] --> entities__EntityAnalyzer[entities::EntityAnalyzer]
    core__analysis__business__mod[core::analysis::business::mod] --> workflows__WorkflowAnalyzer[workflows::WorkflowAnalyzer]
    core__analysis__business__mod[core::analysis::business::mod] --> rules__RuleAnalyzer[rules::RuleAnalyzer]
    core__analysis__business__mod[core::analysis::business::mod] --> integrations__IntegrationAnalyzer[integrations::IntegrationAnalyzer]
    core__analysis__business__workflows[core::analysis::business::workflows] --> super___[super::*]
    core__analysis__business__workflows[core::analysis::business::workflows] --> regex__Regex[regex::Regex]
    core__analysis__business__workflows[core::analysis::business::workflows] --> std__collections[std::collections]
    core__analysis__business__workflows[core::analysis::business::workflows] --> std__path[std::path]
    core__analysis__business__workflows[core::analysis::business::workflows] --> tokio__fs[tokio::fs]
    core__analysis__business__workflows[core::analysis::business::workflows] --> walkdir__WalkDir[walkdir::WalkDir]
    core__lib[core::lib] --> analysis__analyzer[analysis::analyzer]
    core__lib[core::lib] --> analysis___ApiAnalyzer__BusinessLogicAnalyzer_[analysis::{ApiAnalyzer, BusinessLogicAnalyzer}]
    core__lib[core::lib] --> docs___DocGenerator__DocGeneratorConfig__GeneratedDocs_[docs::{DocGenerator, DocGeneratorConfig, GeneratedDocs}]
    core__lib[core::lib] --> errors___RiwaqError__Result_[errors::{RiwaqError, Result}]
    core__lib[core::lib] --> llm___HttpLlmClient__LLMClient__LLMConfig__LLMResponse__ContextBuilder_[llm::{HttpLlmClient, LLMClient, LLMConfig, LLMResponse, ContextBuilder}]
    core__lib[core::lib] --> models__snapshot[models::snapshot]
    core__lib[core::lib] --> server___create_router__AppState_[server::{create_router, AppState}]
    core__lib[core::lib] --> cache__CacheManager[cache::CacheManager]
    core__lib[core::lib] --> security___PathValidator__SecretDetector__RateLimiter_[security::{PathValidator, SecretDetector, RateLimiter}]
    core__lib[core::lib] --> metrics___MetricsCollector__HealthChecker__HealthStatus_[metrics::{MetricsCollector, HealthChecker, HealthStatus}]
    core__lib[core::lib] --> diagrams___DiagramRenderer__DiagramFormat__RenderedDiagram_[diagrams::{DiagramRenderer, DiagramFormat, RenderedDiagram}]
    core__analysis__business__entities[core::analysis::business::entities] --> super___[super::*]
    core__analysis__business__entities[core::analysis::business::entities] --> regex__Regex[regex::Regex]
    core__analysis__business__entities[core::analysis::business::entities] --> std__collections[std::collections]
    core__analysis__business__entities[core::analysis::business::entities] --> std__path[std::path]
    core__analysis__business__entities[core::analysis::business::entities] --> tokio__fs[tokio::fs]
    core__analysis__business__entities[core::analysis::business::entities] --> walkdir__WalkDir[walkdir::WalkDir]
    vscode_extension_views_architectureTree[vscode-extension/views/architectureTree] --> __as_vscode_from__vscode[* as vscode from 'vscode]
    vscode_extension_views_architectureTree[vscode-extension/views/architectureTree] --> __RiwaqClient___from____[{ RiwaqClient } from '..]
    core__analysis__fs_scanner[core::analysis::fs_scanner] --> ignore__WalkBuilder[ignore::WalkBuilder]
    core__analysis__fs_scanner[core::analysis::fs_scanner] --> std__collections[std::collections]
    core__analysis__fs_scanner[core::analysis::fs_scanner] --> std__path[std::path]
    core__analysis__fs_scanner[core::analysis::fs_scanner] --> tracing___debug__info__warn_[tracing::{debug, info, warn}]
    core__analysis__fs_scanner[core::analysis::fs_scanner] --> crate__errors[crate::errors]
    core__analysis__fs_scanner[core::analysis::fs_scanner] --> crate__models[crate::models]
    core__cache[core::cache] --> std__collections[std::collections]
    core__cache[core::cache] --> std__hash[std::hash]
    core__cache[core::cache] --> std__path[std::path]
    core__cache[core::cache] --> std__sync[std::sync]
    core__cache[core::cache] --> std__time[std::time]
    core__llm__client[core::llm::client] --> async_trait__async_trait[async_trait::async_trait]
    core__llm__client[core::llm::client] --> reqwest__Client[reqwest::Client]
    core__llm__client[core::llm::client] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__llm__client[core::llm::client] --> std__time[std::time]
    core__llm__client[core::llm::client] --> tracing___debug__info__warn_[tracing::{debug, info, warn}]
    core__llm__client[core::llm::client] --> super__config[super::config]
    core__llm__client[core::llm::client] --> super__prompts[super::prompts]
    core__llm__client[core::llm::client] --> crate__errors[crate::errors]
    core__llm__client[core::llm::client] --> crate__models[crate::models]
    core__models__mod[core::models::mod] --> file__FileSummary[file::FileSummary]
    core__models__mod[core::models::mod] --> function__FunctionSummary[function::FunctionSummary]
    core__models__mod[core::models::mod] --> git__GitInsights[git::GitInsights]
    core__models__mod[core::models::mod] --> module__ModuleSummary[module::ModuleSummary]
    core__models__mod[core::models::mod] --> snapshot__CodebaseSnapshot[snapshot::CodebaseSnapshot]
    core__main[core::main] --> clap___Parser__Subcommand_[clap::{Parser, Subcommand}]
    core__main[core::main] --> riwaq_core__logging[riwaq_core::logging]
    core__main[core::main] --> std__path[std::path]
    core__main[core::main] --> tracing__info[tracing::info]
    core__security[core::security] --> std__path[std::path]
    core__security[core::security] --> std__collections[std::collections]
    core__security[core::security] --> regex__Regex[regex::Regex]
    core__diagrams__renderer[core::diagrams::renderer] --> mermaid_rs__Mermaid[mermaid_rs::Mermaid]
    core__diagrams__renderer[core::diagrams::renderer] --> std__path[std::path]
    core__diagrams__renderer[core::diagrams::renderer] --> tokio__fs[tokio::fs]
    core__diagrams__renderer[core::diagrams::renderer] --> tracing__info[tracing::info]
    vscode_extension_serverManager[vscode-extension/serverManager] --> __as_vscode_from__vscode[* as vscode from 'vscode]
    vscode_extension_serverManager[vscode-extension/serverManager] --> __as_cp_from__child_process[* as cp from 'child_process]
    vscode_extension_serverManager[vscode-extension/serverManager] --> __as_path_from__path[* as path from 'path]
    vscode_extension_serverManager[vscode-extension/serverManager] --> __as_fs_from__fs[* as fs from 'fs]
    core__analysis__parser[core::analysis::parser] --> std__path[std::path]
    core__analysis__parser[core::analysis::parser] --> tracing__debug[tracing::debug]
    core__analysis__parser[core::analysis::parser] --> crate__errors[crate::errors]
    core__analysis__parser[core::analysis::parser] -- 2 refs --> crate__models[crate::models]
    core__analysis__api__graphql[core::analysis::api::graphql] --> super___[super::*]
    core__analysis__api__graphql[core::analysis::api::graphql] --> regex__Regex[regex::Regex]
    core__analysis__api__graphql[core::analysis::api::graphql] --> std__path[std::path]
    core__analysis__api__graphql[core::analysis::api::graphql] --> tokio__fs[tokio::fs]
    core__analysis__api__graphql[core::analysis::api::graphql] --> walkdir__WalkDir[walkdir::WalkDir]
    core__docs__api_reference[core::docs::api_reference] --> crate__analysis[crate::analysis]
    core__docs__api_reference[core::docs::api_reference] --> std__io[std::io]
    core__analysis__api__webhooks[core::analysis::api::webhooks] --> super___[super::*]
    core__analysis__api__webhooks[core::analysis::api::webhooks] --> regex__Regex[regex::Regex]
    core__analysis__api__webhooks[core::analysis::api::webhooks] --> std__path[std::path]
    core__analysis__api__webhooks[core::analysis::api::webhooks] --> tokio__fs[tokio::fs]
    core__analysis__api__webhooks[core::analysis::api::webhooks] --> walkdir__WalkDir[walkdir::WalkDir]
    core__docs__business_model[core::docs::business_model] --> crate__analysis[crate::analysis]
    vscode_extension_views_modulesTree[vscode-extension/views/modulesTree] --> __as_vscode_from__vscode[* as vscode from 'vscode]
    vscode_extension_views_modulesTree[vscode-extension/views/modulesTree] --> __RiwaqClient__ModuleSummary___from____[{ RiwaqClient, ModuleSummary } from '..]
    core__server__handlers[core::server::handlers] --> axum________extract[axum::{
    extract]
    core__server__handlers[core::server::handlers] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__server__handlers[core::server::handlers] --> std__path[std::path]
    core__server__handlers[core::server::handlers] --> tracing___error__info_[tracing::{error, info}]
    core__server__handlers[core::server::handlers] --> super__state[super::state]
    core__server__handlers[core::server::handlers] --> crate__analysis[crate::analysis]
    core__server__handlers[core::server::handlers] --> crate__docs[crate::docs]
    core__server__handlers[core::server::handlers] --> crate__llm[crate::llm]
    core__analysis__mod[core::analysis::mod] --> api__ApiAnalyzer[api::ApiAnalyzer]
    core__analysis__mod[core::analysis::mod] --> business__BusinessLogicAnalyzer[business::BusinessLogicAnalyzer]
    vscode_extension_panels_askPanel[vscode-extension/panels/askPanel] --> __as_vscode_from__vscode[* as vscode from 'vscode]
    vscode_extension_panels_askPanel[vscode-extension/panels/askPanel] --> __RiwaqClient___from____[{ RiwaqClient } from '..]
    core__models__snapshot[core::models::snapshot] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__models__snapshot[core::models::snapshot] --> std__path[std::path]
    core__models__snapshot[core::models::snapshot] --> super__file[super::file]
    core__models__snapshot[core::models::snapshot] --> super__git[super::git]
    core__models__snapshot[core::models::snapshot] --> super__module[super::module]
    core__server__routes[core::server::routes] --> axum________routing[axum::{
    routing]
    core__server__routes[core::server::routes] --> tower_http__cors[tower_http::cors]
    core__server__routes[core::server::routes] --> super__handlers[super::handlers]
    core__server__routes[core::server::routes] --> super__state[super::state]
    core__analysis__business__rules[core::analysis::business::rules] --> super___[super::*]
    core__analysis__business__rules[core::analysis::business::rules] --> regex__Regex[regex::Regex]
    core__analysis__business__rules[core::analysis::business::rules] --> std__path[std::path]
    core__analysis__business__rules[core::analysis::business::rules] --> tokio__fs[tokio::fs]
    core__analysis__business__rules[core::analysis::business::rules] --> walkdir__WalkDir[walkdir::WalkDir]
    vscode_extension_client[vscode-extension/client] --> axios____AxiosInstance___from__axios[axios, { AxiosInstance } from 'axios]
    core__diagrams__mod[core::diagrams::mod] --> renderer________DiagramRenderer______DiagramRendererConfig______DiagramFormat______RenderedDiagram______render_architecture_diagram______render_er_diagram______render_state_diagram___[renderer::{
    DiagramRenderer,
    DiagramRendererConfig,
    DiagramFormat,
    RenderedDiagram,
    render_architecture_diagram,
    render_er_diagram,
    render_state_diagram,
}]
    core__models__module[core::models::module] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__models__module[core::models::module] --> std__collections[std::collections]
    core__models__module[core::models::module] --> std__path[std::path]
    vscode_extension_views_insightsTree[vscode-extension/views/insightsTree] --> __as_vscode_from__vscode[* as vscode from 'vscode]
    vscode_extension_views_insightsTree[vscode-extension/views/insightsTree] --> __RiwaqClient___from____[{ RiwaqClient } from '..]
    core__docs__mermaid[core::docs::mermaid] -- 2 refs --> crate__models[crate::models]
    core__docs__mermaid[core::docs::mermaid] --> crate__analysis[crate::analysis]
    core__docs__generator[core::docs::generator] --> std__fs[std::fs]
    core__docs__generator[core::docs::generator] --> std__path[std::path]
    core__docs__generator[core::docs::generator] --> tracing__info[tracing::info]
    core__docs__generator[core::docs::generator] --> super__markdown[super::markdown]
    core__docs__generator[core::docs::generator] --> super__mermaid[super::mermaid]
    core__docs__generator[core::docs::generator] --> crate__errors[crate::errors]
    core__docs__generator[core::docs::generator] --> crate__llm[crate::llm]
    core__docs__generator[core::docs::generator] --> crate__models[crate::models]
    core__logging[core::logging] --> tracing__Level[tracing::Level]
    core__logging[core::logging] --> tracing_subscriber________fmt[tracing_subscriber::{
    fmt]
    core__errors[core::errors] --> std__path[std::path]
    core__errors[core::errors] --> thiserror__Error[thiserror::Error]
    vscode_extension_panels_architecturePanel[vscode-extension/panels/architecturePanel] --> __as_vscode_from__vscode[* as vscode from 'vscode]
    vscode_extension_panels_architecturePanel[vscode-extension/panels/architecturePanel] --> __RiwaqClient__CodebaseSnapshot___from____[{ RiwaqClient, CodebaseSnapshot } from '..]
    core__models__function[core::models::function] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__models__function[core::models::function] --> super__file[super::file]
    core__models__file[core::models::file] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__models__file[core::models::file] --> std__path[std::path]
    core__models__file[core::models::file] --> super__function[super::function]
    core__models__git[core::models::git] --> chrono___DateTime__Utc_[chrono::{DateTime, Utc}]
    core__models__git[core::models::git] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__models__git[core::models::git] --> std__collections[std::collections]
    core__models__git[core::models::git] --> std__path[std::path]
    core__docs__markdown[core::docs::markdown] --> std__fmt[std::fmt]
    core__metrics[core::metrics] --> std__collections[std::collections]
    core__metrics[core::metrics] -- 2 refs --> std__sync[std::sync]
    core__metrics[core::metrics] --> std__time[std::time]
    core__metrics[core::metrics] --> serde___Serialize__Deserialize_[serde::{Serialize, Deserialize}]
    core__llm__mod[core::llm::mod] --> client___HttpLlmClient__LLMClient__LLMResponse_[client::{HttpLlmClient, LLMClient, LLMResponse}]
    core__llm__mod[core::llm::mod] --> config__LLMConfig[config::LLMConfig]
    core__llm__mod[core::llm::mod] --> context___ContextBuilder__EnhancedContext__QuestionIntent_[context::{ContextBuilder, EnhancedContext, QuestionIntent}]
    core__docs__mod[core::docs::mod] --> generator___DocGenerator__DocGeneratorConfig__GeneratedDocs_[generator::{DocGenerator, DocGeneratorConfig, GeneratedDocs}]
    core__docs__mod[core::docs::mod] --> api_reference__ApiReferenceGenerator[api_reference::ApiReferenceGenerator]
    core__docs__mod[core::docs::mod] --> business_model__BusinessModelGenerator[business_model::BusinessModelGenerator]
    core__server__mod[core::server::mod] --> routes__create_router[routes::create_router]
    core__server__mod[core::server::mod] --> state__AppState[state::AppState]
    core__analysis__git[core::analysis::git] --> chrono___TimeZone__Utc_[chrono::{TimeZone, Utc}]
    core__analysis__git[core::analysis::git] --> git2___Commit__DiffOptions__Repository__Sort_[git2::{Commit, DiffOptions, Repository, Sort}]
    core__analysis__git[core::analysis::git] --> std__collections[std::collections]
    core__analysis__git[core::analysis::git] --> std__path[std::path]
    core__analysis__git[core::analysis::git] --> tracing__info[tracing::info]
    core__analysis__git[core::analysis::git] --> crate__errors[crate::errors]
    core__analysis__git[core::analysis::git] --> crate__models[crate::models]
    core__analysis__api__rest[core::analysis::api::rest] --> super___[super::*]
    core__analysis__api__rest[core::analysis::api::rest] --> regex__Regex[regex::Regex]
    core__analysis__api__rest[core::analysis::api::rest] --> std__collections[std::collections]
    core__analysis__api__rest[core::analysis::api::rest] --> std__path[std::path]
    core__analysis__api__rest[core::analysis::api::rest] --> tokio__fs[tokio::fs]
    core__analysis__api__rest[core::analysis::api::rest] --> walkdir__WalkDir[walkdir::WalkDir]
    core__analysis__business__integrations[core::analysis::business::integrations] --> super___[super::*]
    core__analysis__business__integrations[core::analysis::business::integrations] --> regex__Regex[regex::Regex]
    core__analysis__business__integrations[core::analysis::business::integrations] --> std__collections[std::collections]
    core__analysis__business__integrations[core::analysis::business::integrations] --> std__path[std::path]
    core__analysis__business__integrations[core::analysis::business::integrations] --> tokio__fs[tokio::fs]
    core__analysis__business__integrations[core::analysis::business::integrations] --> walkdir__WalkDir[walkdir::WalkDir]
    core__analysis__api__mod[core::analysis::api::mod] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__analysis__api__mod[core::analysis::api::mod] --> std__path[std::path]
    core__analysis__api__mod[core::analysis::api::mod] --> rest__RestAnalyzer[rest::RestAnalyzer]
    core__analysis__api__mod[core::analysis::api::mod] --> grpc__GrpcAnalyzer[grpc::GrpcAnalyzer]
    core__analysis__api__mod[core::analysis::api::mod] --> graphql__GraphQLAnalyzer[graphql::GraphQLAnalyzer]
    core__analysis__api__mod[core::analysis::api::mod] --> webhooks__WebhookAnalyzer[webhooks::WebhookAnalyzer]
    vscode_extension_extension[vscode-extension/extension] --> __as_vscode_from__vscode[* as vscode from 'vscode]
    vscode_extension_extension[vscode-extension/extension] --> __as_path_from__path[* as path from 'path]
    vscode_extension_extension[vscode-extension/extension] --> __RiwaqClient___from___[{ RiwaqClient } from '.]
    vscode_extension_extension[vscode-extension/extension] --> __ServerManager___from___[{ ServerManager } from '.]
    vscode_extension_extension[vscode-extension/extension] --> __ArchitectureTreeProvider___from___[{ ArchitectureTreeProvider } from '.]
    vscode_extension_extension[vscode-extension/extension] --> __ModulesTreeProvider___from___[{ ModulesTreeProvider } from '.]
    vscode_extension_extension[vscode-extension/extension] --> __InsightsTreeProvider___from___[{ InsightsTreeProvider } from '.]
    vscode_extension_extension[vscode-extension/extension] --> __AskPanel___from___[{ AskPanel } from '.]
    vscode_extension_extension[vscode-extension/extension] --> __ArchitecturePanel___from___[{ ArchitecturePanel } from '.]
    core__llm__config[core::llm::config] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__llm__context[core::llm::context] --> crate__models[crate::models]
    core__llm__context[core::llm::context] -- 2 refs --> crate__analysis[crate::analysis]
    core__llm__context[core::llm::context] --> std__collections[std::collections]
    core__server__state[core::server::state] --> std__collections[std::collections]
    core__server__state[core::server::state] --> std__sync[std::sync]
    core__server__state[core::server::state] --> tokio__sync[tokio::sync]
    core__server__state[core::server::state] --> crate__llm[crate::llm]
    core__server__state[core::server::state] --> crate__models[crate::models]
    core__analysis__analyzer[core::analysis::analyzer] --> std__collections[std::collections]
    core__analysis__analyzer[core::analysis::analyzer] --> std__path[std::path]
    core__analysis__analyzer[core::analysis::analyzer] --> std__time[std::time]
    core__analysis__analyzer[core::analysis::analyzer] --> tracing___info__warn_[tracing::{info, warn}]
    core__analysis__analyzer[core::analysis::analyzer] -- 3 refs --> crate__analysis[crate::analysis]
    core__analysis__analyzer[core::analysis::analyzer] --> crate__errors[crate::errors]
    core__analysis__analyzer[core::analysis::analyzer] --> crate__logging[crate::logging]
    core__analysis__analyzer[core::analysis::analyzer] -- 3 refs --> crate__models[crate::models]
```

