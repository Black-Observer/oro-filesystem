use std::collections::HashMap;

use crate::{config::index::{IndexFile, IndexType}, FilesystemError};

/// A [`HashMap`] containing all the paths and Aura/OAP data for every file in
/// the Virtual Filesystem
pub type AssetMap = HashMap<String, IndexType>;

impl TryFrom<IndexFile> for AssetMap {
    type Error = FilesystemError;
    
    /// Transforms an [`IndexFile`] into an [`AssetMap`].  
    /// This operation can fail if the [`IndexFile`] contains duplicate file entries. In this case,
    /// [`FilesystemError::DuplicatePathsInIndex`] will be returned
    /// 
    /// This function does not check for negative numbers, because negative indices or file
    /// sizes would already give an error while parsing the [`IndexFile`] from JSON (because
    /// of `u64`s being unsigned).
    fn try_from(value: IndexFile) -> Result<Self, Self::Error> {
        let mut map = AssetMap::with_capacity(value.files.len());

        for file in value.files {
            // If it's not some, we already had this path registered
            if map.insert(file.path.clone(), file.index).is_some() {
                return Err(FilesystemError::DuplicatePathsInIndex(file.path));
            }
        }

        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::{config::{assetmap::AssetMap, index::{AssetPackIndex, IndexEntry, IndexFile, IndexType}}, FilesystemResult};

    /// Serializes an OAPI OROI file and checks its contents once converted to an [`AssetMap`] via [`TryFrom`]
    #[test]
    fn from_index_file() {
        let index_file = IndexFile::from_file(&PathBuf::from_str("tests/assetpackage/indices.oroi").unwrap())
            .expect("Couldn't read test file (tests/assetpackage/indices.oroi) or couldn't serialize");

        let asset_map: AssetMap = index_file.try_into().unwrap();
        assert!(asset_map.contains_key("virtualFolder/vfile1.txt"));
        let index_data = asset_map.get("virtualFolder/vfile1.txt").unwrap();
        if let IndexType::AssetPack(contents) = index_data {
            assert_eq!(contents.package, String::from("package.oap"));
            assert_eq!(contents.starting_index, 0);
            assert_eq!(contents.file_size, 28);
        } else {
            panic!("Expected oroi file to contain OAP data");
        }
    }

    /// Converts an [`AssetMap`] into an [`IndexFile`], serializes it and checks its contents
    #[test]
    fn into_index_file() {
        let mut asset_map = AssetMap::new();
        asset_map.insert("virtualFolder/vfile1.txt".to_string(), IndexType::AssetPack(
            AssetPackIndex {
                package: "folder/example.oap".to_string(),
                starting_index: 0,
                file_size: 10
            }
        ));
        asset_map.insert("virtualFolder/vfile1-copy.txt".to_string(), IndexType::AssetPack(
            AssetPackIndex {
                package: "folder/example.oap".to_string(),
                starting_index: 11,
                file_size: 10
            }
        ));

        let index_file: IndexFile = asset_map.into();

        // Two asserts because we don't know in which order the entries will appear
        // because of the HashMap
        assert!(
            index_file.files.contains(&IndexEntry {
                path: "virtualFolder/vfile1.txt".to_string(),
                index: IndexType::AssetPack(
                    AssetPackIndex {
                        package: "folder/example.oap".to_string(),
                        starting_index: 0,
                        file_size: 10
                    }
                )
            })
        );
        assert!(
            index_file.files.contains(&IndexEntry {
                path: "virtualFolder/vfile1-copy.txt".to_string(),
                index: IndexType::AssetPack(
                    AssetPackIndex {
                        package: "folder/example.oap".to_string(),
                        starting_index: 11,
                        file_size: 10
                    }
                )
            })
        );
    }

    #[test]
    fn from_index_file_duplicate_filenames() {
        let index_file = IndexFile::from_file(&PathBuf::from_str("tests/errors/duplicate_paths.oroi").unwrap())
            .expect("Couldn't read test file (tests/errors/duplicate_paths.oroi) or couldn't serialize");

        let asset_map: FilesystemResult<AssetMap> = index_file.try_into();
        asset_map.unwrap_err();
    }

    #[test]
    fn from_index_file_negative_filesize() {
        IndexFile::from_file(&PathBuf::from_str("tests/errors/negative_filesize.oroi").unwrap()).unwrap_err();
    }
    #[test]
    fn from_index_file_negative_index() {
        IndexFile::from_file(&PathBuf::from_str("tests/errors/negative_index.oroi").unwrap()).unwrap_err();
    }
}