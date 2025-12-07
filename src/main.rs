use std::env;
use moonfs::cli::parse;
use moonfs::cli::ParseError;

fn main() {
    let args: Vec<String> = env::args().collect();
    match parse::parse(args) {
        Err(ParseError::InvalidArg) => {

        },

        Err(ParseError::Empty) => {
            println!("Usage: push, add")
        }, 

        Err(ParseError::InvalidPassword) => {

        },
        _ => {}
    }
}
