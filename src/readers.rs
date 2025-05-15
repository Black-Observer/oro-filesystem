use std::{error::Error, fmt::Display};

/// Any error that could happen while reading from the filesystem.
/// 
/// It is a subset of the [`std::io::ErrorKind`] enum that only includes
/// common Reading errors
#[derive(Debug)]
pub enum FilesystemError {
    /// Refer to [`std::io::ErrorKind::NotFound`]
    NotFound(String),
    /// Refer to [`std::io::ErrorKind::PermissionDenied`]
    PermissionDenied(String),
    /// Refer to [`std::io::ErrorKind::BrokenPipe`]
    BrokenPipe(String),
    /// Refer to [`std::io::ErrorKind::NotADirectory`]
    NotADirectory(String),
    /// Refer to [`std::io::ErrorKind::IsADirectory`]
    IsADirectory(String),
    /// Refer to [`std::io::ErrorKind::UnexpectedEof`]
    UnexpectedEof(String),
    /// Refer to [`std::io::ErrorKind::OutOfMemory`]
    OutOfMemory(String),
    /// Any other type of error that I didn't want to add into this enum.   
    /// The first parameter is the path, the second one is the actual error's `to_string()`
    Generic(String, String)
}
impl Display for FilesystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilesystemError::NotFound(path) => write!(f, "Couldn't find file at \"{path}\""),
            FilesystemError::PermissionDenied(path) => write!(f, "You don't have permissions to read \"{path}\""),
            FilesystemError::BrokenPipe(path) => write!(f, "Apparently your pipes are broken or whatever. \"{path}\""),
            FilesystemError::NotADirectory(path) => write!(f, "The \"directory\" was not a directory \"{path}\""),
            FilesystemError::IsADirectory(path) => write!(f, "The \"file\" was actually secretly a directory \"{path}\""),
            FilesystemError::UnexpectedEof(path) => write!(f, "Unexpected end of file in file \"{path}\""),
            FilesystemError::OutOfMemory(path) => write!(f, "Can't load this chunky-ass file (out of memory): \"{path}\""),
            FilesystemError::Generic(path, reason) => write!(f, "Couldn't read \"{path}\".{}", if reason.is_empty() {String::new()} else {String::from(" Reason: ")+ reason}),
        }
    }
}
impl Error for FilesystemError {}

/// The result of reading from the filesystem.   
/// It holds either the data expected from the operation
/// or a [FilesystemError].
pub type FilesystemResult<T> = Result<T, FilesystemError>;

pub mod filesystem;