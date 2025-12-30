use std::{path::{Path, PathBuf}, fs, io};
use std::os::unix::fs::PermissionsExt;

const DIR_BIT: u16 = 1 << 10;  // 0o040000

pub struct FileData {
    pub path: PathBuf,
    pub perm_bit: u16,  // 16 bit for permission bit
    pub content: Vec<u8>,  // Vec<u8> to read the binary
    pub modified: std::time::SystemTime,  // Time last modified
    pub size: u64  // File size
}

/*
 * Assume path is valid, read content from path
 * */
pub fn read_file(path: &Path) -> io::Result<FileData> {
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
    let content = fs::read(path)?;

    // Get size
    let size = metadata.len();

    // Modification timestamp
    let modified = metadata.modified().unwrap_or_else(|_| std::time::SystemTime::now());

    // Return FileData
    Ok(FileData {
        path: path.to_path_buf(),
        perm_bit,
        content,
        modified,
        size
    })
}