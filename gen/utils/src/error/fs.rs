use std::{fmt::Display, path::PathBuf};

/// # File System Error
#[derive(Debug, Clone, PartialEq)]
pub enum FsError {
    /// File Not Found
    FileNotFound(PathBuf),
    /// Dir Not Found
    DirNotFound(PathBuf),
    /// Read Error
    Read { path: PathBuf, reason: String },
    /// Write Error
    Write { path: PathBuf, reason: String },
    /// UnExpected Error, maybe the file or dir can not access to be read or wrote or found
    UnExpected(String),
}

impl Display for FsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FsError::FileNotFound(p) => f.write_fmt(format_args!("File: {:?} can not found", p)),
            FsError::DirNotFound(p) => f.write_fmt(format_args!("Dir: {:?} can not found", p)),
            FsError::Read { path, reason } => f.write_fmt(format_args!(
                "Read File: {:?} Error\n Reason: {}",
                path, reason
            )),
            FsError::Write { path, reason } => f.write_fmt(format_args!(
                "Write File: {:?} Error\n Reason: {}",
                path, reason
            )),
            FsError::UnExpected(reason) => f.write_fmt(format_args!("UnExpected Error: {}", reason)),
        }
    }
}
