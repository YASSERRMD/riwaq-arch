//! HTTP request handlers.

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{error, info, warn};

use super::state::AppState;
use crate::analysis::analyzer::CodebaseAnalyzer;
use crate::diagrams::DiagramRenderer;
use crate::docs::{DocGenerator, DocGeneratorConfig};
use crate::llm::LLMClient;

// ============================================================================
// Request/Response types
// ============================================================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct AnalyzeResponse {
    pub success: bool,
    pub snapshot: Option<crate::models::snapshot::CodebaseSnapshot>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocgenRequest {
    pub path: String,
    pub output: String,
    #[serde(default)]
    pub skip_llm: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocgenResponse {
    pub success: bool,
    pub file_count: usize,
    pub output_dir: String,
    pub generation_time_ms: u64,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AskRequest {
    pub question: String,
    pub project_path: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AskResponse {
    pub success: bool,
    pub answer: Option<String>,
    pub file_refs: Vec<FileRef>,
    pub confidence: f32,
    pub error: Option<String>,
    /// SVG diagrams extracted from the answer (rendered images, not text)
    pub diagrams: Vec<DiagramData>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagramData {
    pub name: String,
    pub svg: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRef {
    pub path: String,
    pub line: Option<usize>,
    pub relevance: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_secs: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagramResponse {
    pub success: bool,
    /// SVG content (rendered image, not mermaid text)
    pub svg: String,
    pub format: String,
}

/// Request to generate a single document type
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleDocRequest {
    pub path: String,
    pub output: String,
    /// Document type: srs, user_stories, brd, architecture, code_docs, api_specs, release_notes, user_guides, runbook
    pub doc_type: String,
}

/// Response for single document generation
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleDocResponse {
    pub success: bool,
    pub doc_type: String,
    pub file_path: Option<String>,
    pub generation_time_ms: u64,
    pub error: Option<String>,
}

/// List of available document types
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocTypesResponse {
    pub doc_types: Vec<DocTypeInfo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocTypeInfo {
    pub id: String,
    pub name: String,
    pub description: String,
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

/// List available document types.
pub async fn list_doc_types() -> impl IntoResponse {
    let doc_types = vec![
        DocTypeInfo {
            id: "srs".to_string(),
            name: "Software Requirements Specification".to_string(),
            description: "Functional and non-functional requirements, system interfaces, data requirements".to_string(),
        },
        DocTypeInfo {
            id: "user_stories".to_string(),
            name: "User Stories".to_string(),
            description: "User personas, epics, detailed stories with acceptance criteria".to_string(),
        },
        DocTypeInfo {
            id: "brd".to_string(),
            name: "Business Requirements Document".to_string(),
            description: "Business overview, objectives, stakeholders, ROI analysis".to_string(),
        },
        DocTypeInfo {
            id: "architecture".to_string(),
            name: "Architecture Documentation".to_string(),
            description: "System overview, components, data, integration, deployment architecture".to_string(),
        },
        DocTypeInfo {
            id: "code_docs".to_string(),
            name: "Code Documentation".to_string(),
            description: "Getting started, module docs, coding patterns, testing guide".to_string(),
        },
        DocTypeInfo {
            id: "api_specs".to_string(),
            name: "API Specifications".to_string(),
            description: "API overview, REST endpoints, gRPC/GraphQL, error handling".to_string(),
        },
        DocTypeInfo {
            id: "release_notes".to_string(),
            name: "Release Notes".to_string(),
            description: "Current release features, breaking changes, roadmap".to_string(),
        },
        DocTypeInfo {
            id: "user_guides".to_string(),
            name: "User Guides".to_string(),
            description: "Quick start, features guide, troubleshooting, FAQ".to_string(),
        },
        DocTypeInfo {
            id: "runbook".to_string(),
            name: "Operations Runbook".to_string(),
            description: "Deployment, monitoring, incident response, backup, maintenance".to_string(),
        },
    ];
    
    (StatusCode::OK, Json(DocTypesResponse { doc_types }))
}

/// Generate a single document type.
pub async fn generate_single_doc(
    State(state): State<AppState>,
    Json(req): Json<SingleDocRequest>,
) -> impl IntoResponse {
    use crate::docs::{AgenticDocGenerator, DocumentType};
    use std::time::Instant;
    
    info!(path = %req.path, doc_type = %req.doc_type, "Generating single document");
    
    let start = Instant::now();
    let path = PathBuf::from(&req.path);
    
    // Parse document type
    let doc_type = match req.doc_type.as_str() {
        "srs" => DocumentType::SRS,
        "user_stories" => DocumentType::UserStories,
        "brd" => DocumentType::BRD,
        "architecture" => DocumentType::ArchitectureDiagram,
        "code_docs" => DocumentType::CodeDocs,
        "api_specs" => DocumentType::APISpecs,
        "release_notes" => DocumentType::ReleaseNotes,
        "user_guides" => DocumentType::UserGuides,
        "runbook" => DocumentType::Runbook,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(SingleDocResponse {
                    success: false,
                    doc_type: req.doc_type,
                    file_path: None,
                    generation_time_ms: 0,
                    error: Some("Invalid document type. Use: srs, user_stories, brd, architecture, code_docs, api_specs, release_notes, user_guides, runbook".to_string()),
                }),
            );
        }
    };
    
    // Get or create snapshot
    let snapshot = if let Some(s) = state.get_snapshot(&req.path).await {
        s
    } else {
        let analyzer = CodebaseAnalyzer::new(&path);
        match analyzer.analyze().await {
            Ok(s) => {
                state.set_snapshot(req.path.clone(), s.clone()).await;
                s
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(SingleDocResponse {
                        success: false,
                        doc_type: req.doc_type,
                        file_path: None,
                        generation_time_ms: start.elapsed().as_millis() as u64,
                        error: Some(format!("Analysis failed: {}", e)),
                    }),
                );
            }
        }
    };
    
    // Setup output directory
    let output_path = if req.output.starts_with('/') {
        PathBuf::from(&req.output)
    } else {
        path.join(&req.output)
    };
    
    if let Err(e) = std::fs::create_dir_all(&output_path) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SingleDocResponse {
                success: false,
                doc_type: req.doc_type,
                file_path: None,
                generation_time_ms: start.elapsed().as_millis() as u64,
                error: Some(format!("Failed to create output directory: {}", e)),
            }),
        );
    }
    
    // Get project name
    let project_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Project")
        .to_string();
    
    // Create generator using internal HttpLlmClient with RIWAQ_LLM_API_KEY
    let generator = AgenticDocGenerator::new(
        output_path.clone(),
        project_name,
    );
    
    // Build context once
    let context = generator.build_context_summary(&snapshot);
    
    // Generate the single document
    match generator.generate_document(doc_type, &snapshot, &context).await {
        Ok(file_path) => {
            info!(path = %file_path.display(), "Document generated successfully");
            (
                StatusCode::OK,
                Json(SingleDocResponse {
                    success: true,
                    doc_type: req.doc_type,
                    file_path: Some(file_path.to_string_lossy().to_string()),
                    generation_time_ms: start.elapsed().as_millis() as u64,
                    error: None,
                }),
            )
        }
        Err(e) => {
            error!(error = %e, "Single document generation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SingleDocResponse {
                    success: false,
                    doc_type: req.doc_type,
                    file_path: None,
                    generation_time_ms: start.elapsed().as_millis() as u64,
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
                diagrams: vec![],
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
                    diagrams: vec![],
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

            // Extract and render any Mermaid diagrams to SVG
            let (clean_answer, diagrams) = extract_and_render_diagrams(&result.answer);

            (
                StatusCode::OK,
                Json(AskResponse {
                    success: true,
                    answer: Some(clean_answer),
                    file_refs,
                    confidence: result.confidence,
                    error: None,
                    diagrams,
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
                    diagrams: vec![],
                }),
            )
        }
    }
}

/// Extract Mermaid code blocks from text and render them to SVG images
fn extract_and_render_diagrams(text: &str) -> (String, Vec<DiagramData>) {
    let mut diagrams = Vec::new();
    let mut clean_text = text.to_string();
    
    // Pattern 1: Match ```mermaid ... ``` blocks (fenced)
    let fenced_regex = Regex::new(r"```mermaid\s*([\s\S]*?)```").unwrap();
    
    // Pattern 2: Match unfenced mermaid diagram blocks (graph, flowchart, sequenceDiagram, etc.)
    // This catches LLM responses that don't use proper code fences
    let unfenced_regex = Regex::new(
        r"(?m)^((?:graph|flowchart|sequenceDiagram|classDiagram|stateDiagram|erDiagram|gantt|pie|journey)\s+(?:TD|TB|BT|RL|LR)?[\s\S]*?)(?:\n\n|\z)"
    ).unwrap();

    let mut diagram_count = 0;

    // First, handle fenced mermaid blocks
    for cap in fenced_regex.captures_iter(text) {
        let full_match = cap.get(0).unwrap().as_str();
        let mermaid_code = cap.get(1).unwrap().as_str().trim();
        
        if let Ok(svg) = render_mermaid_to_svg(mermaid_code) {
            diagram_count += 1;
            let name = format!("diagram_{}", diagram_count);
            
            // Remove the mermaid block from text since we'll show SVG
            clean_text = clean_text.replace(full_match, "");
            
            diagrams.push(DiagramData {
                name,
                svg,
            });
        }
    }

    // Then, handle unfenced mermaid diagrams (raw code in response)
    for cap in unfenced_regex.captures_iter(&clean_text.clone()) {
        let full_match = cap.get(0).unwrap().as_str();
        let mermaid_code = cap.get(1).unwrap().as_str().trim();
        
        // Skip if already processed or too short
        if mermaid_code.len() < 10 {
            continue;
        }

        if let Ok(svg) = render_mermaid_to_svg(mermaid_code) {
            diagram_count += 1;
            let name = format!("diagram_{}", diagram_count);
            
            // Remove the mermaid code from text
            clean_text = clean_text.replace(full_match, "");
            
            diagrams.push(DiagramData {
                name,
                svg,
            });
        }
    }
    
    // Clean up extra newlines
    while clean_text.contains("\n\n\n") {
        clean_text = clean_text.replace("\n\n\n", "\n\n");
    }
    
    (clean_text.trim().to_string(), diagrams)
}

/// Render Mermaid code to SVG string
fn render_mermaid_to_svg(mermaid_code: &str) -> Result<String, String> {
    let config = crate::diagrams::DiagramRendererConfig::default();
    let renderer = DiagramRenderer::new(config)?;
    renderer.render_to_svg(mermaid_code)
}

/// Get architecture diagram as SVG image.
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
                svg: String::new(),
                format: "svg".to_string(),
            }),
        );
    };

    // Generate Mermaid code and render to SVG
    let mermaid_code = crate::docs::mermaid::generate_architecture_diagram(snapshot);
    
    match render_mermaid_to_svg(&mermaid_code) {
        Ok(svg) => (
            StatusCode::OK,
            Json(DiagramResponse {
                success: true,
                svg,
                format: "svg".to_string(),
            }),
        ),
        Err(e) => {
            error!(error = %e, "Failed to render architecture diagram");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DiagramResponse {
                    success: false,
                    svg: String::new(),
                    format: "svg".to_string(),
                }),
            )
        }
    }
}

/// Get dependency diagram as SVG image.
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
                svg: String::new(),
                format: "svg".to_string(),
            }),
        );
    };

    // Generate Mermaid code and render to SVG
    let mermaid_code = crate::docs::mermaid::generate_dependency_diagram(&snapshot.dependency_graph);
    
    match render_mermaid_to_svg(&mermaid_code) {
        Ok(svg) => (
            StatusCode::OK,
            Json(DiagramResponse {
                success: true,
                svg,
                format: "svg".to_string(),
            }),
        ),
        Err(e) => {
            error!(error = %e, "Failed to render dependency diagram");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DiagramResponse {
                    success: false,
                    svg: String::new(),
                    format: "svg".to_string(),
                }),
            )
        }
    }
}

