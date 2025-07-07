use std::fs;

use crate::readers::io_error_to_filesystem_error;

use super::FilesystemResult;

pub fn read_to_string(path: &str) -> FilesystemResult<String> {
    let res = fs::read_to_string(path);
    match res {
        Ok(output) => Ok(output),
        Err(error) => return Err(io_error_to_filesystem_error(path.to_string(), error)),
    }
}

pub fn read(path: &str) -> FilesystemResult<Vec<u8>> {
    let res = fs::read(path);
    match res {
        Ok(output) => Ok(output),
        Err(error) => return Err(io_error_to_filesystem_error(path.to_string(), error)),
    }
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