use std::{ffi::OsString, fs, path::{Path}};

use crate::{FilesystemError, FilesystemResult};

/// Type of filesystem that we want to access.
#[derive(Debug, PartialEq)]
pub enum FilesystemType {
    Filesystem,
    AssetPackage,
    Aura
}

/// Configuration for Obstruction Filesystem. It has two parameters:
/// - `root`: Relative (starting at executable's directory) path to the "root" of the virtual filesystem. `./` by default.
/// - `indices`: The name of the indices file at the root directory. None in [`FilesystemType::Filesystem`] configurations.
/// - `fs_type`: An enum with the type of filesystem (normal, asset package or zip). Autodetected.
#[derive(Debug)]
pub struct FilesystemConfig {
    root: String,
    indices: Option<OsString>,
    fs_type: FilesystemType
}

impl FilesystemConfig {
    /// Constructs a default config with nothing as the root and an
    /// automatically detected filesystem type.
    pub fn new() -> FilesystemResult<Self> {
        let root = Self::get_usable_root("");
        let (fs_type, indices) = Self::autodetect_filesystem(&root)?;
        Ok(FilesystemConfig { root, indices, fs_type })
    }
    /// Constructs a configuration object with a root and an
    /// automatically detected filesystem type (if expecting `AssetPackage`,
    /// the `index.oapi` should be in the `root` folder).
    pub fn with_root(root: &str) -> FilesystemResult<Self> {
        let root = Self::get_usable_root(root);
        let (fs_type, indices) = Self::autodetect_filesystem(&root)?;
        Ok(FilesystemConfig { root, indices, fs_type })
    }

    /// Returns the Filesystem Type in this config
    pub fn fs_type(&self) -> &FilesystemType {
        &self.fs_type
    }
    /// Returns the Root of this config. What the root is depends on the type of filesystem:
    /// 
    /// - [`FilesystemType::Filesystem`]: Path to the root directory of that virtual filesystem.
    /// - [`FilesystemType::AssetPackage`]: Path to the Indices JSON file.
    /// - [`FilesystemType::Aura`]: Path to the Aura JSON file.
    pub fn root(&self) -> String {
        self.root.clone()
    }
    /// Generates a path by concatenating the root to the
    /// path passed as parameter
    pub fn path(&self, path: &str) -> String {
        let c = self.root.clone();
        c + path
    }


    /// If a `*.oapi` file exists, [`FilesystemType::AssetPackage`] is selected,
    /// if a `*.aura` file exists, [`FilesystemType::Aura`] is selected.
    /// if no indices file is found, [`FilesystemType::Filesystem`] is selected
    /// 
    /// Returns a [`FilesystemType`] and an [`Option<OsString>`] with the name of the indices file
    fn autodetect_filesystem(root: &str) -> FilesystemResult<(FilesystemType, Option<OsString>)> {
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
                let file_path_string = file_path.as_os_str().to_os_string();

                if ext == "oapi" {
                    return Ok((FilesystemType::AssetPackage, Some(file_path_string)))
                } else if ext == "aura" {
                    return Ok((FilesystemType::Aura, Some(file_path_string)))
                }
            }
        }
        
        return Ok((FilesystemType::Filesystem, None));
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
        assert_eq!(*configuration.fs_type(), FilesystemType::AssetPackage);
        Ok(())
    }

    #[test]
    fn check_aura_detection() -> FilesystemResult<()> {
        let configuration = FilesystemConfig::with_root("tests/aura")?;
        assert_eq!(*configuration.fs_type(), FilesystemType::Aura);
        Ok(())
    }

    #[test]
    fn check_filesystem_detection() -> FilesystemResult<()> {
        let configuration = FilesystemConfig::with_root("     tests    ")?; // <- trimmed
        assert_eq!(*configuration.fs_type(), FilesystemType::Filesystem);
        Ok(())
    }
}