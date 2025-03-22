use std::str::FromStr;

use clap::{builder::PossibleValue, ValueEnum};

#[derive(Clone, Debug)]
pub enum SortKey {
    Name,
    Size,
    Time,
    Ext,
}

impl ValueEnum for SortKey {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Name, Self::Size, Self::Time, Self::Ext]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            SortKey::Name => Some(PossibleValue::new("n").alias("name")),
            SortKey::Size => Some(PossibleValue::new("s").alias("size")),
            SortKey::Time => Some(PossibleValue::new("t").alias("time")),
            SortKey::Ext => Some(PossibleValue::new("e").alias("ext")),
        }
    }
}

impl FromStr for SortKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" | "name" => Ok(SortKey::Name),
            "s" | "size" => Ok(SortKey::Size),
            "t" | "tiem" => Ok(SortKey::Time),
            "e" | "ext" => Ok(SortKey::Ext),
            _ => Err(format!("Invalid size format:{}", s)),
        }
    }
}
