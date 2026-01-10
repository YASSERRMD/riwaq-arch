//! Git-related data models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Insights extracted from git history.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GitInsights {
    /// Repository information.
    pub repository: Option<RepositoryInfo>,

    /// Commit history (most recent first).
    pub commits: Vec<CommitInfo>,

    /// File churn analysis.
    pub file_churn: HashMap<PathBuf, FileChurn>,

    /// Author statistics.
    pub author_stats: HashMap<String, AuthorStats>,

    /// Co-change patterns (files that change together).
    pub co_changes: Vec<CoChangePattern>,

    /// Hotspots (high churn + high complexity areas).
    pub hotspots: Vec<Hotspot>,

    /// Detected domains/areas from commit messages.
    pub domains: Vec<Domain>,
}

impl GitInsights {
    /// Create new empty git insights.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the top N files by churn.
    pub fn top_churning_files(&self, n: usize) -> Vec<(&PathBuf, &FileChurn)> {
        let mut files: Vec<_> = self.file_churn.iter().collect();
        files.sort_by(|a, b| b.1.change_count.cmp(&a.1.change_count));
        files.into_iter().take(n).collect()
    }

    /// Get the most active authors.
    pub fn top_authors(&self, n: usize) -> Vec<(&String, &AuthorStats)> {
        let mut authors: Vec<_> = self.author_stats.iter().collect();
        authors.sort_by(|a, b| b.1.commit_count.cmp(&a.1.commit_count));
        authors.into_iter().take(n).collect()
    }
}

/// Repository information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryInfo {
    /// Repository name.
    pub name: String,

    /// Remote URL (if any).
    pub remote_url: Option<String>,

    /// Default branch.
    pub default_branch: String,

    /// Current branch.
    pub current_branch: String,

    /// Total number of commits.
    pub total_commits: usize,

    /// Repository age (first commit date).
    pub created_at: Option<DateTime<Utc>>,

    /// Last commit date.
    pub last_commit_at: Option<DateTime<Utc>>,
}

/// Information about a single commit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    /// Commit hash (full).
    pub hash: String,

    /// Short hash (first 7 characters).
    pub short_hash: String,

    /// Commit message (first line).
    pub message: String,

    /// Full commit message.
    pub full_message: Option<String>,

    /// Author name.
    pub author_name: String,

    /// Author email.
    pub author_email: String,

    /// Commit timestamp.
    pub timestamp: DateTime<Utc>,

    /// Files changed in this commit.
    pub files_changed: Vec<FileChange>,

    /// Lines added.
    pub lines_added: usize,

    /// Lines deleted.
    pub lines_deleted: usize,

    /// Detected tags (e.g., "feat", "fix", "refactor" from conventional commits).
    pub tags: Vec<String>,
}

impl CommitInfo {
    /// Parse conventional commit tags from the message.
    pub fn parse_conventional_tags(message: &str) -> Vec<String> {
        let prefixes = ["feat", "fix", "docs", "style", "refactor", "test", "chore", "perf", "ci"];
        let message_lower = message.to_lowercase();
        
        prefixes
            .iter()
            .filter(|&prefix| {
                message_lower.starts_with(prefix)
                    || message_lower.starts_with(&format!("[{}]", prefix))
                    || message_lower.contains(&format!("({})", prefix))
            })
            .map(|s| s.to_string())
            .collect()
    }
}

/// Change to a file in a commit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    /// Path to the file.
    pub path: PathBuf,

    /// Kind of change.
    pub kind: FileChangeKind,

    /// Lines added.
    pub lines_added: usize,

    /// Lines deleted.
    pub lines_deleted: usize,
}

/// Kind of file change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileChangeKind {
    Added,
    Modified,
    Deleted,
    Renamed,
    Copied,
}

/// File churn statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileChurn {
    /// Number of times the file was changed.
    pub change_count: usize,

    /// Total lines added over all changes.
    pub total_lines_added: usize,

    /// Total lines deleted over all changes.
    pub total_lines_deleted: usize,

    /// Authors who changed this file.
    pub authors: Vec<String>,

    /// Last change timestamp.
    pub last_changed: Option<DateTime<Utc>>,

    /// Churn rate (changes per month).
    pub churn_rate: f32,
}

impl FileChurn {
    /// Check if this file is a hotspot (high churn).
    pub fn is_hotspot(&self, threshold: usize) -> bool {
        self.change_count >= threshold
    }
}

/// Author statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorStats {
    /// Author name.
    pub name: String,

    /// Author email.
    pub email: String,

    /// Number of commits.
    pub commit_count: usize,

    /// Total lines added.
    pub lines_added: usize,

    /// Total lines deleted.
    pub lines_deleted: usize,

    /// Files touched by this author.
    pub files_touched: Vec<PathBuf>,

    /// First commit timestamp.
    pub first_commit: Option<DateTime<Utc>>,

    /// Last commit timestamp.
    pub last_commit: Option<DateTime<Utc>>,
}

/// Files that frequently change together.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoChangePattern {
    /// Files in this pattern.
    pub files: Vec<PathBuf>,

    /// Number of commits they changed together.
    pub co_change_count: usize,

    /// Confidence score (0-1).
    pub confidence: f32,

    /// Possible reason for co-change.
    pub possible_reason: Option<String>,
}

/// A hotspot in the codebase (high churn + high complexity).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotspot {
    /// File path.
    pub path: PathBuf,

    /// Change count.
    pub change_count: usize,

    /// Complexity score.
    pub complexity_score: f32,

    /// Risk score (combined).
    pub risk_score: f32,

    /// Suggested action.
    pub suggestion: Option<String>,
}

impl Hotspot {
    /// Calculate risk score from change count and complexity.
    pub fn calculate_risk(change_count: usize, complexity_score: f32) -> f32 {
        // Normalize change count (assuming >50 changes is high)
        let churn_factor = (change_count as f32 / 50.0).min(1.0);
        // Weight: 40% churn, 60% complexity
        churn_factor * 0.4 + complexity_score * 0.6
    }
}

/// Detected domain/area from commit analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    /// Domain name.
    pub name: String,

    /// Description.
    pub description: Option<String>,

    /// Files belonging to this domain.
    pub files: Vec<PathBuf>,

    /// Keywords associated with this domain.
    pub keywords: Vec<String>,

    /// Activity level (0-1, based on recent changes).
    pub activity_level: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_conventional_tags() {
        let tags = CommitInfo::parse_conventional_tags("feat: add new feature");
        assert!(tags.contains(&"feat".to_string()));

        let tags = CommitInfo::parse_conventional_tags("fix(auth): resolve login issue");
        assert!(tags.contains(&"fix".to_string()));
    }

    #[test]
    fn test_file_churn_is_hotspot() {
        let mut churn = FileChurn::default();
        assert!(!churn.is_hotspot(10));

        churn.change_count = 15;
        assert!(churn.is_hotspot(10));
    }

    #[test]
    fn test_hotspot_risk_calculation() {
        let risk = Hotspot::calculate_risk(25, 0.5);
        assert!(risk > 0.0 && risk < 1.0);

        // High churn and high complexity should give high risk
        let high_risk = Hotspot::calculate_risk(100, 1.0);
        assert!(high_risk > 0.8);
    }
}
