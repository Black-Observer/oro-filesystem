use std::{error::Error, fmt::Display};

/// Any error that could happen while reading from the filesystem.
/// 
/// It contains a subset of the [`std::io::ErrorKind`] enum that only includes
/// common Reading errors as well as some custom errors specific to functions of
/// this library.
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
    /// An error that occurred while reading a serialized file.  
    /// The parameter is the error message obtained from [`serde_json`].
    DeserializationError(String),
    /// An error that occurred while serializing a file.  
    /// The parameter is the error message obtained from [`serde_json`].
    SerializationError(String),
    /// An index file contained multiple definitions for one single
    /// file
    DuplicatePathsInIndex(String),
    /// Attempted to get index information from an unindexed filesystem (Native Filesystem)
    UnindexedFilesystem(String),
    /// Any error that happens during the fetch of a web resource. The first parameter is the URL
    /// and the second one is the error message.
    FetchError(String, String),
    /// Attempted to read a file or directory that should be inside a specific directory
    /// but isn't. This can be as simple as "the user tried to read the filesystem root" but
    /// it can also be triggered by trying to access parent directories with "..".
    OutOfBounds(String, String),
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
            FilesystemError::DuplicatePathsInIndex(path) => write!(f, "Duplicate path found in index file: {path}"),
            FilesystemError::DeserializationError(message) => write!(f, "Couldn't deserialize. Obtained error: {message}"),
            FilesystemError::SerializationError(message) => write!(f, "Couldn't serialize. Obtained error: {message}"),
            FilesystemError::UnindexedFilesystem(path) => write!(f, "Couldn't obtain index for file at \"{path}\". Filesystem is unindexed"),
            FilesystemError::FetchError(url, errormsg) => write!(f, "Couldn't fetch web resource at \"{url}\". Reason: {errormsg}"),
            FilesystemError::OutOfBounds(path, root) => write!(f, "Can't access \"{path}\". Resource outside directory \"{root}\""),
            FilesystemError::Generic(path, reason) => write!(f, "Couldn't read \"{path}\".{}", if reason.is_empty() {String::new()} else {String::from(" Reason: ")+ reason}),
        }
    }
}
impl Error for FilesystemError {}

impl From<std::io::Error> for FilesystemError {
    fn from(value: std::io::Error) -> Self {
        let path = String::from("[unspecified]"); // dummy
        match value.kind() {
            std::io::ErrorKind::NotFound => FilesystemError::NotFound(path),
            std::io::ErrorKind::PermissionDenied => FilesystemError::PermissionDenied(path),
            std::io::ErrorKind::BrokenPipe => FilesystemError::BrokenPipe(path),
            std::io::ErrorKind::NotADirectory => FilesystemError::NotADirectory(path),
            std::io::ErrorKind::IsADirectory => FilesystemError::IsADirectory(path),
            std::io::ErrorKind::UnexpectedEof => FilesystemError::UnexpectedEof(path),
            std::io::ErrorKind::OutOfMemory => FilesystemError::OutOfMemory(path),
            _ => FilesystemError::Generic(path, value.to_string()),
        }
    }
}

impl FilesystemError {
    /// When using [`From`] to convert from [`std::io::Error`] to [`FilesystemError`],
    /// the path is not known. You can use this function to add it after converting.
    pub fn with_path(self, path: String) -> Self {
        match self {
            FilesystemError::NotFound(_) => FilesystemError::NotFound(path),
            FilesystemError::PermissionDenied(_) => FilesystemError::PermissionDenied(path),
            FilesystemError::BrokenPipe(_) => FilesystemError::BrokenPipe(path),
            FilesystemError::NotADirectory(_) => FilesystemError::NotADirectory(path),
            FilesystemError::IsADirectory(_) => FilesystemError::IsADirectory(path),
            FilesystemError::UnexpectedEof(_) => FilesystemError::UnexpectedEof(path),
            FilesystemError::OutOfMemory(_) => FilesystemError::OutOfMemory(path),
            FilesystemError::Generic(_, value) => FilesystemError::Generic(path, value),
            _ => self
        }
    }
}

/// The result of reading from the filesystem.   
/// It holds either the data expected from the operation
/// or a [FilesystemError].
pub type FilesystemResult<T> = Result<T, FilesystemError>;

pub mod filesystem;
pub mod assetpackage;
pub mod aura;