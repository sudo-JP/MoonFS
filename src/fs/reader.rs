use std::{path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileData {
    pub path: PathBuf,
    pub perm_bit: u16,      // 16 bit for permission bit
    pub content: Vec<u8>,  // Vec<u8> to read the binary
    
    // TODO: Add more things like time created, size
}

pub enum FileReaderErr {
    FileNotFound, 
}

/*
 * Assume path is valid, read content from path
 * */
pub fn read_file(path: &Path) -> Result<FileData, FileReaderErr> {
    // TODO: Read file content from path, along with its metadata
    todo!()
}
