use std::path::Path;

/// Type of filesystem that we want to access.
#[derive(Debug)]
pub enum FilesystemType {
    Filesystem,
    AssetPackage,
    ZIP
}

/// Configuration for Obstruction Filesystem. It has two parameters:
/// - `root`: Relative path to the "root" folder. Empty by default.
/// - `fs_type`: An enum with the type of filesystem (normal, asset package or zip). Autodetected by default.
pub struct FilesystemConfig {
    root: String,
    fs_type: FilesystemType
}

impl FilesystemConfig {
    /// Constructs a default config with nothing as the root and an
    /// automatically detected filesystem type.
    pub fn new() -> Self {
        let root = "".to_string();
        let fs_type = Self::autodetect_filesystem(&root);
        FilesystemConfig { root, fs_type }
    }
    /// Constructs a configuration object with a root and an
    /// automatically detected filesystem type (if expecting `AssetPackage`,
    /// the `index.oapi` should be in the `root` folder).
    pub fn with_root(root: &str) -> Self {
        let root = Self::get_usable_root(root);
        let fs_type = Self::autodetect_filesystem(&root);
        FilesystemConfig { root, fs_type }
    }
    /// Constructs a configuration object with a root and a filesystem type
    pub fn with_root_and_fs_type(root: &str, fs_type: FilesystemType) -> Self {
        let root = Self::get_usable_root(root);
        FilesystemConfig { root, fs_type }
    }
    /// Constructs a configuration object a filesystem type and nothing as the root
    pub fn with_fs_type(fs_type: FilesystemType) -> Self {
        let root = "".to_string();
        FilesystemConfig { root, fs_type }
    }

    /// Returns the Filesystem Type in this config
    pub fn fs_type(&self) -> &FilesystemType {
        &self.fs_type
    }
    /// Generates a path by concatenating the root to the
    /// path passed as parameter
    pub fn path(&self, path: &str) -> String {
        let c = self.root.clone();
        c + path
    }


    /// If an `index.oapi` file exists, AssetPackage is selected,
    /// if it doesn't, Filesystem is selected
    fn autodetect_filesystem(root: &str) -> FilesystemType {
        if Path::new(&(root.to_string()+"index.oapi")).exists() {
            return FilesystemType::AssetPackage;
        }
        
        return FilesystemType::Filesystem;
    }

    fn get_usable_root(root: &str) -> String {
        if root.is_empty() {
            return String::new();
        }

        if root.ends_with('/') {
            root.to_string()
        } else {
            let mut c = root.to_string();
            c.push('/');
            c
        }
    }
}