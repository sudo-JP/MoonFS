use crate::FileData;

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

#[repr(C, packed)]
pub struct Header {
    pub name: String, 
    pub size: usize, 
}

#[repr(C, packed)]
pub struct FilePayload {
    // TODO: Add to this field
}

#[repr(C, packed)]
pub struct DirectoryPayload {
    // TODO: Add to this field
}

pub struct Payload {
    pub header: Header, 
    pub payload: SerializedPayload<FilePayload, DirectoryPayload>,
}


pub fn serialize(data: &FileData) -> Result<Payload, SerializeErr> {
    // TODO: convert this into u8 form sepcified by header
    todo!()
}
