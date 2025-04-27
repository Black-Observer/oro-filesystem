//! # Obstruction Filesystem
//! 
//! mbsfar

mod config;

// Config re-exports
pub use config::{FilesystemConfig, FilesystemType};

/// Reads the file in the indicated path and
/// returns its contents as a string
pub fn read_to_string(path: &str, config: &FilesystemConfig) -> String {
    println!("Reading file at {} using method {:?}", config.path(path), config.fs_type());
    todo!()
}