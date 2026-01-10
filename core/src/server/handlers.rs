//! HTTP request handlers.

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{error, info};

use super::state::AppState;
use crate::analysis::analyzer::CodebaseAnalyzer;
use crate::docs::{DocGenerator, DocGeneratorConfig};
use crate::llm::LLMClient;

// ============================================================================
// Request/Response types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub path: String,
    #[serde(default = "default_max_commits")]
    pub max_commits: usize,
    #[serde(default)]
    pub include_tests: bool,
}

fn default_max_commits() -> usize {
    500
}

#[derive(Debug, Serialize)]
pub struct AnalyzeResponse {
    pub success: bool,
    pub snapshot: Option<crate::models::snapshot::CodebaseSnapshot>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DocgenRequest {
    pub path: String,
    pub output: String,
    #[serde(default)]
    pub skip_llm: bool,
}

#[derive(Debug, Serialize)]
pub struct DocgenResponse {
    pub success: bool,
    pub file_count: usize,
    pub output_dir: String,
    pub generation_time_ms: u64,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AskRequest {
    pub question: String,
    pub project_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AskResponse {
    pub success: bool,
    pub answer: Option<String>,
    pub file_refs: Vec<FileRef>,
    pub confidence: f32,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FileRef {
    pub path: String,
    pub line: Option<usize>,
    pub relevance: String,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_secs: u64,
}

#[derive(Debug, Serialize)]
pub struct DiagramResponse {
    pub success: bool,
    pub diagram: String,
    pub format: String,
}

// ============================================================================
// Handlers
// ============================================================================

/// Health check endpoint.
pub async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: crate::VERSION.to_string(),
        uptime_secs: 0, // TODO: Track actual uptime
    })
}

/// List analyzed projects.
pub async fn list_projects(State(state): State<AppState>) -> impl IntoResponse {
    let projects = state.list_projects().await;
    Json(projects)
}

/// Analyze a codebase.
pub async fn analyze(
    State(state): State<AppState>,
    Json(req): Json<AnalyzeRequest>,
) -> impl IntoResponse {
    info!(path = %req.path, max_commits = req.max_commits, "Analyzing codebase");

    let path = PathBuf::from(&req.path);
    if !path.exists() {
        return (
            StatusCode::BAD_REQUEST,
            Json(AnalyzeResponse {
                success: false,
                snapshot: None,
                error: Some(format!("Path does not exist: {}", req.path)),
            }),
        );
    }

    let analyzer = CodebaseAnalyzer::new(&path)
        .with_max_commits(req.max_commits)
        .with_include_tests(req.include_tests);

    match analyzer.analyze().await {
        Ok(snapshot) => {
            // Cache the snapshot
            state.set_snapshot(req.path.clone(), snapshot.clone()).await;

            info!(
                files = snapshot.statistics.total_files,
                modules = snapshot.statistics.total_modules,
                "Analysis complete"
            );

            (
                StatusCode::OK,
                Json(AnalyzeResponse {
                    success: true,
                    snapshot: Some(snapshot),
                    error: None,
                }),
            )
        }
        Err(e) => {
            error!(error = %e, "Analysis failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AnalyzeResponse {
                    success: false,
                    snapshot: None,
                    error: Some(e.to_string()),
                }),
            )
        }
    }
}

