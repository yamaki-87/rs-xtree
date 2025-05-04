use std::str::FromStr;

use clap::{builder::PossibleValue, ValueEnum};

#[derive(Debug, Clone)]
pub enum Mode {
    Async,
    Sync,
    Parallel,
}

impl ValueEnum for Mode {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Sync, Self::Parallel]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Mode::Async => Some(PossibleValue::new("a").alias("async")),
            Mode::Sync => Some(PossibleValue::new("s").alias("sync")),
            Mode::Parallel => Some(PossibleValue::new("p").alias("parallel")),
        }
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" | "async" => Ok(Mode::Async),
            "s" | "sync" => Ok(Mode::Sync),
            "p" | "parallel" => Ok(Mode::Parallel),
            _ => Err(format!("Invalid size format:{}", s)),
        }
    }
}
