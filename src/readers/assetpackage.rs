// use std::fs;
// 
// use super::{FilesystemError, FilesystemResult};
// 
// pub fn read_to_string(path: &str) -> FilesystemResult<String> {
//     let res = fs::read_to_string(path);
//     match res {
//         Ok(output) => Ok(output),
//         Err(error) => {
//             match error.kind() {
//                 std::io::ErrorKind::NotFound => Err(FilesystemError::NotFound(path.to_string())),
//                 std::io::ErrorKind::PermissionDenied => Err(FilesystemError::PermissionDenied(path.to_string())),
//                 std::io::ErrorKind::BrokenPipe => Err(FilesystemError::BrokenPipe(path.to_string())),
//                 std::io::ErrorKind::NotADirectory => Err(FilesystemError::NotADirectory(path.to_string())),
//                 std::io::ErrorKind::IsADirectory => Err(FilesystemError::IsADirectory(path.to_string())),
//                 std::io::ErrorKind::UnexpectedEof => Err(FilesystemError::UnexpectedEof(path.to_string())),
//                 std::io::ErrorKind::OutOfMemory => Err(FilesystemError::OutOfMemory(path.to_string())),
//                 _ => Err(FilesystemError::Generic(path.to_string(), error.to_string())),
//             }
//         },
//     }
// }