use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use async_recursion::async_recursion;
use serde::Serialize;

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

use super::{
    datetime::DateTimeWrap,
    size::{Unit, ONE_KELE_BYTE_F32},
};

const CURRENT_DIR: &str = ".";

#[derive(Debug)]
pub struct MetaDataInfo {
    pub size: u64,

    #[cfg(unix)]
    pub owner: u32,

    #[cfg(unix)]
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

#[async_recursion(?Send)]
/// ## Summary
/// async用ファイルサイズ取得関数
///
/// ## Note
/// この関数はマルチスレッドで使わないでください。
/// マルチスレッド用に設計されてないです
///
/// ## Parameters
/// - `directory`:
///
/// ## Returns
///
/// ## Examples
///```
///
///```
pub async fn get_filesize_async_unsafe<P: AsRef<Path>>(directory: P) -> Result<u64> {
    let mut sum_size = 0;
    let mut entries = tokio::fs::read_dir(directory).await?;

    while let Some(entry) = entries.next_entry().await? {
        let metadata = entry.metadata().await?;
        if metadata.is_file() {
            sum_size += metadata.len();
        } else if metadata.is_dir() {
            sum_size += get_filesize(entry.path())?;
        }
    }

    Ok(sum_size)
}

/// ## Summary
/// async用ファイルサイズ取得関数
///
/// ## Note
/// この関数はマルチスレッド用テスト
///
/// ## Parameters
/// - `directory`:
///
/// ## Returns
///
/// ## Examples
///```
///
///```
pub async fn get_filesize_async_safe<P: AsRef<Path>>(directory: P) -> Result<u64> {
    let start = directory.as_ref();
    let mut sum_size = 0u64;
    let mut stack = vec![start.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let mut entries = match tokio::fs::read_dir(&dir).await {
            Ok(e) => e,
            Err(e) => return Err(e.into()),
        };

        while let Ok(Some(entry)) = entries.next_entry().await {
            let metadata = entry.metadata().await?;
            if metadata.is_file() {
                sum_size += metadata.len();
            } else if metadata.is_dir() {
                stack.push(entry.path());
            }
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

    Ok(Unit::new(size))
}

pub async fn get_human_readable_filesize_async<P: AsRef<Path>>(path: P) -> Result<Unit> {
    let size = if path.as_ref().is_file() {
        let meta = tokio::fs::metadata(path).await?;
        meta.len()
    } else {
        get_filesize_async_safe(path).await?
    };

    Ok(Unit::new(size))
}

#[cfg(windows)]
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

#[cfg(windows)]
pub async fn get_metadata_async<P: AsRef<Path>>(path: P) -> Result<MetaDataInfo> {
    use std::time::SystemTime;

    let p = path.as_ref();
    let metadata = tokio::fs::metadata(p).await?;
    let size = if metadata.is_dir() {
        get_filesize(p).unwrap_or_default()
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

#[cfg(unix)]
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

#[cfg(unix)]
pub async fn get_metadata_async<P: AsRef<Path>>(path: P) -> Result<MetaDataInfo> {
    let metadata = tokio::fs::metadata(path).await?;
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
        #[cfg(unix)]
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

        #[cfg(windows)]
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

    // #[test]
    // fn get_filename_returns_filename() {
    //     let fname = get_filename(".");
    //     assert_eq!("rxtree", fname);
    // }
}
