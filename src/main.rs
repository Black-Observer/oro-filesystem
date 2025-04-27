use obstruction_filesystem::{read_to_string, FilesystemConfig};

fn main() {
    let config = FilesystemConfig::with_root("test");
    let contents = read_to_string("test_file.txt", &config);
    println!("CONTENTS OF FILE: `{}`", contents)
}