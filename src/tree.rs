use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use git2::{Repository, Status};
use serde::Serialize;

#[derive(Debug)]
pub struct Tree {
    pub path: String,
    extensions: Vec<String>,
    ignores: Vec<String>,
    max_depth: Option<u32>,
    pub output_format: OutputFormat,
    pub git_intergration: bool,
}

#[derive(Debug)]
pub enum OutputFormat {
    Standard,
    Json,
    Markdown,
}

#[derive(Serialize, Debug)]
pub struct TreeNode {
    name: String,
    git_status: Option<String>,
    children: Option<Vec<TreeNode>>,
}

impl Tree {
    pub fn new(matches: &clap::ArgMatches) -> Self {
        let path = matches.get_one::<String>("path").unwrap().clone();
        let exts = matches
            .get_many::<String>("ext")
            .map(|vals| vals.cloned().collect())
            .unwrap_or_default();
        let ignores = matches
            .get_many::<String>("ignore")
            .map(|vals| vals.cloned().collect())
            .unwrap_or_default();
        let max_depth = matches.get_one::<u32>("depth").copied();
        let output_format = if matches.get_flag("json") {
            OutputFormat::Json
        } else if matches.get_flag("markdown") {
            OutputFormat::Markdown
        } else {
            OutputFormat::Standard
        };

        let git_intergration = matches.get_flag("git");
        Self {
            path: path,
            extensions: exts,
            ignores: ignores,
            max_depth: max_depth,
            output_format: output_format,
            git_intergration: git_intergration,
        }
    }
}

pub fn get_git_statuses(repo_path: &Path) -> HashMap<PathBuf, Status> {
    let repo = Repository::discover(repo_path).expect("Not a git repository");
    let mut statuses = HashMap::new();
    let mut status_options = git2::StatusOptions::new();
    let git_statuses = repo
        .statuses(Some(&mut status_options))
        .expect("Failed to get git statuses");

    for entry in git_statuses.iter() {
        if let Some(path) = entry.path() {
            statuses.insert(repo_path.join(path), entry.status());
        }
    }

    statuses
}

pub fn print_tree(path: &Path, prefix: &str, depth: u32, config: &Tree) {
    if let Some(max_depth) = config.max_depth {
        if depth > max_depth {
            return;
        }
    }

    if prefix.is_empty() {
        println!(
            "{}",
            &path
                .canonicalize()
                .ok()
                .unwrap_or_default()
                .file_name()
                .map(|f| f.to_str().unwrap_or_default())
                .unwrap_or_default()
        );
    }

    if let Ok(entries) = fs::read_dir(path) {
        let entries: Vec<_> = entries
            .filter_map(|d| d.ok())
            .filter(|entry| {
                let filename = entry.file_name().to_string_lossy().to_string();
                !config.ignores.contains(&filename)
            })
            .collect();

        entries.iter().enumerate().for_each(|(i, entry)| {
            let entry_file_name = entry.file_name();
            let filename = entry_file_name.to_str().unwrap_or_default();
            let is_last = i == entries.len() - 1;
            let new_prefix = if is_last {
                " └── "
            } else {
                " ├── "
            };

            println!("{}{}{}", prefix, new_prefix, filename);

            if entry.path().is_dir() {
                let child_prefix = if is_last { "     " } else { " │   " };
                print_tree(
                    &entry.path(),
                    &format!("{}{}", prefix, child_prefix),
                    depth + 1,
                    config,
                );
            }
        });
    }
}

pub fn build_tree(
    path: &Path,
    depth: u32,
    tree: &Tree,
    git_statuses: &HashMap<PathBuf, Status>,
) -> Option<TreeNode> {
    if let Some(max_depth) = tree.max_depth {
        if depth > max_depth {
            return None;
        }
    }
    let name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let name = if name.is_empty() {
        path.canonicalize()
            .ok()
            .unwrap_or_default()
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or_default()
            .to_string()
    } else {
        name
    };
    if tree.ignores.contains(&name) {
        return None;
    }

    let git_status = git_statuses.get(path).map(|status| format!("{:?}", status));

    if path.is_dir() {
        let children: Vec<TreeNode> = fs::read_dir(path)
            .ok()?
            .filter_map(Result::ok)
            .filter(|entry| {
                tree.extensions.is_empty()
                    || entry
                        .path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| tree.extensions.contains(&ext.to_string()))
                        .unwrap_or(false)
            })
            .filter_map(|entry| build_tree(&entry.path(), depth + 1, tree, git_statuses))
            .collect();

        Some(TreeNode {
            name,
            git_status: git_status,
            children: if children.is_empty() {
                None
            } else {
                Some(children)
            },
        })
    } else {
        Some(TreeNode {
            name,
            git_status: git_status,
            children: None,
        })
    }
}

pub fn tree_to_markdown(node: &TreeNode, depth: usize) -> String {
    let mut markdown = format!("{}- {}\n", "  ".repeat(depth), node.name);
    if let Some(children) = &node.children {
        for child in children {
            markdown.push_str(&tree_to_markdown(child, depth + 1));
        }
    }
    markdown
}
