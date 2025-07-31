//! ORO Filesystem is Read-Only, but it allows reading a real filesystem
//! to build an Asset Package.
//! 
//! This requires an input directory that we can recursively read and an
//! output directory for the package an index

use std::{fs::{self, File, OpenOptions}, io::{self, BufReader, BufWriter, Write}, path::{Path, PathBuf}};

use crate::{config::{assetmap::AssetMap, index::{AssetPackIndex, IndexFile, IndexType}, secure_path::BoundChecker}, FilesystemError, FilesystemResult};

/// Used when reading 
struct FsObjectsList {
    files: Vec<PathBuf>,
    directories: Vec<PathBuf>
}

struct OutputPackageFile {
    path: PathBuf,
    writer: BufWriter<File>,
    current_size: u64
}

impl OutputPackageFile {
    pub fn new(path: &Path) -> FilesystemResult<Self> {
        Self::delete_file(path)?;

        let destination = OpenOptions::new()
            .write(true)
            .append(true)
            .create_new(true)
            .open(path)
            .map_err(|e| FilesystemError::from(e).with_path(path_to_string(path)))?
        ;
        let writer = BufWriter::new(destination);

        Ok(
            OutputPackageFile {
                path: path.to_path_buf(),
                current_size: 0,
                writer
            }
        )
    }

    /// If the file already exists, deletes it.
    fn delete_file(path: &Path) -> FilesystemResult<()> {
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    } 
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

    let content = directory.read_dir().map_err(|e| 
        FilesystemError::from(e).with_path(path_to_string(directory))
    )?;

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

/// Scans a directory recursively and returns paths to all the files inside
/// that directory and its subdirectories
fn scan_directory_recursively(directory: &Path) -> FilesystemResult<Vec<PathBuf>> {
    let mut dir_objects = scan_directory(directory)?;
    for dir in dir_objects.directories {
        dir_objects.files.append(&mut scan_directory_recursively(&dir)?);
    }
    Ok(dir_objects.files)
}

/// Appends the contents of a file into a destination file and registers
/// the file in the provided [`AssetMap`].
fn append_file(bound_checker: &BoundChecker, input_file: &Path, output_file: &mut OutputPackageFile, asset_map: &mut AssetMap) -> FilesystemResult<()> {
    // We propagate the error because filesystems with out of bounds files are unsafe.
    // This function already checks for bounds so if the function continues it means the file is in bounds.
    let relative_path = bound_checker.get_relative_string(input_file)?;

    // write to output file
    let source = File::open(input_file).map_err(|e| FilesystemError::from(e).with_path(path_to_string(input_file)))?;
    let file_size = source
        .metadata()
        .map_err(|e| FilesystemError::from(e).with_path(path_to_string(input_file)))?
        .len();

    let mut reader = BufReader::new(source);

    // Get starting index first
    let starting_index = output_file.current_size;

    io::copy(&mut reader, &mut output_file.writer).map_err(|e| FilesystemError::from(e).with_path(path_to_string(&output_file.path)))?;
    output_file.writer.flush().map_err(|e| FilesystemError::from(e).with_path(path_to_string(&output_file.path)))?;

    // If everything was right we can register the Asset Map Entry and update the file size
    let index_type = IndexType::AssetPack(AssetPackIndex {
        // for the package we only want the package name, we expect the index and package to be in the same place
        package: output_file.path.file_name().expect("Shouldn't fail, we already wrote to this file").to_string_lossy().to_string(),
        file_size,
        starting_index
    });
    asset_map.insert(relative_path, index_type);
    output_file.current_size += file_size;
    
    Ok(())
}

/// Recursively reads an input directory, builds an Asset Package and
/// saves it into the output directory.
/// 
/// The files are divided into chunks, large files are never fully
/// loaded into memory.
pub fn pack(input: &Path, output: &Path, name_no_extension: &str) -> FilesystemResult<()> {
    let bound_checker = BoundChecker::new(input)?;
    let mut asset_map = AssetMap::new();

    let files = scan_directory_recursively(input)?;
    
    // Create output file
    let package_name = name_no_extension.to_string() + ".oap";
    let mut output_file = OutputPackageFile::new(&output.join(package_name))?;

    // Create the package
    for file in files {
        append_file(&bound_checker, &file, &mut output_file, &mut asset_map)?;
    }

    // Serialize and export
    let index_file: IndexFile = asset_map.into();
    let index_file_serialized = serde_json::to_string(&index_file.files)
        .map_err(|e| FilesystemError::SerializationError(e.to_string()))?
    ;
    let index_file_path = output.join(name_no_extension.to_string() + ".oroi");
    fs::write(&index_file_path, index_file_serialized)
        .map_err(|e| FilesystemError::from(e).with_path(path_to_string(&index_file_path)))?
    ;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crate::{pack, packager::scan_directory, FilesystemConfig, FilesystemResult};

    #[test]
    fn scan_tests_directory() -> FilesystemResult<()> {
        let output = scan_directory(Path::new("tests"))?;
        
        assert!(!output.directories.is_empty(), "Tests directory should contain directories!");
        assert!(!output.files.is_empty(), "Tests directory should contain files!");

        // Tests that files are correct
        assert_eq!(output.files.len(), 1);
        assert_eq!(fs::read_to_string(output.files.get(0).unwrap()).unwrap(), "This tests that the scanner can read files and directories".to_string());

        Ok(())
    }

    #[test]
    fn pack_tests_directory() -> FilesystemResult<()> {
        // We need to create the directory if it doesn't exist
        fs::create_dir_all("tests_packed").expect("Shouldn't happen???");
        pack(Path::new("tests"), Path::new("tests_packed"), "tests_pack")?;

        let config = FilesystemConfig::with_root("tests_packed")?;
        let hello_world = crate::read_to_string("filesystem/testfile.txt", &config)?;
        let dummy_package = crate::read_to_string("assetpackage/package.oap", &config)?;

        assert_eq!(hello_world, "Hello, World!");
        assert_eq!(dummy_package, "hello, world! This is a testhello, world! This is a testWhen The imposter is sus!! This is a script or something.Hello, World!");

        Ok(())
    }
}