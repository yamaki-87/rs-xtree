use std::fmt::Display;

use serde::Serialize;

pub const ONE_KELE_BYTE_F32: f32 = 1024.0;
pub const ONE_KELE_BYTE_F64: f64 = 1024.0;

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

impl Unit {
    fn to_bytes_f64(&self) -> f64 {
        match self {
            Unit::Byte(b) => *b as f64,
            Unit::KByte(kb) => *kb as f64 * ONE_KELE_BYTE_F64,
            Unit::MByte(mb) => *mb as f64 * ONE_KELE_BYTE_F64.powi(2),
            Unit::GByte(gb) => *gb as f64 * ONE_KELE_BYTE_F64.powi(3),
        }
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes_f64() == other.to_bytes_f64()
    }
}

impl Eq for Unit {}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_bytes_f64().partial_cmp(&other.to_bytes_f64())
    }
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // TODO f64の比較はNanがあるとpaincを起こすのでEqualで一応対策
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}
