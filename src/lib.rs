pub mod cli;
pub mod fs;
pub mod protocol;

pub use crate::cli::{CLI};
pub use crate::fs::{read_file, FileData, FileReaderErr}; 
pub use protocol::*;
