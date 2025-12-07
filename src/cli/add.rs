pub enum AddParseError {
   NoDest, 
}

pub fn add_parse(args: Vec<String>) -> Result<(), AddParseError> {
    if args.len() <= 2 {
        return Err(AddParseError::NoDest)
    }
    Ok(())
}
