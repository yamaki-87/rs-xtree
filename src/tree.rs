use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
    str::FromStr,
};

use async_recursion::async_recursion;
use clap::ValueEnum;
use colored::{ColoredString, Colorize};
use git2::{Repository, Status};
use serde::Serialize;

use crate::{
    constatns::{self, STR_EMPTY},
    foramt::{mode::Mode, output::OutputFormat, sizeformat::SizeFormat, sort::SortKey},
    utils::{self, files::MetaDataInfo, size},
};

const TREE_BRANCH: &str = " ├── ";
const TREE_LAST_BRANCH: &str = " └── ";
const TREE_VERTICAL: &str = " │   ";
const TREE_LAST_EMPTY: &str = "     ";

#[derive(Debug)]
pub struct Tree {
    pub path: String,
    extensions: Vec<String>,
    size: Option<SizeFormat>,
    ignores: Vec<String>,
    max_depth: Option<u32>,
    pub output_format: OutputFormat,
    pub git_intergration: bool,
    verbose: bool,
    pub sort: Option<SortKey>,
    pub mode: Mode,
}

#[derive(Serialize, Debug)]
pub struct TreeNode {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    git_status: Option<String>,
    children: Option<Vec<TreeNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<size::Unit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vervose_info: Option<MetaDataInfo>,
}

impl TreeNode {
    /// ## Summary
    /// -s でサイズフラグがtrueであればsizeを表示
    /// -l でロングフラがtrueであれば詳細な情報を表示
    /// sizeとvervoseのフラグは相互排他関係
    ///
    /// ## Returns
    /// サイズまたは詳細な情報のformat
    ///
    /// ## Examples
    ///```
    ///
    ///```
    fn size_and_verbose_print_format(&self) -> ColoredString {
        let mut result = "".black();
        if let Some(size_str) = self.size.as_ref() {
            result = format!(" ({})", size_str).red();
        } else if let Some(vervose_info) = self.vervose_info.as_ref() {
            result = format!(" ({})", vervose_info).red();
        }

        result
    }

    pub fn sort(&mut self, sort_key: &SortKey) {
        match sort_key {
            SortKey::Name => self.sort_by_name(),
            SortKey::Size => self.sort_by_size(),
            SortKey::Time => self.sort_by_time(),
            SortKey::Ext => todo!(),
        }
    }

    pub fn sort_by_size(&mut self) {
        if let Some(ref mut children) = self.children {
            for child in children.iter_mut() {
                child.sort_by_size();
            }

            children.sort_by(|a, b| match (&a.size, &b.size) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(sa), Some(sb)) => sa.cmp(&sb),
            });
        }
    }

    fn sort_by_name(&mut self) {
        if let Some(ref mut children) = self.children {
            for child in children.iter_mut() {
                child.sort_by_name();
            }

            children.sort_by(|a, b| a.name.cmp(&b.name));
        }
    }

    fn sort_by_time(&mut self) {
        if let Some(ref mut children) = self.children {
            for child in children.iter_mut() {
                child.sort_by_time();
            }

            children.sort_by(|a, b| match (&a.vervose_info, &b.vervose_info) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(av), Some(bv)) => av.created.cmp(&bv.created),
            });
        }
    }
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
        let mut size = matches.get_one::<SizeFormat>("size").cloned();

        let max_depth = matches.get_one::<u32>("depth").copied();
        let output_format = if matches.get_flag("json") {
            OutputFormat::Json
        } else if matches.get_flag("markdown") {
            OutputFormat::Markdown
        } else {
            OutputFormat::Standard
        };

        let git_intergration = matches.get_flag("git");

        let mut is_verbose = matches.get_flag("long");
        let mode = matches.get_one::<Mode>("mode").cloned().unwrap();

        // TODO sortによってフラグをtrueにする処理無理矢理感があるので修正
        let sort = matches.get_one::<SortKey>("sort").cloned();
        if let Some(sortkey) = &sort {
            match sortkey {
                SortKey::Size => {
                    if size.is_none() {
                        size = Some(SizeFormat::Bytes);
                    }
                }
                SortKey::Time => {
                    if !is_verbose {
                        is_verbose = true;
                    }
                }
                _ => {}
            }
        }
        Self {
            path: path,
            extensions: exts,
            ignores: ignores,
            size: size,
            max_depth: max_depth,
            output_format: output_format,
            git_intergration: git_intergration,
            verbose: is_verbose,
            sort: sort,
            mode: mode,
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

    fn ext_filter_tokio(&self, entry: &tokio::fs::DirEntry) -> bool {
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

    fn ignore_filename_filter_tokio(&self, entry: &tokio::fs::DirEntry) -> bool {
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
    // prefixが空なの時に対応しているのは
    // 初回実行時のみ空文字でありbranchを入れると崩れるため空文字にしている
    let connector = if prefix.is_empty() {
        STR_EMPTY
    } else {
        if is_last {
            TREE_LAST_BRANCH
        } else {
            TREE_BRANCH
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

    let info = node.size_and_verbose_print_format();

    println!("{}{}{}{}", prefix, connector, colored_name, info);

    if let Some(children) = &node.children {
        let new_prefix = format!(
            "{}{}",
            prefix,
            if is_last {
                TREE_LAST_EMPTY
            } else {
                TREE_VERTICAL
            }
        );
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

    let vervose_info = if tree.verbose {
        utils::files::get_metadata(&path)
            .map_err(|e| eprintln!("ERROR: {}", e))
            .ok()
    } else {
        None
    };

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
                SizeFormat::Bytes => utils::files::get_filesize(&path)
                    .map_err(|e| eprintln!("ERROR: {}", e))
                    .ok()
                    .map(|s| size::Unit::Byte(s)),
                SizeFormat::HumanReadable => utils::files::get_human_readable_filesize(&path)
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
            vervose_info: vervose_info,
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
            vervose_info: vervose_info,
        })
    }
}

#[async_recursion]
pub async fn build_tree_async(
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

    let vervose_info = if tree.verbose {
        utils::files::get_metadata_async(&path)
            .await
            .map_err(|e| eprintln!("ERROR: {}", e))
            .ok()
    } else {
        None
    };

    if path.is_dir() {
        let mut entries = tokio::fs::read_dir(path).await.ok()?;

        let mut children = vec![];
        while let Some(entry) = entries.next_entry().await.ok()? {
            if !tree.ext_filter_tokio(&entry) {
                continue;
            }
            if !tree.ignore_filename_filter_tokio(&entry) {
                continue;
            }
            let child = build_tree_async(&entry.path(), depth + 1, tree, git_statuses).await?;
            children.push(child);
        }
        let size = if let Some(size) = &tree.size {
            match size {
                SizeFormat::Bytes => utils::files::get_filesize_async_safe(&path)
                    .await
                    .map_err(|e| eprintln!("ERROR: {}", e))
                    .ok()
                    .map(|s| size::Unit::Byte(s)),
                SizeFormat::HumanReadable => utils::files::get_human_readable_filesize_async(&path)
                    .await
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
            vervose_info: vervose_info,
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
            vervose_info: vervose_info,
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
