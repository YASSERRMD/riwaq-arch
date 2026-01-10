//! Git repository analysis for extracting history and insights.
//!
//! This module provides functionality to analyze git history,
//! compute file churn, identify hotspots, and detect co-change patterns.

use chrono::{TimeZone, Utc};
use git2::{Commit, DiffOptions, Repository, Sort};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use tracing::info;

use crate::errors::{Result, RiwaqError};
use crate::models::git::{
    AuthorStats, CoChangePattern, CommitInfo, Domain, FileChange, FileChangeKind, FileChurn,
    GitInsights, Hotspot, RepositoryInfo,
};

/// Configuration for git analysis.
#[derive(Debug, Clone)]
pub struct GitAnalysisConfig {
    /// Maximum number of commits to analyze.
    pub max_commits: usize,

    /// Minimum number of co-changes to consider a pattern.
    pub min_co_change_count: usize,

    /// Churn threshold for hotspot detection.
    pub hotspot_churn_threshold: usize,

    /// Whether to analyze commit messages for domains.
    pub analyze_domains: bool,
}

impl Default for GitAnalysisConfig {
    fn default() -> Self {
        Self {
            max_commits: 1000,
            min_co_change_count: 5,
            hotspot_churn_threshold: 10,
            analyze_domains: true,
        }
    }
}

/// Git repository analyzer.
pub struct GitAnalyzer {
    config: GitAnalysisConfig,
}

impl GitAnalyzer {
    /// Create a new git analyzer with default configuration.
    pub fn new() -> Self {
        Self {
            config: GitAnalysisConfig::default(),
        }
    }

    /// Create a new git analyzer with custom configuration.
    pub fn with_config(config: GitAnalysisConfig) -> Self {
        Self { config }
    }

    /// Set the maximum number of commits to analyze.
    pub fn max_commits(mut self, max: usize) -> Self {
        self.config.max_commits = max;
        self
    }

    /// Analyze a git repository.
    pub fn analyze(&self, repo_path: &Path) -> Result<GitInsights> {
        let repo = Repository::discover(repo_path).map_err(|e| {
            if e.code() == git2::ErrorCode::NotFound {
                RiwaqError::NotAGitRepository(repo_path.to_path_buf())
            } else {
                RiwaqError::Git(e)
            }
        })?;

        info!(path = ?repo_path, "Analyzing git repository");

        let mut insights = GitInsights::new();

        // Get repository info
        insights.repository = Some(self.get_repo_info(&repo)?);

        // Get commits
        let commits = self.get_commits(&repo)?;
        info!(count = commits.len(), "Collected commits");

        // Compute file churn
        insights.file_churn = self.compute_file_churn(&commits);

        // Compute author stats
        insights.author_stats = self.compute_author_stats(&commits);

        // Find co-change patterns
        insights.co_changes = self.find_co_change_patterns(&commits);

        // Store commits (limited for memory)
        insights.commits = commits.into_iter().take(100).collect();

        // Detect domains from commit messages
        if self.config.analyze_domains {
            insights.domains = self.detect_domains(&insights.commits);
        }

        Ok(insights)
    }

    /// Get repository information.
    fn get_repo_info(&self, repo: &Repository) -> Result<RepositoryInfo> {
        let workdir = repo.workdir().unwrap_or(Path::new("."));
        let name = workdir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Get remote URL
        let remote_url = repo
            .find_remote("origin")
            .ok()
            .and_then(|r| r.url().map(|s| s.to_string()));

        // Get current branch
        let head = repo.head().ok();
        let current_branch = head
            .as_ref()
            .and_then(|h| h.shorthand())
            .unwrap_or("HEAD")
            .to_string();

        // Get default branch (usually main or master)
        let default_branch = if repo.find_branch("main", git2::BranchType::Local).is_ok() {
            "main".to_string()
        } else if repo.find_branch("master", git2::BranchType::Local).is_ok() {
            "master".to_string()
        } else {
            current_branch.clone()
        };

        // Count commits
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        let total_commits = revwalk.count();

        // Get first and last commit dates
        let (created_at, last_commit_at) = self.get_commit_date_range(repo)?;

        Ok(RepositoryInfo {
            name,
            remote_url,
            default_branch,
            current_branch,
            total_commits,
            created_at,
            last_commit_at,
        })
    }

