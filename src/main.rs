use std::{collections::HashMap, path::PathBuf};

use cli::build_cli;
use foramt::output::OutputFormat;
use stats::Stats;
use tree::{
    build_tree, build_tree_async, build_tree_parallel, get_git_statuses, print_tree,
    tree_to_markdown, Tree,
};

mod cli;
pub mod constatns;
pub mod foramt;
pub mod stats;
pub mod tree;
pub mod utils;
#[tokio::main]
async fn main() {
    let matches = build_cli().get_matches();
    let tree = Tree::new(&matches);
    if cfg!(debug_assertions) {
        println!("{:?}", &tree);
    }

    let root = PathBuf::from(&tree.path);
    let git_status = if tree.git_intergration {
        get_git_statuses(&root)
    } else {
        HashMap::new()
    };

    let mut tree_node = match &tree.mode {
        foramt::mode::Mode::Async => build_tree_async(&root, 1, &tree, &git_status)
            .await
            .unwrap(),
        foramt::mode::Mode::Sync => build_tree(&root, 1, &tree, &git_status).unwrap(),
        foramt::mode::Mode::Parallel => build_tree_parallel(&root, 1, &tree, &git_status).unwrap(),
    };
    if tree.sort.is_some() {
        tree_node.sort(&tree.sort.unwrap());
    }

    match tree.output_format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&tree_node).unwrap());
        }
        OutputFormat::Standard => {
            print_tree(&tree_node, "", true);
        }
        OutputFormat::Markdown => {
            println!("{}", tree_to_markdown(&tree_node, 0));
        }
        OutputFormat::Stats => {
            let mut stats = stats::Stats::empty();
            stats.collect_stats(&tree_node);
            stats.print_stats();
        }
        _ => {
            println!("yet")
        }
    }
}
