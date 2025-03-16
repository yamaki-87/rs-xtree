use std::{collections::HashMap, path::PathBuf};

use cli::build_cli;
use tree::{build_tree, get_git_statuses, print_tree, tree_to_markdown, OutputFormat, Tree};

mod cli;
pub mod constatns;
pub mod tree;
pub mod utils;
fn main() {
    let matches = build_cli().get_matches();
    let tree = Tree::new(&matches);
    println!("{:?}", &tree);

    let root = PathBuf::from(&tree.path);
    let git_status = if tree.git_intergration {
        get_git_statuses(&root)
    } else {
        HashMap::new()
    };

    let tree_node = build_tree(&root, 1, &tree, &git_status).unwrap();

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
        _ => {
            println!("yet")
        }
    }
}
