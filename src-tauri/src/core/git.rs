/*!
 * Git Manager Module
 * 
 * Manages Git operations and repository state.
 */

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;
use git2::{Repository, Status, StatusOptions, DiffOptions, DiffFormat};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitManager {
    pub repositories: HashMap<PathBuf, GitRepository>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitRepository {
    pub path: PathBuf,
    pub current_branch: String,
    pub remote_branches: Vec<String>,
    pub local_branches: Vec<String>,
    pub status: GitStatus,
    pub last_commit: Option<CommitInfo>,
    pub is_dirty: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitStatus {
    pub modified_files: Vec<FileStatus>,
    pub staged_files: Vec<FileStatus>,
    pub untracked_files: Vec<FileStatus>,
    pub deleted_files: Vec<FileStatus>,
    pub renamed_files: Vec<FileStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStatus {
    pub path: PathBuf,
    pub status: FileStatusType,
    pub diff: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileStatusType {
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Unmerged,
    Untracked,
    Ignored,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub is_current: bool,
    pub is_remote: bool,
    pub last_commit: Option<CommitInfo>,
}

impl Default for GitManager {
    fn default() -> Self {
        Self {
            repositories: HashMap::new(),
        }
    }
}

impl GitManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open_repository(&mut self, path: PathBuf) -> Result<()> {
        let repo = Repository::open(&path)?;
        let git_repo = self.create_git_repository(&repo, &path)?;
        self.repositories.insert(path, git_repo);
        Ok(())
    }

    fn create_git_repository(&self, repo: &Repository, path: &PathBuf) -> Result<GitRepository> {
        let current_branch = self.get_current_branch(repo)?;
        let remote_branches = self.get_remote_branches(repo)?;
        let local_branches = self.get_local_branches(repo)?;
        let status = self.get_repository_status_from_repo(repo)?;
        let last_commit = self.get_last_commit(repo)?;
        let is_dirty = !status.modified_files.is_empty() 
            || !status.staged_files.is_empty() 
            || !status.untracked_files.is_empty();

        Ok(GitRepository {
            path: path.clone(),
            current_branch,
            remote_branches,
            local_branches,
            status,
            last_commit,
            is_dirty,
        })
    }

    pub fn get_repository_status(&self, repo_path: &PathBuf) -> Result<GitStatus> {
        if let Some(git_repo) = self.repositories.get(repo_path) {
            Ok(git_repo.status.clone())
        } else {
            // Open repository temporarily to get status
            let repo = Repository::open(repo_path)?;
            self.get_repository_status_from_repo(&repo)
        }
    }

    fn get_repository_status_from_repo(&self, repo: &Repository) -> Result<GitStatus> {
        let mut status_options = StatusOptions::new();
        status_options.include_untracked(true);
        status_options.include_ignored(false);

        let statuses = repo.statuses(Some(&mut status_options))?;
        
        let mut modified_files = Vec::new();
        let mut staged_files = Vec::new();
        let mut untracked_files = Vec::new();
        let mut deleted_files = Vec::new();
        let mut renamed_files = Vec::new();

        for entry in statuses.iter() {
            let path = PathBuf::from(entry.path().unwrap_or(""));
            let status = entry.status();
            let diff = self.get_file_diff(repo, &path, status)?;

            let file_status = FileStatus {
                path,
                status: self.map_git_status(status),
                diff,
            };

            if status.is_index_new() || status.is_index_modified() {
                staged_files.push(file_status);
            } else if status.is_wt_modified() {
                modified_files.push(file_status);
            } else if status.is_wt_new() {
                untracked_files.push(file_status);
            } else if status.is_wt_deleted() {
                deleted_files.push(file_status);
            } else if status.is_wt_renamed() {
                renamed_files.push(file_status);
            }
        }

        Ok(GitStatus {
            modified_files,
            staged_files,
            untracked_files,
            deleted_files,
            renamed_files,
        })
    }

    fn get_file_diff(&self, repo: &Repository, path: &PathBuf, status: Status) -> Result<Option<String>> {
        if !status.is_wt_modified() && !status.is_index_modified() {
            return Ok(None);
        }

        let mut diff_options = DiffOptions::new();
        diff_options.pathspec(path.to_string_lossy().as_ref());

        let diff = repo.diff_index_to_workdir(None, Some(&mut diff_options))?;
        let mut diff_text = String::new();
        
        diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
            diff_text.push_str(&format!("{}\n", std::str::from_utf8(line.content()).unwrap_or("")));
            true
        })?;

        Ok(Some(diff_text))
    }

    fn map_git_status(&self, status: Status) -> FileStatusType {
        if status.is_wt_new() || status.is_index_new() {
            FileStatusType::Added
        } else if status.is_wt_deleted() || status.is_index_deleted() {
            FileStatusType::Deleted
        } else if status.is_wt_renamed() {
            FileStatusType::Renamed
        } else if status.is_conflicted() {
            FileStatusType::Copied
        } else if status.is_conflicted() {
            FileStatusType::Unmerged
        } else if status.is_ignored() {
            FileStatusType::Ignored
        } else {
            FileStatusType::Modified
        }
    }

    pub fn stage_file(&self, repo_path: &PathBuf, file_path: &PathBuf) -> Result<()> {
        let repo = Repository::open(repo_path)?;
        let mut index = repo.index()?;
        index.add_path(file_path)?;
        index.write()?;
        Ok(())
    }

    pub fn unstage_file(&self, repo_path: &PathBuf, file_path: &PathBuf) -> Result<()> {
        let repo = Repository::open(repo_path)?;
        let mut index = repo.index()?;
        index.remove_path(file_path)?;
        index.write()?;
        Ok(())
    }

    pub fn commit_changes(&self, repo_path: &PathBuf, message: &str, author_name: &str, author_email: &str) -> Result<String> {
        let repo = Repository::open(repo_path)?;
        let mut index = repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        
        let signature = git2::Signature::now(author_name, author_email)?;
        let head = repo.head()?;
        let parent_commit = repo.find_commit(head.target().unwrap())?;
        
        let commit_id = repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&parent_commit],
        )?;

        Ok(commit_id.to_string())
    }

    pub fn push_changes(&self, repo_path: &PathBuf, remote_name: &str, branch_name: &str) -> Result<()> {
        let repo = Repository::open(repo_path)?;
        let mut remote = repo.find_remote(remote_name)?;
        
        let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
        remote.push(&[&refspec], None)?;
        
        Ok(())
    }

    pub fn pull_changes(&self, repo_path: &PathBuf, remote_name: &str, branch_name: &str) -> Result<()> {
        let repo = Repository::open(repo_path)?;
        let mut remote = repo.find_remote(remote_name)?;
        
        remote.fetch(&[branch_name], None, None)?;
        
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        
        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        
        if analysis.0.is_fast_forward() {
            let mut reference = repo.find_reference(&format!("refs/heads/{}", branch_name))?;
            reference.set_target(fetch_commit.id(), "Fast-Forward")?;
            repo.set_head(&format!("refs/heads/{}", branch_name))?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        } else if analysis.0.is_normal() {
            // Handle merge conflicts
            return Err(anyhow::anyhow!("Merge conflicts detected"));
        }
        
        Ok(())
    }

    pub fn create_branch(&self, repo_path: &PathBuf, branch_name: &str) -> Result<()> {
        let repo = Repository::open(repo_path)?;
        let head = repo.head()?;
        let commit = repo.find_commit(head.target().unwrap())?;
        
        repo.branch(branch_name, &commit, false)?;
        Ok(())
    }

    pub fn switch_branch(&self, repo_path: &PathBuf, branch_name: &str) -> Result<()> {
        let repo = Repository::open(repo_path)?;
        let reference = repo.find_reference(&format!("refs/heads/{}", branch_name))?;
        let commit = repo.reference_to_annotated_commit(&reference)?;
        
        let tree = repo.find_tree(commit.id())?;
        let tree_obj = tree.as_object();
        repo.checkout_tree(tree_obj, Some(git2::build::CheckoutBuilder::default().force()))?;
        repo.set_head(&format!("refs/heads/{}", branch_name))?;
        
        Ok(())
    }

    fn get_current_branch(&self, repo: &Repository) -> Result<String> {
        let head = repo.head()?;
        if let Some(name) = head.shorthand() {
            Ok(name.to_string())
        } else {
            Ok("detached".to_string())
        }
    }

    fn get_remote_branches(&self, repo: &Repository) -> Result<Vec<String>> {
        let mut branches = Vec::new();
        let remotes = repo.remotes()?;
        
        for remote_name in remotes.iter().flatten() {
            let remote = repo.find_remote(remote_name)?;
            let refs = remote.list()?;
            
            for reference in refs {
                if let Some(branch_name) = reference.name().strip_prefix(&format!("refs/heads/")) {
                    branches.push(format!("{}/{}", remote_name, branch_name));
                }
            }
        }
        
        Ok(branches)
    }

    fn get_local_branches(&self, repo: &Repository) -> Result<Vec<String>> {
        let mut branches = Vec::new();
        let branch_iter = repo.branches(Some(git2::BranchType::Local))?;
        
        for branch in branch_iter {
            let (branch, _) = branch?;
            if let Some(name) = branch.name()? {
                branches.push(name.to_string());
            }
        }
        
        Ok(branches)
    }

    fn get_last_commit(&self, repo: &Repository) -> Result<Option<CommitInfo>> {
        let head = repo.head()?;
        if let Ok(commit) = repo.find_commit(head.target().unwrap()) {
            Ok(Some(CommitInfo {
                hash: commit.id().to_string(),
                message: commit.message().unwrap_or("").to_string(),
                author: commit.author().name().unwrap_or("").to_string(),
                email: commit.author().email().unwrap_or("").to_string(),
                timestamp: chrono::DateTime::from_timestamp(commit.time().seconds(), 0)
                    .unwrap_or_default(),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn refresh_repository(&mut self, repo_path: &PathBuf) -> Result<()> {
        if self.repositories.contains_key(repo_path) {
            let repo = Repository::open(repo_path)?;
            let updated_repo = self.create_git_repository(&repo, repo_path)?;
            self.repositories.insert(repo_path.clone(), updated_repo);
        }
        Ok(())
    }

    pub fn get_repositories(&self) -> Vec<&GitRepository> {
        self.repositories.values().collect()
    }
}