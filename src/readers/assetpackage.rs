use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use crate::{
    config::index::AssetPackIndex, FilesystemError,
};

use super::FilesystemResult;

pub fn read(path: &str, root: &str, index: &AssetPackIndex) -> FilesystemResult<Vec<u8>> {
    let package_path = String::from(root) + &index.package;

    let mut package = File::open(&package_path).map_err(|e| FilesystemError::from(e).with_path(path.to_string()))?;

    // apply file offset (which could fail and return an std::io::error)
    if let Err(e) = package.seek(SeekFrom::Start(index.starting_index)) {
        return Err(FilesystemError::from(e).with_path(path.to_string()));
    }

    // read `index.file_size` bytes
    let mut buffer = vec![0u8; index.file_size as usize];
    let bytes_read = package.read(&mut buffer).map_err(|e| FilesystemError::from(e).with_path(path.to_string()))? as u64;

    // this can happen if the file doesn't have that many bytes.
    if bytes_read < index.file_size {
        return Err(FilesystemError::UnexpectedEof(path.to_string()));
    }

    Ok(buffer)
}

pub fn read_to_string(path: &str, root: &str, index: &AssetPackIndex) -> FilesystemResult<String> {
    let buffer = read(path, root, index)?;
    String::from_utf8(buffer).map_err(|e| FilesystemError::Generic(path.to_string(), e.to_string()))
}
