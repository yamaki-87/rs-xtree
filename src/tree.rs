use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
    str::FromStr,
};

use clap::ValueEnum;
use colored::Colorize;
use git2::{Repository, Status};
use serde::Serialize;

use crate::{
    constatns,
    utils::{self, size},
};

#[derive(Debug)]
pub struct Tree {
    pub path: String,
    extensions: Vec<String>,
    size: Option<SizeFormat>,
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

#[derive(Debug, Clone)]
pub enum SizeFormat {
    Bytes,
    HumanReadable,
}

impl ValueEnum for SizeFormat {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Bytes, Self::HumanReadable]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            SizeFormat::Bytes => Some(clap::builder::PossibleValue::new("b").alias("bytes")),
            SizeFormat::HumanReadable => {
                Some(clap::builder::PossibleValue::new("h").alias("humanreadable"))
            }
        }
    }
}

impl FromStr for SizeFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | "b" | "bytes" => Ok(SizeFormat::Bytes),
            "h" | "humanreadable" => Ok(SizeFormat::HumanReadable),
            _ => Err(format!("Invalid size format:{}", s)),
        }
    }
}
#[derive(Serialize, Debug)]
pub struct TreeNode {
    name: String,
    git_status: Option<String>,
    children: Option<Vec<TreeNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<size::Unit>,
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
        let size = matches.get_one::<SizeFormat>("size").cloned();

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
            size: size,
            max_depth: max_depth,
            output_format: output_format,
            git_intergration: git_intergration,
        }
    }

    fn ext_filter(&self, entry: &DirEntry) -> bool {
        !entry
            .path()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.extensions.contains(&ext.to_string()))
            .unwrap_or(false)
    }

    fn ignore_filename_filter(&self, entry: &DirEntry) -> bool {
        let filename = entry.file_name().to_string_lossy().to_string();
        !self.ignores.contains(&filename)
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

pub fn print_tree(node: &TreeNode, prefix: &str, is_last: bool) {
    let connector = if prefix.is_empty() {
        ""
    } else {
        if is_last {
            " └── "
        } else {
            " ├── "
        }
    };

    let status = node.git_status.as_deref().unwrap_or("");
    let colored_name = if status.contains("WT_MODIFIED") {
        node.name.yellow()
    } else if status.contains("WT_NEW") {
        node.name.green()
    } else if node.children.is_some() {
        node.name.blue()
    } else {
        node.name.white()
    };

    let size_str = node
        .size
        .as_ref()
        .map(|s| format!(" ({})", s))
        .unwrap_or_default()
        .red();

    println!("{}{}{}{}", prefix, connector, colored_name, size_str);

    if let Some(children) = &node.children {
        let new_prefix = format!("{}{}", prefix, if is_last { "     " } else { " │   " });
        let len = children.len();
        for (i, child) in children.iter().enumerate() {
            print_tree(child, &new_prefix, i == len - 1);
        }
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
    let name = utils::files::get_filename(path);
    if tree.ignores.contains(&name) {
        return None;
    }

    let git_status = git_statuses.get(path).map(|status| format!("{:?}", status));

    if path.is_dir() {
        let children: Vec<TreeNode> = fs::read_dir(&path)
            .ok()?
            .filter_map(Result::ok)
            .filter(|entry| tree.ext_filter(entry))
            .filter(|entry| tree.ignore_filename_filter(entry))
            .filter_map(|entry| build_tree(&entry.path(), depth + 1, tree, git_statuses))
            .collect();

        let size = if let Some(size) = &tree.size {
            match size {
                SizeFormat::Bytes => utils::files::get_filesize(path)
                    .map_err(|e| eprintln!("ERROR: {}", e))
                    .ok()
                    .map(|s| size::Unit::Byte(s)),
                SizeFormat::HumanReadable => utils::files::get_human_readable_filesize(path)
                    .map_err(|e| eprintln!("{}", e))
                    .ok(),
            }
        } else {
            None
        };

        Some(TreeNode {
            name,
            git_status: git_status,
            children: if children.is_empty() {
                None
            } else {
                Some(children)
            },
            size: size,
        })
    } else {
        let size = if let Some(size_format) = &tree.size {
            match size_format {
                SizeFormat::Bytes => {
                    let metadata = path.metadata().map_err(|e| eprintln!("ERROR: {}", e)).ok();
                    metadata.map(|m| size::Unit::Byte(m.len()))
                }
                SizeFormat::HumanReadable => utils::files::get_human_readable_filesize(path)
                    .map_err(|e| eprintln!("ERROR:{}", e))
                    .ok(),
            }
        } else {
            None
        };
        Some(TreeNode {
            name,
            git_status: git_status,
            children: None,
            size: size,
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
