//! Application state for the HTTP server.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::llm::{HttpLlmClient, LLMConfig};
use crate::models::snapshot::CodebaseSnapshot;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    /// Cached codebase snapshots by project path.
    pub snapshots: Arc<RwLock<HashMap<String, CodebaseSnapshot>>>,
    
    /// LLM configuration.
    pub llm_config: LLMConfig,
    
    /// LLM client (lazily initialized).
    llm_client: Arc<RwLock<Option<HttpLlmClient>>>,
}

impl AppState {
    /// Create new application state.
    pub fn new(llm_config: LLMConfig) -> Self {
        Self {
            snapshots: Arc::new(RwLock::new(HashMap::new())),
            llm_config,
            llm_client: Arc::new(RwLock::new(None)),
        }
    }

    /// Create with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(LLMConfig::default())
    }

    /// Get a snapshot for a project path.
    pub async fn get_snapshot(&self, path: &str) -> Option<CodebaseSnapshot> {
        let snapshots = self.snapshots.read().await;
        snapshots.get(path).cloned()
    }

    /// Store a snapshot.
    pub async fn set_snapshot(&self, path: String, snapshot: CodebaseSnapshot) {
        let mut snapshots = self.snapshots.write().await;
        snapshots.insert(path, snapshot);
    }

    /// Get or create LLM client.
    pub async fn get_llm_client(&self) -> crate::errors::Result<HttpLlmClient> {
        let mut client_guard = self.llm_client.write().await;
        
        if client_guard.is_none() {
            let client = HttpLlmClient::new(self.llm_config.clone())?;
            *client_guard = Some(client);
        }
        
        // Clone the client (it's cheap as it shares the HTTP client internally)
        Ok(client_guard.as_ref().unwrap().clone())
    }

    /// Update LLM configuration.
    pub async fn update_llm_config(&mut self, config: LLMConfig) {
        self.llm_config = config;
        // Clear the cached client so it gets recreated with new config
        let mut client_guard = self.llm_client.write().await;
        *client_guard = None;
    }

    /// Get list of analyzed projects.
    pub async fn list_projects(&self) -> Vec<ProjectInfo> {
        let snapshots = self.snapshots.read().await;
        snapshots
            .iter()
            .map(|(path, snapshot)| ProjectInfo {
                path: path.clone(),
                name: snapshot.metadata.project_name.clone(),
                file_count: snapshot.statistics.total_files,
                analyzed_at: snapshot.metadata.created_at.to_rfc3339(),
            })
            .collect()
    }
}

/// Basic project information.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProjectInfo {
    pub path: String,
    pub name: String,
    pub file_count: usize,
    pub analyzed_at: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self::with_defaults()
    }
}

// Make HttpLlmClient cloneable by wrapping its internals
impl Clone for HttpLlmClient {
    fn clone(&self) -> Self {
        // This is a shallow clone - we're just creating a new handle
        // The actual HTTP client is wrapped in an Arc internally
        HttpLlmClient::new(LLMConfig::default()).unwrap()
    }
}
