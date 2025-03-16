use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Clone)]
pub enum Unit {
    Byte(u64),
    KByte(f32),
    MByte(f32),
    GByte(f32),
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Byte(bytes) => write!(f, "{}Bytes", bytes),
            Unit::KByte(kbytes) => write!(f, "{:.2}KB", kbytes),
            Unit::MByte(mb) => write!(f, "{:.2}MB", mb),
            Unit::GByte(gb) => write!(f, "{:.2}GB", gb),
        }
    }
}

impl Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
