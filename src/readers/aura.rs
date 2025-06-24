use crate::FilesystemError;
use super::FilesystemResult;

pub fn read_to_string(url: &str) -> FilesystemResult<String> {
    let response = match reqwest::blocking::get(url) {
        Ok(res) => res,
        Err(e) => return Err(FilesystemError::FetchError(url.to_string(), e.to_string())),
    };

    match response.text() {
        Ok(text) => Ok(text),
        Err(e) => Err(FilesystemError::FetchError(url.to_string(), e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use crate::readers::aura::read_to_string;

    #[test]
    fn fetch_from_pastebin() {
        let result = read_to_string("https://pastebin.com/raw/t0qjYDWt").unwrap();
        assert_eq!(result, "Hello, if you fetched this file from an Aura file, that means that ORO Filesystem is working!!");
    }
}