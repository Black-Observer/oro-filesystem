//! # ORO Filesystem
//! 
//! Filesystem access crate for `negen4`.
//! 
//! It allows you to access files from:
//! - The normal filesystem
//! - Packed Obstruction Asset Packages
//! - ZIP files (maybe)
//! 
//! mbsfar

mod config;
mod readers;

// Config re-exports
pub use config::{FilesystemConfig, FilesystemType};
pub use readers::{FilesystemError, FilesystemResult};

/// Reads the file in the indicated path and
/// returns its contents as a string
pub fn read_to_string(path: &str, config: &FilesystemConfig) -> FilesystemResult<String> {
    println!("Reading file at {} using method {:?}", config.path(path), config.fs_type());
    
    // Read files
    match config.fs_type() {
        FilesystemType::Filesystem => readers::filesystem::read_to_string(&config.path(path)),
        FilesystemType::AssetPackage => todo!(),
        FilesystemType::ZIP => todo!(),
    }
}