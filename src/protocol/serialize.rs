use crate::DirectoryEntry;
use serde::{Serialize, Deserialize};
use std::mem;

pub enum SerializeErr {
    Corrupted, 
    // TODO: add more error if needed
}

// Generic F(ile), D(irectory)
pub enum SerializedPayload<F, D> {
    File(F), 
    Directory(D), 
}

/*
 * Packed so compiler doesn't add paddings
 * */

#[derive(Serialize, Deserialize, Debug)]
#[repr(C, packed)]
pub struct Header<'a> {
    pub name: &'a str, 
    pub size: u64, 
    pub permissions: u16,
}

#[repr(C, packed)]
pub struct FilePayload {
    // TODO: Add to this field
    pub content: Vec<u8>,
}

#[repr(C, packed)]
pub struct DirectoryPayload {
    // TODO: Add to this field
    
}



pub fn serialize(data: &FileData) -> Result<Vec<u8>, SerializeErr> {
    // TODO: convert this into u8 form sepcified by header
    todo!();
    /*let header = Header {
        name: data.path.to_string_lossy().to_string(),
        size: mem::size_of::<data>(),
        permissions: data.perm_bit,
    };
    let mut buf Vec<u8> = [0u8; size_of::<Header>() + size_of::<data>()];*/

    let encoded: Vec<u8> = bincode::serialize(data).expect("Failed to serialize");

    
}
