use std::{collections::HashMap, fmt::Display, path::Path};

use crate::tree::TreeNode;

pub struct Stats(HashMap<String, StatsData>);

#[derive(Default)]
pub struct StatsData {
    count: u64,
    size: f64,
}

impl Display for StatsData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<9}{} Bytes", self.count, self.size)
    }
}
impl Stats {
    const EMPTY_KEY: &str = "(no ext)";

    /// ## Summary
    /// 空のHashMapで初期化
    pub fn empty() -> Self {
        Self(HashMap::new())
    }

    pub fn collect_stats(&mut self, node: &TreeNode) {
        if let Some(children) = node.get_children() {
            for child in children {
                self.collect_stats(child);
            }
        }

        let path = Path::new(node.get_name());
        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(ext) => ext,
            None => {
                if node.get_children().is_some() {
                    return;
                }
                Self::EMPTY_KEY
            }
        };

        let entry = self
            .0
            .entry(ext.to_string())
            .or_insert(StatsData::default());
        entry.count += 1;

        if let Some(size) = &node.get_size() {
            entry.size += size.to_bytes_f64();
        }
    }

    pub fn print_stats(&self) {
        println!("{}", HEADER);
        println!("{}", self);
    }
}

const HEADER: &str = r#"Extension    Count    Total Size
--------------------------------"#;

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = self
            .0
            .iter()
            .map(|(k, v)| format!("{:<13}{}", k, v))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", body)
    }
}

#[cfg(test)]
mod test {
    use crate::{tree::print_tree, utils::size};

    use super::*;
    fn file(name: &str, size: u64) -> TreeNode {
        TreeNode::new(name.into(), None, None, Some(size::Unit::Byte(size)), None)
    }

    fn dir(name: &str, children: Vec<TreeNode>) -> TreeNode {
        TreeNode::new(name.into(), None, Some(children), None, None)
    }

    #[test]
    fn test_single_rs_file_stats() {
        let mut stats = Stats::empty();

        let tree_node = file("main.rs", 1000);

        stats.collect_stats(&tree_node);

        assert_eq!(stats.0.get("rs").unwrap().count, 1);
        assert_eq!(stats.0.get("rs").unwrap().size, 1000.0);
    }

    #[test]
    fn test_file_without_extension() {
        let mut stats = Stats::empty();

        let tree_node = file("LOCK", 2048);
        stats.collect_stats(&tree_node);

        assert_eq!(stats.0.get(Stats::EMPTY_KEY).unwrap().count, 1);
        assert_eq!(stats.0.get(Stats::EMPTY_KEY).unwrap().size, 2048.0);
    }

    #[test]
    fn test_directory_node_is_ignored() {
        let mut stats = Stats::empty();

        let tree_node = dir("test", vec![]);
        stats.collect_stats(&tree_node);

        assert!(stats.0.is_empty());
    }

    #[test]
    fn test_mutiple_node_stats() {
        let f1 = file("test.rs", 1048);
        let f2 = file("util.rs", 2048);
        let nested_f1 = file("mod.rs", 500);
        let nested_f2 = file("config.toml", 500);
        let nested = dir("nested", vec![nested_f1, nested_f2]);
        let root = dir("src", vec![f1, f2, nested]);

        let mut stats = Stats::empty();
        stats.collect_stats(&root);

        assert_eq!(stats.0.get("rs").unwrap().count, 3);
        assert_eq!(stats.0.get("rs").unwrap().size, 3596.);

        assert_eq!(stats.0.get("toml").unwrap().count, 1);
        assert_eq!(stats.0.get("toml").unwrap().size, 500.);

        assert!(stats.0.get(Stats::EMPTY_KEY).is_none());
    }
}