    /// Get the date range of commits.
    fn get_commit_date_range(
        &self,
        repo: &Repository,
    ) -> Result<(Option<chrono::DateTime<Utc>>, Option<chrono::DateTime<Utc>>)> {
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(Sort::TIME)?;

        let mut first_commit_time = None;
        let mut last_commit_time = None;

        for (i, oid) in revwalk.enumerate() {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;
            let time = commit.time();
            let datetime = Utc.timestamp_opt(time.seconds(), 0).single();

            if i == 0 {
                last_commit_time = datetime;
            }
            first_commit_time = datetime;
        }

        Ok((first_commit_time, last_commit_time))
    }

    /// Get commit history.
    fn get_commits(&self, repo: &Repository) -> Result<Vec<CommitInfo>> {
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(Sort::TIME)?;

        let mut commits = Vec::new();

        for (i, oid) in revwalk.enumerate() {
            if i >= self.config.max_commits {
                break;
            }

            let oid = oid?;
            let commit = repo.find_commit(oid)?;

            if let Some(commit_info) = self.parse_commit(repo, &commit)? {
                commits.push(commit_info);
            }
        }

        Ok(commits)
    }

    /// Parse a single commit.
    fn parse_commit(&self, repo: &Repository, commit: &Commit) -> Result<Option<CommitInfo>> {
        let hash = commit.id().to_string();
        let short_hash = hash[..7.min(hash.len())].to_string();

        let message = commit.message().unwrap_or("").to_string();
        let first_line = message.lines().next().unwrap_or("").to_string();

        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();
        let author_email = author.email().unwrap_or("unknown@example.com").to_string();

        let time = commit.time();
        let timestamp = Utc
            .timestamp_opt(time.seconds(), 0)
            .single()
            .unwrap_or_else(Utc::now);

        // Get file changes
        let files_changed = self.get_commit_changes(repo, commit)?;

        let (lines_added, lines_deleted) = files_changed.iter().fold((0, 0), |acc, change| {
            (acc.0 + change.lines_added, acc.1 + change.lines_deleted)
        });

        // Parse conventional commit tags
        let tags = CommitInfo::parse_conventional_tags(&first_line);

        Ok(Some(CommitInfo {
            hash,
            short_hash,
            message: first_line,
            full_message: Some(message),
            author_name,
            author_email,
            timestamp,
            files_changed,
            lines_added,
            lines_deleted,
            tags,
        }))
    }

