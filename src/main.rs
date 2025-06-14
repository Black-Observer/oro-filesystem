use oro_filesystem::{read_to_string, FilesystemConfig, FilesystemResult};

fn main() -> FilesystemResult<()> {
    let config = FilesystemConfig::new()?;
    println!("Current ORO Config: {:?}", config);

    let contents = read_to_string("README.md", &config)?;
    println!("CONTENTS OF FILE: `{}`", contents);

    Ok(())
}