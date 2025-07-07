use reqwest::blocking::Response;

use crate::FilesystemError;
use super::FilesystemResult;

fn fetch(url: &str) -> FilesystemResult<Response> {
    match reqwest::blocking::get(url) {
        Ok(res) => Ok(res),
        Err(e) => return Err(FilesystemError::FetchError(url.to_string(), e.to_string())),
    }
}

pub fn read_to_string(url: &str) -> FilesystemResult<String> {
    let response = fetch(url)?;

    match response.text() {
        Ok(text) => Ok(text),
        Err(e) => Err(FilesystemError::FetchError(url.to_string(), e.to_string())),
    }
}

pub fn read(url: &str) -> FilesystemResult<Vec<u8>> {
    let response = fetch(url)?;

    match response.bytes() {
        Ok(binary) => Ok(binary.into()),
        Err(e) => Err(FilesystemError::FetchError(url.to_string(), e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use crate::readers::aura::{read, read_to_string};

    #[test]
    fn fetch_from_pastebin() {
        let result = read_to_string("https://pastebin.com/raw/t0qjYDWt").unwrap();
        assert_eq!(result, "Hello, if you fetched this file from an Aura file, that means that ORO Filesystem is working!!");

        let result_bin = read("https://pastebin.com/raw/eQe9aqfZ").unwrap();
        let expected = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21];
        assert_eq!(result_bin, expected);
    }
}