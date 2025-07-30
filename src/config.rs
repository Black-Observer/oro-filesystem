use std::{fs, path::{Path}};

use crate::{config::{assetmap::AssetMap, index::{IndexFile, IndexType}}, FilesystemError, FilesystemResult};

pub mod index;
pub mod assetmap;
pub mod secure_path;

/// Type of filesystem that we want to access.
#[derive(Debug, PartialEq)]
pub enum FilesystemType {
    Filesystem,
    Indexed
}

/// Configuration for Obstruction Filesystem. It has three attributes:
/// - `root`: Relative (starting at executable's directory) path to the "root" of the virtual filesystem. `./` by default.
/// - `index`: The deserialized indices file. None in [`FilesystemType::Filesystem`] configurations, Some in any Indexed configuration (Aura or AssetPackage)
#[derive(Debug)]
pub struct FilesystemConfig {
    root: String,
    index: Option<AssetMap>
}

impl FilesystemConfig {
    /// Constructs a default config with nothing as the root and an
    /// automatically detected filesystem type.
    pub fn new() -> FilesystemResult<Self> {
        let root = Self::get_usable_root("");
        let index = Self::autodetect_filesystem(&root)?;
        Ok(FilesystemConfig { root, index })
    }
    /// Constructs a configuration object with a root and an
    /// automatically detected filesystem type.
    pub fn with_root(root: &str) -> FilesystemResult<Self> {
        let root = Self::get_usable_root(root);
        let index = Self::autodetect_filesystem(&root)?;
        Ok(FilesystemConfig { root, index })
    }
    
    /// Returns the type of Filesystem in this configuration
    pub fn fs_type(&self) -> FilesystemType {
        match self.index {
            Some(_) => FilesystemType::Indexed,
            None => FilesystemType::Filesystem,
        }
    }
    /// Returns the path to the virtual filesystem.
    pub fn root(&self) -> String {
        self.root.clone()
    }
    /// Generates a path by concatenating the root to the
    /// path passed as parameter
    pub fn path(&self, path: &str) -> String {
        let c = self.root.clone();
        c + path
    }

    /// Returns the index information for a file.   
    /// It simply returns a Value in the [`AssetMap`] for the
    /// Key passed as a parameter.
    /// 
    /// This can fail if a filesystem is unindexed (like the native filesystem),
    /// or if the file is not found.
    pub fn get_index_for_file(&self, path: &str) -> FilesystemResult<IndexType> {
        match &self.index {
            Some(asset_map) => {
                match asset_map.get(path) {
                    Some(index) => Ok(index.clone()),
                    None => Err(FilesystemError::NotFound(path.to_string())),
                }
            }
            None => Err(FilesystemError::UnindexedFilesystem(path.to_string())),
        }
    }

    /// If a `*.oroi` file exists, the indices are read and returned,
    /// if no indices file is found, an index configuration of [`None`] is returned.
    /// 
    /// An index configuration of [`Some`] indicates that the filesystem is Indexed (Aura or AssetPackage),
    /// an index configuration of [`None`] indicates that it is Unindexed (Native Filesystem)
    fn autodetect_filesystem(root: &str) -> FilesystemResult<Option<AssetMap>> {
        let path = Path::new(root);
        let files = match fs::read_dir(path) {
            Ok(f) => f,
            Err(e) => return Err(FilesystemError::Generic(root.to_string(), e.to_string())),
        };
        for file in files {
            let entry = match file {
                Ok(e) => e,
                Err(_) => continue,
            };

            let file_path = entry.path();
            
            if let Some(ext) = file_path.extension() {
                if ext == "oroi" {
                    let index_file = IndexFile::from_file(&file_path)?;
                    let asset_map= AssetMap::try_from(index_file)?;

                    return Ok(Some(asset_map))
                }
            }
        }
        
        return Ok(None);
    }

    fn get_usable_root(root: &str) -> String {
        let trimmed_root = root.trim();

        if trimmed_root.is_empty() {
            return String::from("./");
        }

        if trimmed_root.ends_with('/') {
            trimmed_root.to_string()
        } else {
            let mut c = trimmed_root.to_string();
            c.push('/');
            c
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FilesystemConfig, FilesystemResult, FilesystemType};

    #[test]
    fn check_assetpackage_detection() -> FilesystemResult<()> {
        let configuration = FilesystemConfig::with_root("tests/assetpackage")?;
        assert_eq!(configuration.fs_type(), FilesystemType::Indexed);
        Ok(())
    }

    #[test]
    fn check_aura_detection() -> FilesystemResult<()> {
        let configuration = FilesystemConfig::with_root("tests/aura")?;
        assert_eq!(configuration.fs_type(), FilesystemType::Indexed);
        Ok(())
    }

    #[test]
    fn check_filesystem_detection() -> FilesystemResult<()> {
        let configuration = FilesystemConfig::with_root("     tests    ")?; // <- should be trimmed
        assert_eq!(configuration.fs_type(), FilesystemType::Filesystem);
        Ok(())
    }
}