/// Generate documentation.
pub async fn generate_docs(
    State(state): State<AppState>,
    Json(req): Json<DocgenRequest>,
) -> impl IntoResponse {
    info!(path = %req.path, output = %req.output, "Generating documentation");

    let path = PathBuf::from(&req.path);

    // First, ensure we have a snapshot
    let snapshot = if let Some(s) = state.get_snapshot(&req.path).await {
        s
    } else {
        // Analyze first
        let analyzer = CodebaseAnalyzer::new(&path);
        match analyzer.analyze().await {
            Ok(s) => {
                state.set_snapshot(req.path.clone(), s.clone()).await;
                s
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(DocgenResponse {
                        success: false,
                        file_count: 0,
                        output_dir: req.output,
                        generation_time_ms: 0,
                        error: Some(e.to_string()),
                    }),
                );
            }
        }
    };

    // Generate docs
    let output_path = if req.output.starts_with('/') {
        PathBuf::from(&req.output)
    } else {
        path.join(&req.output)
    };

    let config = DocGeneratorConfig::new(&output_path).with_llm(!req.skip_llm);

    let generator = DocGenerator::new(config);

    match generator.generate(&snapshot).await {
        Ok(result) => {
            info!(files = result.files.len(), "Documentation generated");
            (
                StatusCode::OK,
                Json(DocgenResponse {
                    success: true,
                    file_count: result.files.len(),
                    output_dir: result.output_dir.to_string_lossy().to_string(),
                    generation_time_ms: result.generation_time_ms,
                    error: None,
                }),
            )
        }
        Err(e) => {
            error!(error = %e, "Doc generation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DocgenResponse {
                    success: false,
                    file_count: 0,
                    output_dir: req.output,
                    generation_time_ms: 0,
                    error: Some(e.to_string()),
                }),
            )
        }
    }
}

/// Answer a question about the codebase.
pub async fn ask_question(
    State(state): State<AppState>,
    Json(req): Json<AskRequest>,
) -> impl IntoResponse {
    info!(question = %req.question, "Answering question");

    // Get the snapshot
    let snapshots = state.snapshots.read().await;
    let snapshot = if let Some(path) = &req.project_path {
        snapshots.get(path)
    } else {
        // Use the most recent snapshot
        snapshots.values().next()
    };

    let Some(snapshot) = snapshot.cloned() else {
        return (
            StatusCode::BAD_REQUEST,
            Json(AskResponse {
                success: false,
                answer: None,
                file_refs: vec![],
                confidence: 0.0,
                error: Some("No analysis available. Please analyze a workspace first.".to_string()),
            }),
        );
    };
    drop(snapshots);

    // Get LLM client
    let client = match state.get_llm_client().await {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AskResponse {
                    success: false,
                    answer: None,
                    file_refs: vec![],
                    confidence: 0.0,
                    error: Some(format!("LLM client error: {}", e)),
                }),
            );
        }
    };

    match client.answer_question(&snapshot, &req.question).await {
        Ok(result) => {
            let file_refs: Vec<FileRef> = result
                .file_refs
                .into_iter()
                .map(|r| FileRef {
                    path: r.path,
                    line: r.line,
                    relevance: r.relevance,
                })
                .collect();

            (
                StatusCode::OK,
                Json(AskResponse {
                    success: true,
                    answer: Some(result.answer),
                    file_refs,
                    confidence: result.confidence,
                    error: None,
                }),
            )
        }
        Err(e) => {
            error!(error = %e, "Question answering failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AskResponse {
                    success: false,
                    answer: None,
                    file_refs: vec![],
                    confidence: 0.0,
                    error: Some(e.to_string()),
                }),
            )
        }
    }
}

/// Get architecture diagram.
pub async fn get_architecture_diagram(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let snapshots = state.snapshots.read().await;
    let snapshot = snapshots.values().next();

    let Some(snapshot) = snapshot else {
        return (
            StatusCode::BAD_REQUEST,
            Json(DiagramResponse {
                success: false,
                diagram: String::new(),
                format: "mermaid".to_string(),
            }),
        );
    };

    let diagram = crate::docs::mermaid::generate_architecture_diagram(snapshot);

    (
        StatusCode::OK,
        Json(DiagramResponse {
            success: true,
            diagram,
            format: "mermaid".to_string(),
        }),
    )
}

/// Get dependency diagram.
pub async fn get_dependency_diagram(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let snapshots = state.snapshots.read().await;
    let snapshot = snapshots.values().next();

    let Some(snapshot) = snapshot else {
        return (
            StatusCode::BAD_REQUEST,
            Json(DiagramResponse {
                success: false,
                diagram: String::new(),
                format: "mermaid".to_string(),
            }),
        );
    };

    let diagram = crate::docs::mermaid::generate_dependency_diagram(&snapshot.dependency_graph);

    (
        StatusCode::OK,
        Json(DiagramResponse {
            success: true,
            diagram,
            format: "mermaid".to_string(),
        }),
    )
}
