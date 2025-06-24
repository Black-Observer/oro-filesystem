//! # ORO Filesystem
//! 
//! Filesystem access crate for `negen4`.
//! 
//! It allows you to access files from:
//! - The normal filesystem
//! - Packed Obstruction Asset Packages
//! - Aura files (Web-based file maps)

mod config;
mod readers;

// Config re-exports
pub use config::{FilesystemConfig, FilesystemType};
pub use readers::{FilesystemError, FilesystemResult};

/// Reads the file in the indicated path and
/// returns its contents as a string
pub fn read_to_string(path: &str, config: &FilesystemConfig) -> FilesystemResult<String> {
    // Read files
    match config.fs_type() {
        FilesystemType::Filesystem => {
            println!("Reading file at v-filesystem path {} (real fs {})", path, config.path(path));
            readers::filesystem::read_to_string(&config.path(path))
        }
        FilesystemType::Indexed => {
            println!("Reading file at v-filesystem path {} (from indexed fs {})", path, config.root());
            todo!()
        }
    }
}