    /// Get files changed in a commit.
    fn get_commit_changes(&self, repo: &Repository, commit: &Commit) -> Result<Vec<FileChange>> {
        let tree = commit.tree()?;

        let parent_tree = if commit.parent_count() > 0 {
            commit.parent(0).ok().and_then(|p| p.tree().ok())
        } else {
            None
        };

        let mut diff_opts = DiffOptions::new();
        diff_opts.ignore_whitespace(true);

        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), Some(&mut diff_opts))?;

        let mut changes = Vec::new();

        diff.foreach(
            &mut |delta, _| {
                let path = delta
                    .new_file()
                    .path()
                    .or_else(|| delta.old_file().path())
                    .map(|p| p.to_path_buf())
                    .unwrap_or_default();

                let kind = match delta.status() {
                    git2::Delta::Added => FileChangeKind::Added,
                    git2::Delta::Deleted => FileChangeKind::Deleted,
                    git2::Delta::Modified => FileChangeKind::Modified,
                    git2::Delta::Renamed => FileChangeKind::Renamed,
                    git2::Delta::Copied => FileChangeKind::Copied,
                    _ => FileChangeKind::Modified,
                };

                changes.push(FileChange {
                    path,
                    kind,
                    lines_added: 0,
                    lines_deleted: 0,
                });

                true
            },
            None,
            None,
            None,
        )?;

        // Get line stats
        let stats = diff.stats()?;
        let total_added = stats.insertions();
        let total_deleted = stats.deletions();

        // Distribute evenly if we have changes (simplified)
        if !changes.is_empty() {
            let per_file_added = total_added / changes.len();
            let per_file_deleted = total_deleted / changes.len();
            for change in &mut changes {
                change.lines_added = per_file_added;
                change.lines_deleted = per_file_deleted;
            }
        }

        Ok(changes)
    }

    /// Compute file churn from commits.
    fn compute_file_churn(&self, commits: &[CommitInfo]) -> HashMap<PathBuf, FileChurn> {
        let mut churn_map: HashMap<PathBuf, FileChurn> = HashMap::new();

        for commit in commits {
            for change in &commit.files_changed {
                let entry = churn_map.entry(change.path.clone()).or_default();
                entry.change_count += 1;
                entry.total_lines_added += change.lines_added;
                entry.total_lines_deleted += change.lines_deleted;

                if !entry.authors.contains(&commit.author_name) {
                    entry.authors.push(commit.author_name.clone());
                }

                if entry.last_changed.is_none() || entry.last_changed < Some(commit.timestamp) {
                    entry.last_changed = Some(commit.timestamp);
                }
            }
        }

        // Calculate churn rates
        if let (Some(oldest), Some(newest)) = (commits.last(), commits.first()) {
            let months = (newest.timestamp - oldest.timestamp).num_days() as f32 / 30.0;
            if months > 0.0 {
                for churn in churn_map.values_mut() {
                    churn.churn_rate = churn.change_count as f32 / months;
                }
            }
        }

        churn_map
    }

    /// Compute author statistics.
    fn compute_author_stats(&self, commits: &[CommitInfo]) -> HashMap<String, AuthorStats> {
        let mut stats_map: HashMap<String, AuthorStats> = HashMap::new();

        for commit in commits {
            let entry = stats_map
                .entry(commit.author_email.clone())
                .or_insert_with(|| AuthorStats {
                    name: commit.author_name.clone(),
                    email: commit.author_email.clone(),
                    ..Default::default()
                });

            entry.commit_count += 1;
            entry.lines_added += commit.lines_added;
            entry.lines_deleted += commit.lines_deleted;

            for change in &commit.files_changed {
                if !entry.files_touched.contains(&change.path) {
                    entry.files_touched.push(change.path.clone());
                }
            }

            if entry.first_commit.is_none() || entry.first_commit > Some(commit.timestamp) {
                entry.first_commit = Some(commit.timestamp);
            }
            if entry.last_commit.is_none() || entry.last_commit < Some(commit.timestamp) {
                entry.last_commit = Some(commit.timestamp);
            }
        }

        stats_map
    }

    /// Find files that frequently change together.
    fn find_co_change_patterns(&self, commits: &[CommitInfo]) -> Vec<CoChangePattern> {
        // Build co-change matrix
        let mut co_change_counts: HashMap<(PathBuf, PathBuf), usize> = HashMap::new();

        for commit in commits {
            let files: Vec<_> = commit.files_changed.iter().map(|c| &c.path).collect();

            for i in 0..files.len() {
                for j in (i + 1)..files.len() {
                    let key = if files[i] < files[j] {
                        (files[i].clone(), files[j].clone())
                    } else {
                        (files[j].clone(), files[i].clone())
                    };
                    *co_change_counts.entry(key).or_insert(0) += 1;
                }
            }
        }

        // Filter and group significant patterns
        let mut patterns = Vec::new();

        for ((file1, file2), count) in co_change_counts {
            if count >= self.config.min_co_change_count {
                let confidence = count as f32 / commits.len() as f32;

                patterns.push(CoChangePattern {
                    files: vec![file1, file2],
                    co_change_count: count,
                    confidence,
                    possible_reason: None,
                });
            }
        }

        // Sort by co-change count
        patterns.sort_by(|a, b| b.co_change_count.cmp(&a.co_change_count));

        // Take top 50
        patterns.truncate(50);

        patterns
    }

    /// Detect domains from commit messages.
    fn detect_domains(&self, commits: &[CommitInfo]) -> Vec<Domain> {
        // Simple keyword-based domain detection
        let domain_keywords: HashMap<&str, Vec<&str>> = [
            ("auth", vec!["auth", "login", "logout", "jwt", "token", "session", "password"]),
            ("api", vec!["api", "endpoint", "route", "handler", "controller", "rest", "graphql"]),
            ("database", vec!["database", "db", "sql", "query", "migration", "schema", "model"]),
            ("ui", vec!["ui", "frontend", "component", "view", "style", "css", "react", "vue"]),
            ("testing", vec!["test", "spec", "mock", "fixture", "coverage"]),
            ("docs", vec!["doc", "readme", "documentation", "comment"]),
            ("config", vec!["config", "env", "setting", "option", "parameter"]),
            ("infra", vec!["docker", "k8s", "kubernetes", "deploy", "ci", "cd", "pipeline"]),
        ]
        .into_iter()
        .collect();

        let mut domain_files: HashMap<&str, HashSet<PathBuf>> = HashMap::new();
        let mut domain_activity: HashMap<&str, usize> = HashMap::new();

        for commit in commits {
            let message_lower = commit.message.to_lowercase();
            let full_message_lower = commit
                .full_message
                .as_ref()
                .map(|m| m.to_lowercase())
                .unwrap_or_default();

            for (domain, keywords) in &domain_keywords {
                let matches = keywords.iter().any(|kw| {
                    message_lower.contains(kw) || full_message_lower.contains(kw)
                });

                if matches {
                    *domain_activity.entry(domain).or_insert(0) += 1;

                    for change in &commit.files_changed {
                        domain_files
                            .entry(domain)
                            .or_default()
                            .insert(change.path.clone());
                    }
                }
            }
        }

        let max_activity = domain_activity.values().copied().max().unwrap_or(1) as f32;

        domain_keywords
            .keys()
            .filter_map(|&domain| {
                let files: Vec<PathBuf> = domain_files
                    .get(domain)
                    .map(|s| s.iter().cloned().collect())
                    .unwrap_or_default();

                if files.is_empty() {
                    return None;
                }

                let activity = *domain_activity.get(domain).unwrap_or(&0) as f32 / max_activity;

                Some(Domain {
                    name: domain.to_string(),
                    description: Some(format!("Files related to {}", domain)),
                    files,
                    keywords: domain_keywords[domain]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                    activity_level: activity,
                })
            })
            .collect()
    }

    /// Identify hotspots in the codebase.
    pub fn identify_hotspots(
        &self,
        file_churn: &HashMap<PathBuf, FileChurn>,
        complexity_scores: &HashMap<PathBuf, f32>,
    ) -> Vec<Hotspot> {
        let mut hotspots: Vec<Hotspot> = file_churn
            .iter()
            .filter(|(_, churn)| churn.change_count >= self.config.hotspot_churn_threshold)
            .map(|(path, churn)| {
                let complexity = complexity_scores.get(path).copied().unwrap_or(0.5);
                let risk = Hotspot::calculate_risk(churn.change_count, complexity);

                Hotspot {
                    path: path.clone(),
                    change_count: churn.change_count,
                    complexity_score: complexity,
                    risk_score: risk,
                    suggestion: if risk > 0.7 {
                        Some("Consider refactoring this high-risk file".to_string())
                    } else {
                        None
                    },
                }
            })
            .collect();

        hotspots.sort_by(|a, b| b.risk_score.partial_cmp(&a.risk_score).unwrap());
        hotspots.truncate(20);
        hotspots
    }
}

