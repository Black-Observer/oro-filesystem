//! If we are reading from a Virtual Filesystem that is actually a
//! real directory in our system, we must make sure that the path
//! of the file we want to read is actually inside the VFS'
//! root directory.
//! 
//! This module contains tools to check that a file is actually
//! inside the specified "bounds"

use std::path::{Path, PathBuf};

use crate::{FilesystemError, FilesystemResult};

/// A structure containing the Root of a Filesystem.
/// The path specified is used to check any other path.
pub struct BoundChecker {
    root: PathBuf
}

impl BoundChecker {
    /// Constructs a Bound Checker with a specific Path
    pub fn new(root: &Path) -> FilesystemResult<Self> {
        Ok(BoundChecker {
            root: root.canonicalize().map_err(|e| FilesystemError::from(e).with_path(root.as_os_str().to_string_lossy().to_string()))?
        })
    }

    /// The same as [`BoundChecker::is_in_bounds`] but the path is expected to already be canonicalized.
    fn is_in_bounds_canonical(&self, path: &Path) -> bool {
        let ancestors = path.ancestors();
        for ancestor in ancestors {
            if ancestor == self.root {
                return true;
            }
        }
        return false;
    }

    /// Checks if a specific path is inside the directory of this Bound Checker.
    /// This function returns [`Err`] if there's any error while reading the specified
    /// `path`.
    pub fn is_in_bounds(&self, path: &Path) -> FilesystemResult<bool> {
        let canonical_path = path.canonicalize().map_err(|e| FilesystemError::from(e).with_path(path.as_os_str().to_string_lossy().to_string()))?;

        Ok(self.is_in_bounds_canonical(&canonical_path))
    }

    /// Converts the specified path into a relative path IF the path specified
    /// is inside the root directory of this [`BoundChecker`]
    pub fn get_relative_string(&self, path: &Path) -> FilesystemResult<String> {
        let canonical_path = path.canonicalize().map_err(|e| FilesystemError::from(e).with_path(path.as_os_str().to_string_lossy().to_string()))?;

        let path_str = canonical_path.as_os_str().to_string_lossy().to_string();
        let root_str = self.root.as_os_str().to_string_lossy().to_string();

        if !self.is_in_bounds_canonical(&canonical_path) {
            return Err(FilesystemError::OutOfBounds(path_str, root_str));
        }

        // INFO: This removes the `/` at the beginning of the string, you might not want this
        // TODO: Revise this later (read info above)
        Ok(path_str[root_str.len()+1..].to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{config::secure_path::BoundChecker, FilesystemResult};

    #[test]
    fn check_bounds() -> FilesystemResult<()> {
        let root = Path::new("src");
        let file_in_bounds = Path::new("src/config/secure_path.rs");
        let file_out_of_bounds = Path::new("src/../tests/filesystem/testfile.txt");
        let file_invented = Path::new("src/asdasdasdad.test");

        let invalid_bound_checker = BoundChecker::new(Path::new("invented_directory"));
        let bound_checker = BoundChecker::new(root)?;

        assert!(invalid_bound_checker.is_err());

        assert!(bound_checker.is_in_bounds(file_in_bounds)?);
        assert!(!bound_checker.is_in_bounds(file_out_of_bounds)?);
        assert!(bound_checker.is_in_bounds(file_invented).is_err()); // not found

        // Check relative paths
        let relative_in_bounds = bound_checker.get_relative_string(file_in_bounds)?;
        assert_eq!(relative_in_bounds, "config/secure_path.rs".to_string());

        Ok(())
    }
}