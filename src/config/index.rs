use serde::{Deserialize, Serialize};

/// the type of index data. It can be AssetPack or Aura
/// and each value of the enum contains the data for that index
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum IndexType {
    AssetPack(AssetPackIndex),
    Aura(AuraIndex)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AssetPackIndex {
    package: String,
    starting_index: u64,
    file_size: u64 // could be u32 but I prefer to use 4 more bytes and support 4GB+ files even if I'll never use them
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AuraIndex {
    url: String,
    hash: Option<String>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IndexEntry {
    path: String,
    index: IndexType
}

pub type IndexFile = Vec<IndexEntry>;

#[cfg(test)]
mod tests {
    use crate::config::index::{AssetPackIndex, AuraIndex, IndexEntry, IndexFile, IndexType};
    
    const EXPECTED_AURA: &str = r#"[{"path":"virtualFolder/vfile1.txt","index":{"url":"https://pastebin.com/raw/t0qjYDWt","hash":null}},{"path":"virtualFolder/vfile1-copy.txt","index":{"url":"https://pastebin.com/raw/t0qjYDWt","hash":null}}]"#;
    const EXPECTED_OAP: &str = r#"[{"path":"virtualFolder/vfile1.txt","index":{"package":"folder/example.oap","starting_index":0,"file_size":10}},{"path":"virtualFolder/vfile1-copy.txt","index":{"package":"folder/example.oap","starting_index":11,"file_size":10}}]"#;
    const EXPECTED_MIXED: &str = r#"[{"path":"virtualFolder/vfile-local.txt","index":{"package":"folder/example.oap","starting_index":0,"file_size":10}},{"path":"virtualFolder/vfile-networked.txt","index":{"url":"https://pastebin.com/raw/t0qjYDWt","hash":null}}]"#;

    #[test]
    fn serialize_aura() {
        let index: IndexFile = vec![
            IndexEntry {
                path: String::from("virtualFolder/vfile1.txt"),
                index: IndexType::Aura(AuraIndex {
                    url: String::from("https://pastebin.com/raw/t0qjYDWt"),
                    hash: None
                })
            },
            IndexEntry {
                path: String::from("virtualFolder/vfile1-copy.txt"),
                index: IndexType::Aura(AuraIndex {
                    url: String::from("https://pastebin.com/raw/t0qjYDWt"),
                    hash: None
                })
            }
        ];

        let serialized = serde_json::to_string(&index).expect("Error during serialization of Aura file");
        assert_eq!(serialized, EXPECTED_AURA);
    }

    #[test]
    fn deserialize_serialized_aura() {
        let original: IndexFile = vec![
            IndexEntry {
                path: String::from("virtualFolder/vfile1.txt"),
                index: IndexType::Aura(AuraIndex {
                    url: String::from("https://pastebin.com/raw/t0qjYDWt"),
                    hash: None
                })
            },
            IndexEntry {
                path: String::from("virtualFolder/vfile1-copy.txt"),
                index: IndexType::Aura(AuraIndex {
                    url: String::from("https://pastebin.com/raw/t0qjYDWt"),
                    hash: None
                })
            }
        ];

        let serialized = serde_json::to_string(&original).expect("Error during serialization of Aura file");
        let deserialized: IndexFile = serde_json::from_str(&serialized).expect("Error during deserialization of Aura file");

        assert_eq!(deserialized.len(), original.len(), "Length of the deserialized file must be equal to that of the original ({}). Found {}", original.len(), deserialized.len());
        for (index, index_entry) in original.iter().enumerate() {
            assert_eq!(deserialized.get(index).unwrap(), index_entry, "Deserialized file entries must be the same as in the original");
        }
    }

    #[test]
    fn serialize_oap() {
        let index: IndexFile = vec![
            IndexEntry {
                path: String::from("virtualFolder/vfile1.txt"),
                index: IndexType::AssetPack(AssetPackIndex {
                    package: String::from("folder/example.oap"),
                    starting_index: 0,
                    file_size: 10
                })
            },
            IndexEntry {
                path: String::from("virtualFolder/vfile1-copy.txt"),
                index: IndexType::AssetPack(AssetPackIndex {
                    package: String::from("folder/example.oap"),
                    starting_index: 11,
                    file_size: 10
                })
            }
        ];

        let serialized = serde_json::to_string(&index).expect("Error during serialization of OAP file");
        assert_eq!(serialized, EXPECTED_OAP);
    }

    #[test]
    fn deserialize_serialized_oap() {
        let original: IndexFile = vec![
            IndexEntry {
                path: String::from("virtualFolder/vfile1.txt"),
                index: IndexType::AssetPack(AssetPackIndex {
                    package: String::from("folder/example.oap"),
                    starting_index: 0,
                    file_size: 10
                })
            },
            IndexEntry {
                path: String::from("virtualFolder/vfile1-copy.txt"),
                index: IndexType::AssetPack(AssetPackIndex {
                    package: String::from("folder/example.oap"),
                    starting_index: 11,
                    file_size: 10
                })
            }
        ];

        let serialized = serde_json::to_string(&original).expect("Error during serialization of OAP file");
        let deserialized: IndexFile = serde_json::from_str(&serialized).expect("Error during deserialization of OAP file");

        assert_eq!(deserialized.len(), original.len(), "Length of the deserialized file must be equal to that of the original ({}). Found {}", original.len(), deserialized.len());
        for (index, index_entry) in original.iter().enumerate() {
            assert_eq!(deserialized.get(index).unwrap(), index_entry, "Deserialized file entries must be the same as in the original");
        }
    }

    #[test]
    fn serialize_mixed() {
        let index: IndexFile = vec![
            IndexEntry {
                path: String::from("virtualFolder/vfile-local.txt"),
                index: IndexType::AssetPack(AssetPackIndex {
                    package: String::from("folder/example.oap"),
                    starting_index: 0,
                    file_size: 10
                })
            },
            IndexEntry {
                path: String::from("virtualFolder/vfile-networked.txt"),
                index: IndexType::Aura(AuraIndex {
                    url: String::from("https://pastebin.com/raw/t0qjYDWt"),
                    hash: None
                })
            }
        ];

        let serialized = serde_json::to_string(&index).expect("Error during serialization of Mixed file");
        assert_eq!(serialized, EXPECTED_MIXED);
    }

    #[test]
    fn deserialize_serialized_mixed() {
        let original: IndexFile = vec![
            IndexEntry {
                path: String::from("virtualFolder/vfile-local.txt"),
                index: IndexType::AssetPack(AssetPackIndex {
                    package: String::from("folder/example.oap"),
                    starting_index: 0,
                    file_size: 10
                })
            },
            IndexEntry {
                path: String::from("virtualFolder/vfile-networked.txt"),
                index: IndexType::Aura(AuraIndex {
                    url: String::from("https://pastebin.com/raw/t0qjYDWt"),
                    hash: None
                })
            }
        ];

        let serialized = serde_json::to_string(&original).expect("Error during serialization of Mixed file");
        let deserialized: IndexFile = serde_json::from_str(&serialized).expect("Error during deserialization of Mixed file");

        assert_eq!(deserialized.len(), original.len(), "Length of the deserialized file must be equal to that of the original ({}). Found {}", original.len(), deserialized.len());
        for (index, index_entry) in original.iter().enumerate() {
            assert_eq!(deserialized.get(index).unwrap(), index_entry, "Deserialized file entries must be the same as in the original");
        }
    }
}