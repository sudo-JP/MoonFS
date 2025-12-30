use std::{path::{Path, PathBuf}, fs, io};
use std::os::unix::fs::PermissionsExt;

pub const DIR_BIT: u16 = 1 << 10;  // 0o040000


// General Directory Entry 
pub enum EntryType {
    File {
        content: Vec<u8>,  // Vec<u8> to read the binary
    }, 
    Directory {
        entries: Vec<DirectoryEntry>, 
    },
}

pub struct DirectoryEntry {
    pub path: PathBuf,
    pub perm_bit: u16,  // 16 bit for permission bit
    pub entry: EntryType,
}

pub fn read_direntry(path: &Path) -> io::Result<DirectoryEntry> {
    // Check if file exists
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    // Metadata
    let metadata = fs::metadata(path)?;

    // Check if it is a directory
    let perm_bit = metadata.permissions().mode() as u16;

    if (perm_bit & DIR_BIT) != 0 {
        return Err(io::Error::new(io::ErrorKind::Other, "Is directory"));
    }

    // Read file content
    let entry = if perm_bit & DIR_BIT == 0 {
        EntryType::File { content: fs::read(path)? }
    } else {
        EntryType::Directory { entries: vec!() }
    }; 
    //let content = fs::read(path)?;

    // Modification timestamp
    //let modified = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());

    // Return FileData
    Ok(DirectoryEntry {
        path: path.to_path_buf(),
        perm_bit: perm_bit,
        entry: entry
    })
}
