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