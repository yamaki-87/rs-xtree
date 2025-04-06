use std::str::FromStr;

use clap::{builder::PossibleValue, ValueEnum};

#[derive(Debug, Clone)]
pub enum Mode {
    Async,
    Sync,
}

impl ValueEnum for Mode {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Async, Self::Sync]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Mode::Async => Some(PossibleValue::new("a").alias("async")),
            Mode::Sync => Some(PossibleValue::new("s").alias("sync")),
        }
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" | "async" => Ok(Mode::Async),
            "s" | "sync" => Ok(Mode::Sync),
            _ => Err(format!("Invalid size format:{}", s)),
        }
    }
}
