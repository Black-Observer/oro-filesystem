# <img src="assets/orofilesystem.svg" alt="ORO Filesystem" />

**Obstruction Read-Only Filesystem**.   
A library for accessing packed or unpacked files from the filesystem, ZIP files
or *Obstruction Asset Package*s with a common API.

One of the core components of **Obstruction**.

## Usage

To be decided.

## Obstruction Asset Package

The OAP format is made of two parts: The **package**(s) and the **index**.

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
- **`index`**: OAPI (or [Aura](#aura-todo)) information indicating where the file is located in the package.
- **`package`**: Path from this OAPI file to the asset package containing the desired file. One OAPI file can index several packages
- **`starting_index`**: The index of the first byte of the desired file in the Asset Package.
- **`file_size`**: Total size of the file we want to read.

## Aura

**Aurum Assets**. Aurum is a web server that allows you to install a mod (OAP or
native Filesystem) in your server to allow anyone to try out and use mods without
actually installing them.

When you add the server to the in-game mod menu, the server generates and sends
an Aura file.

Aura files are also based on JSON and follows a similar structure to normal
indices, with the only difference being the `index` object:

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
- **`hash`**: An optional field containing the hash of the file (**ALGORITHM TO BE DECIDED**). It ensures that the files haven't been altered since you added the Aura file. It doesn't indicate that a mod is safe and it may not even be what you want, for example in frequently updated mods or for Aurum modpacks that might even depend on more Aura files (likely killing performance).

## FAQ

### Is this just for games?

Not really. It could be used anywhere else that requires read-only virtual
filesystems. ORO Filesystem is made primarily for **Gamut** (*Game Mutability*,
the Modding Framework that **Obstruction** is based on) so it is made
primarily for game development.