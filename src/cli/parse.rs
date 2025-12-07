use crate::cli::add::{AddParseError, *};

pub enum ParseError {
    InvalidArg, 
    InvalidPassword, 
    Empty
}

pub fn parse(args: Vec<String>) -> Result<(), ParseError> {
    // Check if password is valid 
    // If not, check if string holds password 

    if args.len() <= 1 {
        return Err(ParseError::Empty);
    }

    match args[1].as_str() {
        "add" => {
            match add_parse(args) {
                Err(AddParseError::NoDest) => println!("Nothing Specified"),
                _ => {}
            }
        }
        _ => return Err(ParseError::InvalidArg),
    }

    Ok(())      
}
