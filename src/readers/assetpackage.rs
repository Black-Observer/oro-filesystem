use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use crate::{
    config::index::AssetPackIndex, readers::io_error_to_filesystem_error, FilesystemError,
};

use super::FilesystemResult;

pub fn read_to_string(path: &str, root: &str, index: &AssetPackIndex) -> FilesystemResult<String> {
    let package_path = String::from(root) + &index.package;

    let mut package = match File::open(&package_path) {
        Ok(file) => file,
        Err(e) => return Err(io_error_to_filesystem_error(package_path, e)),
    };

    // apply file offset (which could fail and return an std::io::error)
    if let Err(e) = package.seek(SeekFrom::Start(index.starting_index)) {
        return Err(io_error_to_filesystem_error(path.to_string(), e));
    }

    // read `index.file_size` bytes
    let mut buffer = vec![0u8; index.file_size];
    let bytes_read = match package.read(&mut buffer) {
        Ok(read) => read,
        Err(e) => return Err(io_error_to_filesystem_error(path.to_string(), e)),
    };

    // this can happen if the file doesn't have that many bytes.
    if bytes_read < index.file_size {
        return Err(FilesystemError::UnexpectedEof(path.to_string()));
    }

    match String::from_utf8(buffer) {
        Ok(string) => Ok(string),
        Err(e) => Err(FilesystemError::Generic(path.to_string(), e.to_string())),
    }
}
