use crate::app::AppState;
use git2::Repository;
use jwalk::WalkDir;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct VcsInfo {
    pub path: String,
    pub status: String,
}

pub fn render_vcs_widgets(f: &mut Frame, rect: Rect, app: &AppState) {
    let vcs_block = Block::default()
        .title("Version Control Status")
        .borders(Borders::ALL);

    let items: Vec<ListItem> = app
        .vcs_status
        .iter()
        .map(|info| ListItem::new(format!("{}: {}", info.path, info.status)))
        .collect();

    let list = List::new(items).block(vcs_block);

    f.render_widget(list, rect);
}

pub fn scan_vcs_paths(paths: &[PathBuf]) -> Vec<VcsInfo> {
    let mut vcs_infos = Vec::new();

    for path in paths {
        for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
            if entry.file_name() == ".git" && entry.file_type().is_dir() {
                if let Some(repo_path) = entry.path().parent() {
                    if let Ok(repo) = Repository::open(repo_path) {
                        let status = get_git_status(&repo);
                        vcs_infos.push(VcsInfo {
                            path: repo_path.to_string_lossy().to_string(),
                            status,
                        });
                    }
                }
            }
        }
    }

    vcs_infos
}

fn get_git_status(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return "Detached HEAD".to_string(),
    };

    let branch = head.shorthand().unwrap_or("Unknown");

    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true);

    let statuses = match repo.statuses(Some(&mut opts)) {
        Ok(statuses) => statuses,
        Err(_) => return format!("{} [Error]", branch),
    };

    let changed_files = statuses.iter().count();

    if changed_files == 0 {
        format!("{} [Clean]", branch)
    } else {
        format!("{} [{} changes]", branch, changed_files)
    }
}
