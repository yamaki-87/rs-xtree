use clap::{Arg, Command};

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
}
