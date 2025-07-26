//! ORO Filesystem is Read-Only, but it allows reading a real filesystem
//! to build an Asset Package.
//! 
//! This requires an input directory that we can recursively read and an
//! output directory for the package an index

use std::path::{Path, PathBuf};

use crate::{FilesystemError, FilesystemResult};

/// Used when reading 
struct FsObjectsList {
    files: Vec<PathBuf>,
    directories: Vec<PathBuf>
}

/// Converts a path into a String. Invalid unicode will get f*cked but
/// this shouldn't happen :P
fn path_to_string(path: &Path) -> String {
    path.as_os_str().to_string_lossy().to_string()
}

/// Scans a directory and stores all subdirectories and files
/// in a FsObjectsList
fn scan_directory(directory: &Path) -> FilesystemResult<FsObjectsList> {
    if !directory.is_dir() {
        return Err(FilesystemError::IsADirectory(path_to_string(directory)))
    }

    let content = match directory.read_dir() {
        Ok(read_dir) => read_dir,
        Err(e) => return Err(FilesystemError::Generic(path_to_string(directory), e.to_string())),
    };

    let mut fs_objects_list = FsObjectsList {
        files: Vec::new(),
        directories: Vec::new()
    };

    for file in content {
        let entry = match file {
            Ok(dir_entry) => dir_entry,
            Err(_) => continue,
        };

        if let Ok(file_type) = entry.file_type() {
            // Only add when is_dir() is true and if is_file() is true.
            if file_type.is_dir() {
                fs_objects_list.directories.push(entry.path());
            }
            else if file_type.is_file() {
                fs_objects_list.files.push(entry.path());
            }
        }
    }
    Ok(fs_objects_list)
}

/// Recursively reads an input directory, builds an Asset Package and
/// saves it into the output directory.
pub fn pack(input: &Path, output: &Path) -> FilesystemResult<()> {

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::Path};

    use crate::{packager::scan_directory, FilesystemResult};

    #[test]
    fn scan_tests_directory() -> FilesystemResult<()> {
        let output = scan_directory(Path::new("tests"))?;
        
        assert!(!output.directories.is_empty(), "Tests directory should contain directories!");
        assert!(!output.files.is_empty(), "Tests directory should contain files!");

        // Tests that files are correct
        assert_eq!(output.files.len(), 1);
        assert_eq!(read_to_string(output.files.get(0).unwrap()).unwrap(), "This tests that the scanner can read files and directories".to_string());

        Ok(())
    }
}