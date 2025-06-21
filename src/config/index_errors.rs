use std::fmt::Display;

#[derive(Debug)]
pub enum IndexError {
    DuplicatePath(String),
}

impl Display for IndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexError::DuplicatePath(path) => write!(f, "Duplicate path found in index: {path}"),
        }
    }
}