use std::collections::HashMap;

use crate::config::index::{IndexEntry, IndexFile, IndexType};

/// A [`HashMap`] containing all the paths and Aura/OAP data for every file in
/// the Virtual Filesystem
pub type AssetMap = HashMap<String, IndexType>;

impl Into<IndexFile> for AssetMap {
    /// **NOT RECOMMENDED**.  
    /// Transforms an [`AssetMap`] into a serializable [`IndexFile`] without checking for errors in the data.
    /// Validations include checking for negative numbers, checking for duplicate files and checking for overlapping
    /// vfile data in OAPs.
    /// Using unchecked files is inherently unsafe but it can potentially allow you to save space in OAPs, as well
    /// as letting you create vfiles that represent slices of files. It's faster but risks crashes and can make
    /// modding unstable.
    fn into(self) -> IndexFile {
        IndexFile {
            files: self
                .iter()
                .map(|(key, value)| {
                    IndexEntry::new(key.to_owned(), value.to_owned())
                })
                .collect()
        }
    }
}

impl From<IndexFile> for AssetMap {
    /// **NOT RECOMMENDED**.  
    /// Creates an [`AssetMap`] from an [`IndexFile`] without checking the validity of the content.  
    /// This can lead to unstable modding and crashes.
    fn from(value: IndexFile) -> Self {
        value.files
            .iter()
            .map(|element|
                (element.path(), element.index())
            )
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::config::{assetmap::AssetMap, index::{AssetPackIndex, IndexFile, IndexType}};

    const EXPECTED_OAP: &str = r#"[{"path":"virtualFolder/vfile1.txt","index":{"package":"folder/example.oap","starting_index":0,"file_size":10}},{"path":"virtualFolder/vfile1-copy.txt","index":{"package":"folder/example.oap","starting_index":11,"file_size":10}}]"#;

    /// Serializes an OAPI file and checks its contents once converted to an [`AssetMap`] via [`Into`]
    #[test]
    fn unchecked_from_index_file() {
        let index_file = IndexFile::from_file(&PathBuf::from_str("tests/assetpackage/indices.oapi").unwrap())
            .expect("Couldn't read test file (tests/assetpackage/indices.oapi) or couldn't serialize");

        let asset_map: AssetMap = index_file.into();
        assert!(asset_map.contains_key("virtualFolder/vfile1.txt"));
        let index_data = asset_map.get("virtualFolder/vfile1.txt").unwrap();
        if let IndexType::AssetPack(contents) = index_data {
            assert_eq!(contents.package, String::from("folder/example.oap"));
            assert_eq!(contents.starting_index, 0);
            assert_eq!(contents.file_size, 10);
        } else {
            panic!("Expected oapi file to contain OAP data");
        }
    }

    /// Converts an [`AssetMap`] into an [`IndexFile`], serializes it and checks its contents against a constant string ([`EXPECTED_OAP`])
    #[test]
    fn unchecked_into_index_file() {
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

        let index_map: IndexFile = asset_map.into();
        let serialized = serde_json::to_string(&index_map.files).expect("Error serializing index map obtained from asset map");

        assert_eq!(serialized, EXPECTED_OAP, "Mismatch between expected OAP file and Serialized OAP JSON obtained from unchecked AssetMap conversion to IndexFile");
    }
}