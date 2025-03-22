use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::Serialize;

#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

#[cfg(target_os = "unix")]
use std::os::unix::fs::MetadataExt;

use super::{
    datetime::DateTimeWrap,
    size::{Unit, ONE_KELE_BYTE_F32},
};

const FOUR_DIGITS: u64 = 9999;
const SIX_DIGITS: u64 = 999999;
const NINE_DIGITS: u64 = 999999999;
const CURRENT_DIR: &str = ".";

#[derive(Debug)]
pub struct MetaDataInfo {
    pub size: u64,

    #[cfg(target_os = "unix")]
    pub owner: u32,

    #[cfg(target_os = "unix")]
    pub group: u32,

    pub created: DateTimeWrap,

    pub modified: DateTimeWrap,
}
/// ## Summary
/// パスからファイル名を取得
/// "."の場合は現在のディレクトリ名を取得
///
/// ## Parameters
/// - `path`:
///
/// ## Returns
///
/// ## Examples
///```
///
///```
pub fn get_filename<P: AsRef<Path>>(path: P) -> String {
    let p = path.as_ref();

    if let Some(os_str) = p.file_name() {
        if !os_str.is_empty() && os_str != CURRENT_DIR {
            return os_str.to_string_lossy().into_owned();
        }
    }

    match p.canonicalize() {
        Ok(abs_path) => abs_path
            .file_name()
            .map(|f| f.to_string_lossy().into_owned())
            .unwrap_or_default(),
        Err(_) => {
            // canonicalize できないときはどうするか要検討
            // ここでは空文字 "" にしておく
            String::new()
        }
    }
}

/// ## Summary
/// ディレクトリサイズを取得(再帰的)
///
/// ## Parameters
/// - `directory`: ディレクトリパス
///
/// ## Returns
/// ディレクトリサイズ or Error
///
/// ## Examples
///```
///
///```
pub fn get_filesize<P: AsRef<Path>>(directory: P) -> Result<u64> {
    let mut sum_size = 0;
    let entries = fs::read_dir(directory)?;

    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            sum_size += metadata.len();
        } else if metadata.is_dir() {
            sum_size += get_filesize(entry.path())?;
        }
    }

    Ok(sum_size)
}

pub fn get_human_readable_filesize<P: AsRef<Path>>(path: P) -> Result<Unit> {
    let size = if path.as_ref().is_file() {
        let meta = path.as_ref().metadata()?;
        meta.len()
    } else {
        get_filesize(path)?
    };
    let float_size = size as f32;
    if size <= FOUR_DIGITS {
        Ok(Unit::Byte(size))
    } else if size <= SIX_DIGITS {
        let kb_size = float_size / ONE_KELE_BYTE_F32;
        Ok(Unit::KByte(kb_size))
    } else if size <= NINE_DIGITS {
        let mb_size = float_size / (ONE_KELE_BYTE_F32.powi(2));
        Ok(Unit::MByte(mb_size))
    } else {
        let gb_size = float_size / ONE_KELE_BYTE_F32.powi(3);
        Ok(Unit::GByte(gb_size))
    }
}

#[cfg(target_os = "windows")]
pub fn get_metadata<P: AsRef<Path>>(path: P) -> Result<MetaDataInfo> {
    use std::time::SystemTime;

    let metadata = path.as_ref().metadata()?;
    let size = if metadata.is_dir() {
        get_filesize(path).unwrap_or_default()
    } else if metadata.is_file() {
        metadata.len()
    } else {
        // symbolicは0
        0
    };
    let created = metadata.created().unwrap_or(SystemTime::UNIX_EPOCH);
    let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);

    Ok(MetaDataInfo {
        size,
        created: DateTimeWrap::from(created),
        modified: DateTimeWrap::from(modified),
    })
}

#[cfg(target_os = "unix")]
pub fn get_metadata<P: AsRef<Path>>(path: P) -> Result<MetaDataInfo> {
    let metadata = fs::metadata(path)?;
    Ok(MetaDataInfo {
        size: metadata.len(),
        created: DateTimeWrap::from(metadata.ctime()),
        modified: DateTimeWrap::from(metadata.mtime()),
        owner: metadata.uid(),
        group: metadata.gid(),
    })
}

impl fmt::Display for MetaDataInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(target_os = "unix")]
        {
            write!(
                f,
                "Size: {} bytes | Created: {} | Modified: {} | Owner: {} | Group: {}",
                self.size,
                self.created.yyyy_mm_dd_format(),
                self.modified.yyyy_mm_dd_format(),
                self.owner,
                self.group
            )
        }

        #[cfg(target_os = "windows")]
        {
            write!(
                f,
                "Size: {} bytes | Created: {} | Modified: {}",
                self.size,
                self.created.yyyy_mm_dd_format(),
                self.modified.yyyy_mm_dd_format()
            )
        }
    }
}

impl Serialize for MetaDataInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_filename_returns_filename() {
        let fname = get_filename(".");
        assert_eq!("rxtree", fname);
    }
}
