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
            // Does this path have an index?
            match config.get_index_for_file(path) {
                Ok(index) => {
                    // Is this index an AssetPackage or an Aura file?
                    match index {
                        config::index::IndexType::AssetPack(asset_pack_index) => readers::assetpackage::read_to_string(path, &config.root(), &asset_pack_index),
                        config::index::IndexType::Aura(aura_index) => todo!(),
                    }
                },
                Err(e) => Err(e),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{read_to_string, FilesystemConfig, FilesystemResult};

    /// not much to test here
    #[test]
    fn read_from_real_filesystem() -> FilesystemResult<()> {
        let config = FilesystemConfig::new()?;
        let _contents = read_to_string("README.md", &config)?;
        Ok(())
    }

    #[test]
    fn read_from_asset_pack() -> FilesystemResult<()> {
        let config = FilesystemConfig::with_root("tests/assetpackage")?;
        let contents_f1 = read_to_string("virtualFolder/vfile1.txt", &config)?;
        let contents_f2 = read_to_string("virtualFolder/vfile1-copy.txt", &config)?;
        let contents_f3 = read_to_string("otherFolder/someScript.lua", &config)?;

        assert_eq!(contents_f1, "hello, world! This is a test");
        assert_eq!(contents_f2, "hello, world! This is a test");
        assert_eq!(contents_f3, "When The imposter is sus!! This is a script or something.");
        Ok(())
    }
}