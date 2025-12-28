use std::{path::{Path, PathBuf}, fs};
use std::os::unix::fs::PermissionsExt;

const DIR_BIT: u16 = 1 << 10;  // 0o040000

pub struct FileData {
    pub path: PathBuf,
    pub perm_bit: u16,  // 16 bit for permission bit
    pub content: Vec<u8>,  // Vec<u8> to read the binary
    pub modified: std::time::SystemTime,  // Time last modified
    pub size: u64  // File size
}

pub enum FileReaderErr {
    FileNotFound, 
    IsDirectory,
}

/*
 * Assume path is valid, read content from path
 * */
pub fn read_file(path: &Path) -> Result<FileData, FileReaderErr> {
    // Check if file exists
    if !path.exists(){
        return Err(FileReaderErr::FileNotFound);
    }

    // Read metadata
    let metadata = match fs::metadata(path){
        Ok(file_metadata) => file_metadata,
        Err(_) => return Err(FileReaderErr::FileNotFound),
    };

    // Check if it is a directory
    let perm_bit = metadata.permissions().mode() as u16;
    if is_directory_from_perm(perm_bit){
        return Err(FileReaderErr::IsDirectory);
    }

    // Read file content
    let content = match fs::read(path){
        Ok(data) => data,
        Err(_) => return Err(FileReaderErr::FileNotFound),
    };

    // Get relative path
    let relative_path = ? // TODO: FIX

    // Get size
    let size = metadata.len();

    // Modification timestamp
    let modified = match metadata.modified(){
        Ok(time_modified) => time_modified,
        Err(_) => std::time::SystemTime::now(),
    };

    // Return FileData
    Ok(FileData {
        path: relative_path.to_path_buf(),
        perm_bit,
        content,
        modified,
        size
    })
}

// Helper function to check if directory from permission bit
fn is_directory_from_perm(perm_bit: u16) -> bool {
    (perm_bit & DIR_BIT) != 0
}