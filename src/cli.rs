use clap::{Arg, ArgAction, Command};

use crate::{
    constatns,
    foramt::{mode::Mode, sizeformat::SizeFormat, sort::SortKey},
};

pub fn build_cli() -> Command {
    Command::new("rsxtree")
        .version("1.0")
        .about("Custome tree command written in Rust")
        .arg(
            Arg::new("path")
                .help("Target directory path")
                .default_value(".")
                .index(1),
        )
        .arg(
            Arg::new("ext")
                .short('e')
                .long("ext")
                .help("Filter files by extension")
                .action(clap::ArgAction::Append)
                .num_args(1..),
        )
        .arg(
            Arg::new("ignore")
                .short('i')
                .long("ignore")
                .help("Ignore directories or files")
                .action(clap::ArgAction::Append)
                .num_args(1..),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .help("See size")
                .value_parser(clap::value_parser!(SizeFormat))
                .conflicts_with("long"),
        )
        .arg(
            Arg::new("sort")
                .short('S')
                .long("sort")
                .help("See sort")
                .value_parser(clap::value_parser!(SortKey)),
        )
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .help("Set the max depth of tree")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .help("output json")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("markdown")
                .short('m')
                .long("md")
                .help("output markdown")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("git")
                .short('g')
                .long("git")
                .help("show git diff status")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("long")
                .short('l')
                .long("long")
                .help("show verbose infos")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("size"),
        )
        .arg(
            Arg::new("mode")
                .long("mode")
                .help("execute async or sync or parallel")
                .value_parser(clap::value_parser!(Mode))
                .default_value("sync"),
        )
        .arg(
            Arg::new("stats")
                .long("stats")
                .help("show directory stats")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Show hidden files and directories")
                .action(ArgAction::SetTrue),
        )
}
