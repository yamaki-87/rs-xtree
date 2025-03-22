use std::str::FromStr;

use clap::ValueEnum;

#[derive(Debug, Clone, Default)]
pub enum SizeFormat {
    Bytes,
    #[default]
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
            "b" | "bytes" => Ok(SizeFormat::Bytes),
            "h" | "humanreadable" => Ok(SizeFormat::HumanReadable),
            _ => Err(format!("Invalid size format:{}", s)),
        }
    }
}
