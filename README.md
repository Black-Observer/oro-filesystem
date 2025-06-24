# <img src="assets/orofilesystem.svg" alt="ORO Filesystem" />

**Obstruction Read-Only Filesystem**.   
A library for accessing packed or unpacked files from the filesystem, web resources
or *Obstruction Asset Package*s with a common API.

## Usage

To use ORO Filesystem, you need a configuration, which is the object that contains
information like where the virtual filesystem is located and what type of filesystem
it is. This is autodetected, so you only have to provide a "root" directory to
the virtual filesystem:

```rust
// use oro_filesystem::FilesystemConfig;

// Create a configuration with the present working directory as the root
let config = FilesystemConfig::new().unwrap();

// Create a configuration with a specific path as root
let with_root = FilesystemConfig::with_root("path/to/directory").unwrap();
```

The creation of a configuration can fail if there's any error while reading
that directory (such as the directory not existing or not having permission
to read it) or if there's an error while reading the indices of an indexed
filesystem.

The configuration will autodetect the type of filesystem present in the
specified directory based on these rules:

- If a `.oroi` file is present directly in that directory, it is an
    **Indexed** filesystem.
- If the file is not present, it is a normal filesystem.

The `.oroi` files are simply JSON files containing file indices
for an indexed filesystem. Indexed filesystems map virtual files to an address
in an Asset Package or to a URL.

Assuming that we have a configuration for a filesystem already configured,
indexed or not, we can read any file like this:

```rust
// use oro_filesystem::read_to_string;

let contents = read_to_string("path/to/file.txt", &config).unwrap();
```

The errors that we can get from reading like this can vary depending on the
type of filesystem. A web-based filesystem might get a non-200 response from
a server, which can't happen in Asset Packages or the real filesystem, for 
example.  
Reading to string directly like this, while very comfortable, can also cause
errors.

## Obstruction Asset Package

The OAP format is an extremely simple package-based Indexed filesystem.

> **Note**:  
> OAP is **NOT** compression. It's simply a way of packing several files into
one in a way that is quick to read.  
> It's meant to be used with pre-processed assets that can be directly loaded
into GPU memory, making OAP packages larger than unpacked files.

### The Package

The package is the file where every file is actually stored.  
A game (or other program) can have one or more package but only one **index**.

### The Index

The index is where every file in the package is registered. For simplicity, these
files are JSONs that follow this structure (represented in TypeScript):

```ts
[
    {
        name: string,
        index: {
            package: string,
            starting_size: number,
            file_size: number
        }
    }
]
```
- **`name`**: Full path from the virtual root to the file.
- **`index`**: Information indicating where the virtual file actually is. In this
case it contains the package, file size and index for that file.
- **`package`**: Path from this OROI file to the asset package containing the desired file. One OROI file can index several packages (and web resources).
- **`starting_index`**: The index of the first byte of the desired file in the Asset Package.
- **`file_size`**: Total size of the file we want to read.

## Aura

**Aurum Assets**. Aurum is a web server that allows you to install a mod (OAP or
native Filesystem) in your server to allow anyone to try out and use mods without
actually installing them.

When you add the server to the in-game mod menu, the server generates and sends
an OROI file containing Aura information instead of Asset Package information.

The `index` object of web-based resources follows this structure:

```ts
[
    {
        name: string,
        index: {
            url: string,
            hash: string | null
        }
    }
]
```
- **`url`**: The URL of the file (raw file data).
- **`hash`**: An optional field containing the hash of the file (**NOT IMPLEMENTED YET**). It ensures that the files haven't been altered since you added the Aura file. It doesn't indicate that a mod is safe and it may not even be what you want, for example in frequently updated mods or for Aurum modpacks that might even depend on more Aura files (likely killing performance).

## FAQ

### Is this just for games?

Not really. It could be used anywhere else that requires read-only virtual
filesystems. ORO Filesystem is made primarily for **Gamut** (*Game Mutability*,
the Modding Framework that **Obstruction** is based on) so it is made
primarily for game development.