impl Default for GitAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::process::Command;

    fn create_test_repo() -> TempDir {
        let dir = TempDir::new().unwrap();
        
        // Initialize git repo
        Command::new("git")
            .args(["init"])
            .current_dir(dir.path())
            .output()
            .unwrap();

        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(dir.path())
            .output()
            .unwrap();

        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(dir.path())
            .output()
            .unwrap();

        // Create and commit a file
        std::fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
        
        Command::new("git")
            .args(["add", "."])
            .current_dir(dir.path())
            .output()
            .unwrap();

        Command::new("git")
            .args(["commit", "-m", "feat: initial commit"])
            .current_dir(dir.path())
            .output()
            .unwrap();

        dir
    }

    #[test]
    fn test_analyze_repo() {
        let dir = create_test_repo();
        let analyzer = GitAnalyzer::new();
        
        let insights = analyzer.analyze(dir.path()).unwrap();
        
        assert!(insights.repository.is_some());
        assert!(!insights.commits.is_empty());
    }

    #[test]
    fn test_parse_conventional_tags() {
        let tags = CommitInfo::parse_conventional_tags("feat(auth): add login");
        assert!(tags.contains(&"feat".to_string()));

        let tags = CommitInfo::parse_conventional_tags("fix: resolve bug");
        assert!(tags.contains(&"fix".to_string()));
    }

    #[test]
    fn test_not_a_git_repo() {
        let dir = TempDir::new().unwrap();
        let analyzer = GitAnalyzer::new();
        
        let result = analyzer.analyze(dir.path());
        assert!(matches!(result, Err(RiwaqError::NotAGitRepository(_))));
    }
}
