use std::fs;

use crate::FilesystemError;

use super::FilesystemResult;

pub fn read_to_string(path: &str) -> FilesystemResult<String> {
    fs::read_to_string(path).map_err(|e| FilesystemError::from(e).with_path(path.to_string()))
}

pub fn read(path: &str) -> FilesystemResult<Vec<u8>> {
    fs::read(path).map_err(|e| FilesystemError::from(e).with_path(path.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::readers::filesystem::{read, read_to_string};

    #[test]
    fn read_string() {
        let contents = read_to_string("tests/filesystem/testfile.txt").unwrap();
        assert_eq!(contents, "Hello, World!");
    }
    #[test]
    fn read_from_root() {
        let contents = read("tests/filesystem/testfile.txt").unwrap();
        let expected = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21];

        assert_eq!(contents, expected);
    }
}