# Dependencies & Risk Analysis

## Technology Stack

| Language | Files | Lines | Percentage |
| --- | --- | --- | --- |
| TypeScript | 8 | 1850 | 19.4% |
| Rust | 27 | 7675 | 80.6% |

## Dependency Graph

```mermaid
graph TD
    core__docs__markdown[core::docs::markdown] --> std__fmt[std::fmt]
    vscode_extension_panels_architecturePanel[vscode-extension/panels/architecturePanel] --> __as_vscode_from__vscode[* as vscode from 'vscode]
    vscode_extension_panels_architecturePanel[vscode-extension/panels/architecturePanel] --> __RiwaqClient__CodebaseSnapshot___from____[{ RiwaqClient, CodebaseSnapshot } from '..]
    core__models__mod[core::models::mod] --> file__FileSummary[file::FileSummary]
    core__models__mod[core::models::mod] --> function__FunctionSummary[function::FunctionSummary]
    core__models__mod[core::models::mod] --> git__GitInsights[git::GitInsights]
    core__models__mod[core::models::mod] --> module__ModuleSummary[module::ModuleSummary]
    core__models__mod[core::models::mod] --> snapshot__CodebaseSnapshot[snapshot::CodebaseSnapshot]
    core__analysis__fs_scanner[core::analysis::fs_scanner] --> ignore__WalkBuilder[ignore::WalkBuilder]
    core__analysis__fs_scanner[core::analysis::fs_scanner] --> std__collections[std::collections]
    core__analysis__fs_scanner[core::analysis::fs_scanner] --> std__path[std::path]
    core__analysis__fs_scanner[core::analysis::fs_scanner] --> tracing___debug__info__warn_[tracing::{debug, info, warn}]
    core__analysis__fs_scanner[core::analysis::fs_scanner] --> crate__errors[crate::errors]
    core__analysis__fs_scanner[core::analysis::fs_scanner] --> crate__models[crate::models]
    core__server__routes[core::server::routes] --> axum________routing[axum::{
    routing]
    core__server__routes[core::server::routes] --> tower_http__cors[tower_http::cors]
    core__server__routes[core::server::routes] --> super__handlers[super::handlers]
    core__server__routes[core::server::routes] --> super__state[super::state]
    vscode_extension_views_insightsTree[vscode-extension/views/insightsTree] --> __as_vscode_from__vscode[* as vscode from 'vscode]
    vscode_extension_views_insightsTree[vscode-extension/views/insightsTree] --> __RiwaqClient___from____[{ RiwaqClient } from '..]
    core__models__module[core::models::module] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__models__module[core::models::module] --> std__collections[std::collections]
    core__models__module[core::models::module] --> std__path[std::path]
    vscode_extension_panels_askPanel[vscode-extension/panels/askPanel] --> __as_vscode_from__vscode[* as vscode from 'vscode]
    vscode_extension_panels_askPanel[vscode-extension/panels/askPanel] --> __RiwaqClient___from____[{ RiwaqClient } from '..]
    core__analysis__analyzer[core::analysis::analyzer] --> std__collections[std::collections]
    core__analysis__analyzer[core::analysis::analyzer] --> std__path[std::path]
    core__analysis__analyzer[core::analysis::analyzer] --> std__time[std::time]
    core__analysis__analyzer[core::analysis::analyzer] --> tracing___info__warn_[tracing::{info, warn}]
    core__analysis__analyzer[core::analysis::analyzer] -- 3 refs --> crate__analysis[crate::analysis]
    core__analysis__analyzer[core::analysis::analyzer] --> crate__errors[crate::errors]
    core__analysis__analyzer[core::analysis::analyzer] --> crate__logging[crate::logging]
    core__analysis__analyzer[core::analysis::analyzer] -- 3 refs --> crate__models[crate::models]
    core__server__mod[core::server::mod] --> routes__create_router[routes::create_router]
    core__server__mod[core::server::mod] --> state__AppState[state::AppState]
    core__server__handlers[core::server::handlers] --> axum________extract[axum::{
    extract]
    core__server__handlers[core::server::handlers] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__server__handlers[core::server::handlers] --> std__path[std::path]
    core__server__handlers[core::server::handlers] --> tracing___error__info_[tracing::{error, info}]
    core__server__handlers[core::server::handlers] --> super__state[super::state]
    core__server__handlers[core::server::handlers] --> crate__analysis[crate::analysis]
    core__server__handlers[core::server::handlers] --> crate__docs[crate::docs]
    core__server__handlers[core::server::handlers] --> crate__llm[crate::llm]
    vscode_extension_client[vscode-extension/client] --> axios____AxiosInstance___from__axios[axios, { AxiosInstance } from 'axios]
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
    core__main[core::main] --> clap___Parser__Subcommand_[clap::{Parser, Subcommand}]
    core__main[core::main] --> riwaq_core__logging[riwaq_core::logging]
    core__main[core::main] --> std__path[std::path]
    core__main[core::main] --> tracing__info[tracing::info]
    core__models__function[core::models::function] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__models__function[core::models::function] --> super__file[super::file]
    core__logging[core::logging] --> tracing__Level[tracing::Level]
    core__logging[core::logging] --> tracing_subscriber________fmt[tracing_subscriber::{
    fmt]
    core__llm__mod[core::llm::mod] --> client___HttpLlmClient__LLMClient__LLMResponse_[client::{HttpLlmClient, LLMClient, LLMResponse}]
    core__llm__mod[core::llm::mod] --> config__LLMConfig[config::LLMConfig]
    core__analysis__git[core::analysis::git] --> chrono___TimeZone__Utc_[chrono::{TimeZone, Utc}]
    core__analysis__git[core::analysis::git] --> git2___Commit__DiffOptions__Repository__Sort_[git2::{Commit, DiffOptions, Repository, Sort}]
    core__analysis__git[core::analysis::git] --> std__collections[std::collections]
    core__analysis__git[core::analysis::git] --> std__path[std::path]
    core__analysis__git[core::analysis::git] --> tracing__info[tracing::info]
    core__analysis__git[core::analysis::git] --> crate__errors[crate::errors]
    core__analysis__git[core::analysis::git] --> crate__models[crate::models]
    core__analysis__parser[core::analysis::parser] --> std__path[std::path]
    core__analysis__parser[core::analysis::parser] --> tracing__debug[tracing::debug]
    core__analysis__parser[core::analysis::parser] --> crate__errors[crate::errors]
    core__analysis__parser[core::analysis::parser] -- 2 refs --> crate__models[crate::models]
    vscode_extension_serverManager[vscode-extension/serverManager] --> __as_vscode_from__vscode[* as vscode from 'vscode]
    vscode_extension_serverManager[vscode-extension/serverManager] --> __as_cp_from__child_process[* as cp from 'child_process]
    vscode_extension_serverManager[vscode-extension/serverManager] --> __as_path_from__path[* as path from 'path]
    vscode_extension_serverManager[vscode-extension/serverManager] --> __as_fs_from__fs[* as fs from 'fs]
    core__llm__client[core::llm::client] --> async_trait__async_trait[async_trait::async_trait]
    core__llm__client[core::llm::client] --> reqwest__Client[reqwest::Client]
    core__llm__client[core::llm::client] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__llm__client[core::llm::client] --> std__time[std::time]
    core__llm__client[core::llm::client] --> tracing___debug__info__warn_[tracing::{debug, info, warn}]
    core__llm__client[core::llm::client] --> super__config[super::config]
    core__llm__client[core::llm::client] --> super__prompts[super::prompts]
    core__llm__client[core::llm::client] --> crate__errors[crate::errors]
    core__llm__client[core::llm::client] --> crate__models[crate::models]
    core__docs__mod[core::docs::mod] --> generator___DocGenerator__DocGeneratorConfig__GeneratedDocs_[generator::{DocGenerator, DocGeneratorConfig, GeneratedDocs}]
    core__server__state[core::server::state] --> std__collections[std::collections]
    core__server__state[core::server::state] --> std__sync[std::sync]
    core__server__state[core::server::state] --> tokio__sync[tokio::sync]
    core__server__state[core::server::state] --> crate__llm[crate::llm]
    core__server__state[core::server::state] --> crate__models[crate::models]
    core__models__snapshot[core::models::snapshot] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__models__snapshot[core::models::snapshot] --> std__path[std::path]
    core__models__snapshot[core::models::snapshot] --> super__file[super::file]
    core__models__snapshot[core::models::snapshot] --> super__git[super::git]
    core__models__snapshot[core::models::snapshot] --> super__module[super::module]
    core__docs__generator[core::docs::generator] --> std__fs[std::fs]
    core__docs__generator[core::docs::generator] --> std__path[std::path]
    core__docs__generator[core::docs::generator] --> tracing__info[tracing::info]
    core__docs__generator[core::docs::generator] --> super__markdown[super::markdown]
    core__docs__generator[core::docs::generator] --> super__mermaid[super::mermaid]
    core__docs__generator[core::docs::generator] --> crate__errors[crate::errors]
    core__docs__generator[core::docs::generator] --> crate__llm[crate::llm]
    core__docs__generator[core::docs::generator] --> crate__models[crate::models]
    vscode_extension_views_architectureTree[vscode-extension/views/architectureTree] --> __as_vscode_from__vscode[* as vscode from 'vscode]
    vscode_extension_views_architectureTree[vscode-extension/views/architectureTree] --> __RiwaqClient___from____[{ RiwaqClient } from '..]
    core__docs__mermaid[core::docs::mermaid] -- 2 refs --> crate__models[crate::models]
    core__models__git[core::models::git] --> chrono___DateTime__Utc_[chrono::{DateTime, Utc}]
    core__models__git[core::models::git] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__models__git[core::models::git] --> std__collections[std::collections]
    core__models__git[core::models::git] --> std__path[std::path]
    core__errors[core::errors] --> std__path[std::path]
    core__errors[core::errors] --> thiserror__Error[thiserror::Error]
    core__models__file[core::models::file] --> serde___Deserialize__Serialize_[serde::{Deserialize, Serialize}]
    core__models__file[core::models::file] --> std__path[std::path]
    core__models__file[core::models::file] --> super__function[super::function]
    core__lib[core::lib] --> analysis__analyzer[analysis::analyzer]
    core__lib[core::lib] --> docs___DocGenerator__DocGeneratorConfig__GeneratedDocs_[docs::{DocGenerator, DocGeneratorConfig, GeneratedDocs}]
    core__lib[core::lib] --> errors___RiwaqError__Result_[errors::{RiwaqError, Result}]
    core__lib[core::lib] --> llm___HttpLlmClient__LLMClient__LLMConfig__LLMResponse_[llm::{HttpLlmClient, LLMClient, LLMConfig, LLMResponse}]
    core__lib[core::lib] --> models__snapshot[models::snapshot]
    core__lib[core::lib] --> server___create_router__AppState_[server::{create_router, AppState}]
    vscode_extension_views_modulesTree[vscode-extension/views/modulesTree] --> __as_vscode_from__vscode[* as vscode from 'vscode]
    vscode_extension_views_modulesTree[vscode-extension/views/modulesTree] --> __RiwaqClient__ModuleSummary___from____[{ RiwaqClient, ModuleSummary } from '..]
```

## Circular Dependencies

✅ No circular dependencies detected.

## Repository Insights

| Metric | Value |
| --- | --- |
| Repository | riwaq-arch |
| Total Commits | 37 |
| Default Branch | main |

## Detected Domains

Domains inferred from commit messages and file groupings:

| Domain | Files | Activity |
| --- | --- | --- |
| infra | 19 | 47% |
| config | 48 | 65% |
| ui | 52 | 47% |
| database | 11 | 18% |
| auth | 1 | 6% |
| api | 21 | 35% |
| testing | 31 | 41% |
| docs | 41 | 100% |

